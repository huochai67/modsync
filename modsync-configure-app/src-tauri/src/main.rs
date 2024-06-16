// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use modsync_core::msconfig::MSConfig;
use modsync_core::msmod::MSMOD;

fn writetofile(filepath: &str, data: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filepath)?
        .write(data)?)
}

#[tauri::command]
fn generate(
    changelog: &str,
    title: &str,
    serverurl: &str,
    forceserverlist: bool,
) -> Result<(), String> {
    if let Err(err) = writetofile("./data/changelog.txt", changelog.as_bytes()) {
        return Err(err.to_string());
    }

    let mut modlist_url = "null".to_string();
    if Path::new("./data/data/mods").exists() {
        modlist_url = format!("{}modslist.json", serverurl);

        match MSMOD::from_directory("./data/data/mods", Some(format!("{}", serverurl).as_str())) {
            Ok(vecmsmod) => {
                if let Err(err) = writetofile(
                    "./data/modslist.json",
                    serde_json::to_string(&vecmsmod).unwrap().as_bytes(),
                ) {
                    return Err(err.to_string());
                }
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }

    let mut changelog_url = "null".to_string();
    let mut option_url: String = "null".to_string();
    let mut serverlist_url = "null".to_string();
    if Path::new("./data/changelog.txt").exists() {
        changelog_url = format!("{}changelog.txt", serverurl);
    }
    if Path::new("./data/data/options.txt").exists() {
        option_url = format!("{}data/options.txt", serverurl);
    }
    if Path::new("./data/data/servers.dat").exists() {
        serverlist_url = format!("{}data/servers.dat", serverurl);
    }

    let config = MSConfig::new(
        serverurl.to_string(),
        changelog_url,
        modlist_url,
        option_url,
        serverlist_url,
        forceserverlist,
        title.to_string(),
    );
    if let Err(err) = writetofile(
        "./data/info.json",
        serde_json::to_string(&config).unwrap().as_bytes(),
    ) {
        return Err(err.to_string());
    }

    Ok(())
    //format!("[{:?}] {}", std::time::SystemTime::now(), "OK")
}

#[tauri::command]
fn get_changelog() -> String {
    if Path::new("./data/changelog.txt").exists() {
        match File::open("./data/changelog.txt") {
            Ok(mut file) => {
                let mut str: String = "".to_string();
                if let Err(err) = file.read_to_string(&mut str) {
                    return err.to_string();
                }
                return str;
            }
            Err(err) => return err.to_string(),
        }
    }
    "no changelog".to_string()
}

#[tauri::command]
fn get_config() -> Option<MSConfig> {
    if !Path::new("./data/info.json").exists() {
        return None;
    }
    if let Ok(config) = MSConfig::from_file("./data/info.json") {
        return Some(config);
    }
    return None;
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            generate,
            get_changelog,
            get_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
