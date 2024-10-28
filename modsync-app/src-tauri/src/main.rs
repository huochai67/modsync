// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use modsync_app::task_pool::TaskPool;
use modsync_core::{
    msclient::{MODDiff, MSClient, MSClientBuilder},
    msconfig::MSConfig,
};
use tokio::sync::Mutex;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    MSCore(#[from] modsync_core::error::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

struct MSNextRunTime {
    client: Mutex<Option<MSClient>>,
    changelog: Mutex<Option<String>>,
    taskpool: Mutex<TaskPool>,
}

impl MSNextRunTime {
    async fn try_get_client(&self) -> Result<MSClient, Error> {
        let mut selfclient = self.client.lock().await;
        match selfclient.as_ref() {
            None => {
                let client = MSClientBuilder::new()
                    .msconfig(MSConfig::get_remote_config().await?)
                    .path(getdotminecraft())
                    .build()?;
                *selfclient = Some(client.clone());
                Ok(client)
            }
            Some(client) => Ok(client.clone()),
        }
    }
}

fn getdotminecraft() -> String {
    let pwd = format!(
        "{}/../.minecraft",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let _ = std::fs::create_dir_all(pwd.clone());
    return pwd;
}

#[tauri::command]
async fn download_serverlist(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<(), Error> {
    let client = msnruntime.try_get_client().await?;
    Ok(client.sync_serverlist().await?)
}

#[tauri::command]
async fn download_options(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<(), Error> {
    let client = msnruntime.try_get_client().await?;
    Ok(client.sync_option().await?)
}

#[tauri::command]
async fn get_changelog(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<String, Error> {
    let mut selfchangelog = msnruntime.changelog.lock().await;
    match selfchangelog.as_ref() {
        Some(changelog) => Ok(changelog.clone()),
        None => {
            let client = msnruntime.try_get_client().await?;
            let changelog = client.get_changelog().await?;
            *selfchangelog = Some(changelog.clone());
            Ok(changelog)
        }
    }
}

#[tauri::command]
async fn get_title(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<String, Error> {
    let client = msnruntime.try_get_client().await?;
    Ok(client.get_remoteconfig().title)
}

#[tauri::command]
async fn get_diff(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<Vec<MODDiff>, Error> {
    let client = msnruntime.try_get_client().await?;
    Ok(client.get_difflist().await?)
}

#[tauri::command]
async fn apply_diff(
    msnruntime: tauri::State<'_, MSNextRunTime>,
    diffs: Vec<MODDiff>,
) -> Result<(), Error> {
    let client = msnruntime.try_get_client().await?;
    let mut vec_task = client.apply_diff(diffs.as_slice()).await?;

    let mut taskpool = msnruntime.taskpool.lock().await;
    while let Some(task) = vec_task.pop() {
        taskpool.push(task);
    }
    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct TaskInfo {
    totalsize: u64,
    downloadsize: u64,
    name: String,
}

impl TaskInfo {
    fn new(totalsize: u64, downloadsize: u64, name: String) -> TaskInfo {
        TaskInfo {
            totalsize,
            downloadsize,
            name,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct GetTaskPayload {
    tasks: Vec<TaskInfo>,
    num_total: usize,
    num_finished: usize,
}
#[tauri::command]
async fn get_tasks(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<GetTaskPayload, Error> {
    let mut ret = vec![];
    let mut taskpool = msnruntime.taskpool.lock().await;
    taskpool.check().await?;
    let running_task = taskpool.get_status();
    for ptask in running_task.iter() {
        ret.push(TaskInfo::new(ptask.total, ptask.now, ptask.name.clone()))
    }
    Ok(GetTaskPayload {
        tasks: ret,
        num_total: taskpool.num_total,
        num_finished: taskpool.num_finished,
    })
}

build_info::build_info!(fn build_info);

#[tauri::command]
fn get_version() -> String {
    build_info::format!("v{}", $.crate_info.version).into()
}

#[tauri::command]
fn get_buildinfo() -> String {
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
    .into()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(MSNextRunTime {
            client: Default::default(),
            changelog: Default::default(),
            taskpool: Mutex::new(TaskPool::new()),
        })
        .invoke_handler(tauri::generate_handler![
            get_version,
            get_buildinfo,
            get_diff,
            get_tasks,
            apply_diff,
            download_options,
            download_serverlist,
            get_changelog,
            get_title,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
