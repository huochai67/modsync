// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::{path::Path, sync::Arc};

use log::{debug, error, info, warn};
use modsync_core::{
    msclient::{DiffType, MODDiff, MSClient, MSClientBuilder},
    msconfig::{MSConfig, ReleaseInfo},
    msmod::MSMOD,
    mstaskmanager::{TaskManager, TaskRequest, TaskStatus},
};
use serde::Serialize;
use tauri::{Manager, State};
use tokio::sync::Mutex;

fn getdotminecraft() -> String {
    let pwd = format!(
        "{}/.minecraft",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let _ = std::fs::create_dir_all(pwd.clone());
    return pwd;
}

fn get_configpack_path() -> String {
    format!("{}/configpack.zip", getdotminecraft())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    MSCore(#[from] modsync_core::error::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ZIPError(#[from] zip::result::ZipError),

    #[error("Already running")]
    AlreadyRunning,

    #[error("Not running")]
    NotRunning,

    #[error("Runtime not initialized")]
    NotInitialized,

    #[error("Unknown error")]
    Err,
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Clone, Debug, Serialize)]
struct RuntimeInfo {
    title: String,
    version: String,
    buildinfo: String,
    has_serverdat: bool,
    has_options: bool,
    has_hcml: bool,
    has_pclce: bool,
    has_configpack: bool,
    release_info: Vec<ReleaseInfo>,
}
struct AppRuntimeInner {
    client: Option<MSClient>,
    runtime_info: Option<RuntimeInfo>,

    is_running: Arc<Mutex<bool>>,
    running_tasks: Arc<Mutex<Vec<TaskStatus>>>,
}
impl AppRuntimeInner {
    async fn try_get_client(&mut self) -> Result<MSClient, Error> {
        match &self.client {
            None => {
                info!("Client not initialized, creating new client");
                let client = MSClientBuilder::new()
                    .msconfig(
                        // MSConfig::get_remote_config("http://127.0.0.1:8086/info.json").await?,
                        MSConfig::get_remote_config("https://cn.ms.nicefish4520.com/info.json")
                            .await?,
                    )
                    .path(getdotminecraft())
                    .build()?;
                info!("Client created successfully");
                self.client = Some(client.clone());
                Ok(client)
            }
            Some(client) => Ok(client.clone()),
        }
    }
}
type AppRuntime = Mutex<AppRuntimeInner>;

#[tauri::command]
async fn download_utility(state: State<'_, AppRuntime>, utility: &str) -> Result<(), Error> {
    info!("Downloading utility: {}", utility);
    let mut state = state.lock().await;
    let client = state.try_get_client().await?;
    match utility {
        "hmcl" => client.sync_hcml().await?,
        "pclce" => client.sync_pclce().await?,
        "options" => client.sync_options().await?,
        "serverdat" => client.sync_serverdat().await?,
        _ => {
            error!("Unknown utility: {}", utility);
            return Err(Error::Err);
        }
    }
    info!("Utility {} downloaded successfully", utility);
    Ok(())
}

#[tauri::command]
async fn get_diff(state: State<'_, AppRuntime>) -> Result<Vec<MODDiff>, Error> {
    debug!("Getting diff list...");
    let mut state = state.lock().await;
    let client = state.try_get_client().await?;
    let diffs = client.get_difflist().await?;
    debug!("Found {} diffs", diffs.len());
    Ok(diffs)
}

#[tauri::command]
async fn apply_diff(
    state: State<'_, AppRuntime>,
    diffs: Vec<MODDiff>,
    backup: bool,
    sync_config_pack: bool,
) -> Result<(), Error> {
    info!(
        "Applying {} diffs (backup: {}, sync_config_pack: {})",
        diffs.len(),
        backup,
        sync_config_pack
    );
    let mut tasks: Vec<TaskRequest> = vec![];

    if sync_config_pack {
        let configpack_opt = {
            let mut state = state.lock().await;
            let client = state.try_get_client().await?;
            client.get_configpack()
        };
        let configpack = match configpack_opt {
            Some(url) => url,
            None => {
                error!("No config pack found in MSConfig");
                return Err(Error::MSCore(
                    modsync_core::error::Error::MSConfigNoConfigPack,
                ));
            }
        };

        let configpack_str = get_configpack_path();
        let configpack_path = Path::new(&configpack_str);
        let mut downloadconfigpack = true;

        debug!("检查本地ConfigPack at {:?}", configpack_path);
        // Check local configpack
        if configpack_path.exists() {
            let local = MSMOD::from_file(configpack_path, "", None);
            debug!("本地 MD5: {:?}, 远程 MD5: {:?}", local.md5, configpack.md5);
            if local.md5 == configpack.md5 {
                downloadconfigpack = false
            }
        }

        info!("需要下载ConfigPack: {}", downloadconfigpack);
        if downloadconfigpack {
            tasks.push(TaskRequest::download(
                "Download ConfigPack".to_string(),
                configpack.url.unwrap(),
                configpack_str,
            ));
        }
    }

    // Check backup dirctory
    if backup {
        let strpath = format!("{}/bakmods", getdotminecraft());
        let backupdir = Path::new(&strpath);
        if !backupdir.exists() {
            info!("Creating backup directory at {:?}", backupdir);
            tokio::fs::create_dir_all(backupdir).await?;
        }
    }

    for diff in diffs.iter() {
        match diff.difftype {
            DiffType::NEWED | DiffType::MODIFIED => {
                if let Some(remote) = &diff.remote {
                    debug!("Adding download task for: {}", remote.path);
                    tasks.push(TaskRequest::download(
                        format!("下载{}", remote.path.clone()),
                        remote.url.clone().unwrap(),
                        format!("{}/mods/{}", getdotminecraft(), remote.path),
                    ));
                }
            }
            DiffType::DELETED => {
                if let Some(local) = &diff.local {
                    if backup {
                        debug!("Adding backup/delete task for: {}", local.path);
                        tasks.push(TaskRequest::rename(
                            format!("删除{}", local.path),
                            format!("{}/mods/{}", getdotminecraft(), local.path),
                            format!("{}/bakmods/{}", getdotminecraft(), local.path),
                        ));
                    } else {
                        debug!("Adding delete task for: {}", local.path);
                        tasks.push(TaskRequest::delete(
                            format!("删除{}", local.path),
                            format!("{}/mods/{}", getdotminecraft(), local.path),
                        ));
                    }
                }
            }
        }
    }

    info!("Submitting {} tasks", tasks.len());
    summit_task(state.clone(), tasks).await?;

    // unzip configpack
    if sync_config_pack {
        info!("Unzipping config pack");
        let unziptask = TaskRequest::unzip(
            "Process ConfigPack".to_string(),
            get_configpack_path(),
            getdotminecraft(),
        );

        summit_task(state.clone(), vec![unziptask]).await?;
    }

    info!("Diff application finished");
    Ok(())
}

#[tauri::command]
async fn is_running(state: State<'_, AppRuntime>) -> Result<bool, Error> {
    let state = state.lock().await;
    let is_running = state.is_running.lock().await.clone();
    Ok(is_running)
}

#[tauri::command]
async fn summit_task(state: State<'_, AppRuntime>, tasks: Vec<TaskRequest>) -> Result<(), Error> {
    if is_running(state.clone()).await? {
        return Err(Error::AlreadyRunning);
    }

    info!("init taskmanager now");
    // init TaskManager and run tasks
    let mut taskmanager = TaskManager::new(20);
    let running_task = taskmanager.get_vec_task_status().await;
    {
        let mut state = state.lock().await;
        state.running_tasks = running_task;
        *state.is_running.lock().await = true;
    }

    // Post: set is_running to false after tasks complete
    if let Err(e) = taskmanager.run(tasks).await {
        error!("Task execution failed: {:?}", e);
        return Err(Error::Err);
    }

    let state = state.lock().await;
    *state.is_running.lock().await = false;
    Ok(())
}

#[tauri::command]
async fn getall_task(state: State<'_, AppRuntime>) -> Result<Vec<TaskStatus>, Error> {
    if !is_running(state.clone()).await? {
        return Err(Error::NotRunning);
    }

    let running_task: Arc<Mutex<Vec<TaskStatus>>>;
    {
        let state = state.lock().await;
        running_task = state.running_tasks.clone();
    }
    let running_task = running_task.lock().await;
    Ok(running_task.clone())
}

build_info::build_info!(fn build_info);
#[tauri::command]
async fn init_runtime(state: State<'_, AppRuntime>) -> Result<(), Error> {
    info!("Initializing runtime...");

    let mut state = state.lock().await;
    let client = state.try_get_client().await?;

    let config = client.get_config();
    let release_info = config.release_info;

    state.runtime_info = Some(RuntimeInfo {
        title: client.get_title(),
        version: format!("v{}", env!("CARGO_PKG_VERSION")),
        buildinfo: {
            let bi = build_info();
            let commit_id = match &bi.version_control {
                Some(vc) => vc.git().unwrap().commit_short_id.as_str(),
                None => "unknown",
            };
            format!(
                "{} {} {} at {}",
                bi.crate_info.name, bi.target.os, commit_id, bi.timestamp
            )
        },
        release_info,
        has_serverdat: client.get_serverdat().is_some(),
        has_options: client.get_options().is_some(),
        has_hcml: client.get_launcher_hmcl().is_some(),
        has_pclce: client.get_launcher_pclce().is_some(),
        has_configpack: client.get_configpack().is_some(),
    });

    info!(
        "Runtime initialized: {}",
        state.runtime_info.as_ref().unwrap().title
    );

    Ok(())
}

#[tauri::command]
async fn get_runtime(state: State<'_, AppRuntime>) -> Result<RuntimeInfo, Error> {
    let state = state.lock().await;
    match &state.runtime_info {
        Some(runtime_info) => Ok(runtime_info.clone()),
        None => Err(Error::NotInitialized),
    }
}

#[tauri::command]
async fn is_init(state: State<'_, AppRuntime>) -> Result<bool, Error> {
    let state = state.lock().await;
    Ok(state.runtime_info.is_some())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .setup(|app| {
            // let mut downloader = Downloader::new(8);
            //tokio::task::spawn(async move { downloader.run().await });
            app.manage(Mutex::new(AppRuntimeInner {
                client: None,
                runtime_info: None,
                running_tasks: Arc::new(Mutex::new(Vec::new())),
                is_running: Arc::new(Mutex::new(false)),
            }));
            info!("MSSYNC启动完成");
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            init_runtime,
            get_runtime,
            is_init,
            get_diff,
            apply_diff,
            is_running,
            summit_task,
            getall_task,
            download_utility,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
