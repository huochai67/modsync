pub mod task_pool;

use modsync_core::{
    msclient::{MODDiff, MSClient, MSClientBuilder},
    msconfig::MSConfig,
    mstask::MSTaskStatus,
};
use task_pool::Task;
use tauri::{Manager, State};
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

struct MSNextRunTimeInner {
    client: Option<MSClient>,
    changelog: Option<String>,
    taskpool: Task,
}

impl MSNextRunTimeInner {
    async fn try_get_client(&mut self) -> Result<MSClient, Error> {
        match &self.client {
            None => {
                let client = MSClientBuilder::new()
                    .msconfig(MSConfig::get_remote_config().await?)
                    .path(getdotminecraft())
                    .build()?;
                self.client = Some(client.clone());
                Ok(client)
            }
            Some(client) => Ok(client.clone()),
        }
    }
}
type MSNextRunTime = Mutex<MSNextRunTimeInner>;

fn getdotminecraft() -> String {
    let pwd = format!(
        "{}/../.minecraft",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let _ = std::fs::create_dir_all(pwd.clone());
    return pwd;
}

#[tauri::command]
async fn download_serverlist(state: State<'_, MSNextRunTime>) -> Result<(), Error> {
    let mut state = state.lock().await;
    let client = state.try_get_client().await?;
    Ok(client.sync_serverlist().await?)
}

#[tauri::command]
async fn download_options(state: State<'_, MSNextRunTime>) -> Result<(), Error> {
    let mut state = state.lock().await;
    let client = state.try_get_client().await?;
    Ok(client.sync_option().await?)
}

#[tauri::command]
async fn get_changelog(state: State<'_, MSNextRunTime>) -> Result<String, Error> {
    let mut state = state.lock().await;
    match &state.changelog {
        Some(changelog) => Ok(changelog.clone()),
        None => {
            let client = state.try_get_client().await?;
            let changelog = client.get_changelog().await?;
            state.changelog = Some(changelog.clone());
            Ok(changelog)
        }
    }
}

#[tauri::command]
async fn get_title(state: State<'_, MSNextRunTime>) -> Result<String, Error> {
    let mut state = state.lock().await;
    let client = state.try_get_client().await?;
    Ok(client.get_remoteconfig().title)
}

#[tauri::command]
async fn get_diff(state: State<'_, MSNextRunTime>) -> Result<Vec<MODDiff>, Error> {
    let mut state = state.lock().await;
    let client = state.try_get_client().await?;
    Ok(client.get_difflist().await?)
}

#[tauri::command]
async fn apply_diff(state: State<'_, MSNextRunTime>, diffs: Vec<MODDiff>) -> Result<(), Error> {
    let mut state = state.lock().await;
    let client = state.try_get_client().await?;

    let vec_task = client.apply_diff(diffs.as_slice()).await?;
    state.taskpool.num_total = vec_task.len();
    //tokio::task::spawn(state.taskpool.run(Box::new(vec_task)));
    
    tokio::task::spawn(task_pool::ddrun(
        state.taskpool.tx.clone(),
        Box::new(vec_task),
    ));
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct GetTaskPayload {
    tasks: Vec<MSTaskStatus>,
    num_total: usize,
    num_finished: usize,
}
#[tauri::command]
async fn get_tasks(state: State<'_, MSNextRunTime>) -> Result<GetTaskPayload, Error> {
    let mut state = state.lock().await;
    let running_task = state.taskpool.get_status();
    Ok(GetTaskPayload {
        tasks: running_task,
        num_total: state.taskpool.num_total,
        num_finished: state.taskpool.num_finished,
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

//#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(MSNextRunTimeInner {
                client: Default::default(),
                changelog: Default::default(),
                taskpool: Task::new(),
            }));
            Ok(())
        })
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
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
