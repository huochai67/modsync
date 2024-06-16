// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use modsync_core::{
    msclient::{MODDiff, MSClient},
    msconfig::MSConfig,
    mstask::{DownloadTask, MSTask},
};
use tokio::sync::Mutex;

struct MSNextRunTime {
    config: Mutex<Option<MSConfig>>,
    changelog: Mutex<Option<String>>,
    tasks: Mutex<Vec<Arc<DownloadTask>>>,
}

impl MSNextRunTime {
    async fn try_get_config(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut selfconfig = self.config.lock().await;
        match selfconfig.as_ref() {
            None => match MSConfig::get_remote_config().await {
                Ok(msconfig) => {
                    *selfconfig = Some(msconfig);
                    Ok(())
                }
                Err(err) => Err(err),
            },
            Some(_) => Ok(()),
        }
    }

    async fn getconfig(&self) -> tokio::sync::MutexGuard<'_, Option<MSConfig>> {
        self.config.lock().await
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
async fn download_serverlist(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<(), String> {
    if let Err(err) = msnruntime.try_get_config().await {
        return Err(err.to_string());
    }
    match MSClient::config(msnruntime.getconfig().await.as_ref().unwrap())
        .path(getdotminecraft())
        .sync_serverlist()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn download_options(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<(), String> {
    if let Err(err) = msnruntime.try_get_config().await {
        return Err(err.to_string());
    }
    match MSClient::config(msnruntime.getconfig().await.as_ref().unwrap())
        .path(getdotminecraft())
        .sync_option()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn try_init(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<(), String> {
    if let Err(err) = msnruntime.try_get_config().await {
        return Err(err.to_string());
    }
    Ok(())
}

#[tauri::command]
async fn get_changelog(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<String, String> {
    if let Err(err) = msnruntime.try_get_config().await {
        return Err(err.to_string());
    }

    let mut selfchangelog = msnruntime.changelog.lock().await;
    if let Some(changelog) = selfchangelog.as_ref() {
        return Ok(changelog.clone());
    }

    let config = msnruntime.getconfig().await;
    match MSClient::config(config.as_ref().unwrap())
        .get_changelog()
        .await
    {
        Ok(changelog) => {
            let ret = changelog.clone();
            *selfchangelog = Some(changelog);
            Ok(ret)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn get_title(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<String, String> {
    if let Err(err) = msnruntime.try_get_config().await {
        return Err(err.to_string());
    }

    let config = msnruntime.getconfig().await;
    Ok(config.as_ref().unwrap().title.clone())
}

#[tauri::command]
async fn get_diff(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<Vec<MODDiff>, String> {
    if let Err(err) = msnruntime.try_get_config().await {
        return Err(err.to_string());
    }

    let config = msnruntime.getconfig().await;
    let mut client_ = MSClient::config(config.as_ref().unwrap());
    let client = client_.path(getdotminecraft());
    match client.get_modlist().await {
        Ok(modlist_remote) => match client.get_difflist(modlist_remote) {
            Ok(diff) => Ok(diff),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn apply_diff(
    msnruntime: tauri::State<'_, MSNextRunTime>,
    diffs: Vec<MODDiff>,
) -> Result<(), String> {
    if let Err(err) = msnruntime.try_get_config().await {
        return Err(err.to_string());
    }
    let mut msntasks = msnruntime.tasks.lock().await;

    let vec_task = MSClient::config(msnruntime.getconfig().await.as_ref().unwrap())
        .path(getdotminecraft())
        .apply_diff(diffs.as_slice());
    for mut task in vec_task {
        match task.spawn().await {
            Ok(_) => msntasks.push(Arc::from(task)),
            Err(err) => {
                return Err(err.to_string());
            }
        };
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

#[tauri::command]
async fn get_tasks(msnruntime: tauri::State<'_, MSNextRunTime>) -> Result<Vec<TaskInfo>, String> {
    let mut ret = vec![];
    let msntasks = msnruntime.tasks.lock().await;
    for ptask in msntasks.iter() {
        if !ptask.get_join_handle().is_finished() {
            ret.push(TaskInfo::new(
                ptask.get_size_total(),
                ptask.get_size_downloaded().await,
                ptask.get_name().to_string(),
            ));
        }
    }
    Ok(ret)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(MSNextRunTime {
            config: Default::default(),
            changelog: Default::default(),
            tasks: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            get_diff,
            get_tasks,
            apply_diff,
            download_options,
            download_serverlist,
            get_changelog,
            get_title,
            try_init
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
