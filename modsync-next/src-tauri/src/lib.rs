// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Arc;

use modsync_core::{
    msclient::{DiffType, MODDiff, MSClient, MSClientBuilder},
    msconfig::{MSConfig, ReleaseInfo},
    mstaskmanager::{TaskManager, TaskRequest, TaskStatus},
};
use serde::Serialize;
use tauri::{Manager, State};
use tokio::sync::Mutex;

fn getdotminecraft() -> String {
    let pwd = format!(
        "{}/../.minecraft",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let _ = std::fs::create_dir_all(pwd.clone());
    return pwd;
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    MSCore(#[from] modsync_core::error::Error),

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
                let client = MSClientBuilder::new()
                    .msconfig(
                        MSConfig::get_remote_config("http://127.0.0.1:8086/info.json").await?,
                        // .msconfig(
                        //     MSConfig::get_remote_config("https://cn.ms.nicefish4520.com/info.json")
                        //         .await?,
                    )
                    .path(getdotminecraft())
                    .build()?;
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
    let mut state = state.lock().await;
    let client = state.try_get_client().await?;
    match utility {
        "hmcl" => client.sync_hcml().await?,
        "pclce" => client.sync_pclce().await?,
        "options" => client.sync_options().await?,
        "serverlist" => client.sync_serverdat().await?,
        _ => return Err(Error::Err),
    }
    Ok(())
}

#[tauri::command]
async fn get_diff(state: State<'_, AppRuntime>) -> Result<Vec<MODDiff>, Error> {
    let mut state = state.lock().await;
    let client = state.try_get_client().await?;
    Ok(client.get_difflist().await?)
}

#[tauri::command]
async fn apply_diff(state: State<'_, AppRuntime>, diffs: Vec<MODDiff>) -> Result<(), Error> {
    let mut tasks: Vec<TaskRequest> = vec![];
    for diff in diffs.iter() {
        match diff.difftype {
            DiffType::NEWED | DiffType::MODIFIED => {
                if let Some(remote) = &diff.remote {
                    tasks.push(TaskRequest::download(
                        format!("下载{}", remote.path.clone()),
                        remote.url.clone().unwrap(),
                        format!("{}/mods/{}", getdotminecraft(), remote.path),
                    ));
                }
            }
            DiffType::DELETED => {
                if let Some(local) = &diff.local {
                    tasks.push(TaskRequest::delete(
                        format!("删除{}", local.path),
                        format!("{}/mods/{}", getdotminecraft(), local.path),
                    ));
                }
            }
        }
    }

    summit_task(state, tasks).await?;

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

    println!("init taskmanager now");
    // init TaskManager and run tasks
    let mut taskmanager = TaskManager::new(20);
    let running_task = taskmanager.get_vec_task_status().await;
    {
        let mut state = state.lock().await;
        state.running_tasks = running_task;
        *state.is_running.lock().await = true;
    }

    // Post: set is_running to false after tasks complete
    if let Err(_e) = taskmanager.run(tasks).await {
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
    println!("Initializing runtime...");

    let mut state = state.lock().await;
    let client = state.try_get_client().await?;

    let config = client.get_config();
    let release_info = config.release_info;

    state.runtime_info = Some(RuntimeInfo {
        title: client.get_title(),
        version: format!("v{}", env!("CARGO_PKG_VERSION")),
        buildinfo: {
            let bi = build_info();
            format!(
                "{} {} {} at {}",
                bi.crate_info.name,
                bi.target.os,
                match &bi.version_control {
                    Some(vc) => {
                        vc.git().unwrap().commit_short_id.as_str()
                    }
                    None => "unknown",
                },
                bi.timestamp
            )
        },
        release_info,
        has_serverdat: client.get_serverdat().is_some(),
        has_options: client.get_options().is_some(),
        has_hcml: client.get_launcher_hmcl().is_some(),
        has_pclce: client.get_launcher_pclce().is_some(),
    });

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
