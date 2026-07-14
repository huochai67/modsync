use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::vec;

use modsync_core::msclient::MSClient;
use modsync_core::msconfig::MSConfig;
use modsync_core::msconfig::MetaData;
use modsync_core::msconfig::ReleaseInfo;
use modsync_core::msmod::MSMOD;

fn writetofile(filepath: &str, data: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
    let path = Path::new(filepath);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let temporary = path.with_extension(format!(
        "{}.tmp",
        path.extension().and_then(|v| v.to_str()).unwrap_or("file")
    ));
    let written = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&temporary)?
        .write(data)?;
    std::fs::rename(&temporary, path)?;
    Ok(written)
}

fn data_root() -> PathBuf {
    std::env::var("MODSYNC_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./data"))
}

/// 更新日志文件并返回所有日志
pub fn update_and_get_logs(new_log: ReleaseInfo, path: &str) -> std::io::Result<Vec<ReleaseInfo>> {
    let path = Path::new(path);

    // 0. 读取并解析整个文件返回数组
    let mut logs = read_all_logs(path)?;

    // 1. 构造写入字符串
    let new_entry = format!(
        "$riblockstart\n$version: {}\n$date: {}\n{}\n$adds: {}\n$subs: {}\n$mods: {}\n$size: {}\n$riblockend\n",
        new_log.version,
        new_log.date,
        new_log.changelog,
        new_log
            .adds
            .as_ref()
            .map_or("[]".to_string(), |v| format!("{:?}", v)),
        new_log
            .subs
            .as_ref()
            .map_or("[]".to_string(), |v| format!("{:?}", v)),
        new_log
            .mods
            .as_ref()
            .map_or("[]".to_string(), |v| format!("{:?}", v)),
        new_log.size.unwrap_or(0)
    );

    // 2. 追加写入文件 (如果文件不存在则创建)
    let mut updated = if path.exists() {
        std::fs::read_to_string(path)?
    } else {
        String::new()
    };
    updated.push_str(&new_entry);
    writetofile(&path.to_string_lossy(), updated.as_bytes())
        .map_err(|err| std::io::Error::other(err.to_string()))?;

    // 3. 返回所有日志
    logs.push(new_log);
    Ok(logs)
}

/// 从文件中解析所有日志
pub fn read_all_logs(path: &Path) -> std::io::Result<Vec<ReleaseInfo>> {
    if !path.exists() {
        return Ok(vec![]);
    }

    let content = std::fs::read_to_string(path)?;
    let mut logs = Vec::new();

    // 使用标记位分割块
    let blocks = content.split("$riblockstart");

    for block in blocks {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        // 去除结尾标记
        let clean_block = block.replace("$riblockend", "");
        let lines: Vec<&str> = clean_block.lines().collect();

        let mut version = String::new();
        let mut date = String::new();
        let mut changelog_lines = Vec::new();
        let mut size: Option<isize> = None;
        let mut adds: Option<Vec<String>> = None;
        let mut subs: Option<Vec<String>> = None;
        let mut mods: Option<Vec<String>> = None;

        for line in lines {
            if line.starts_with("$version: ") {
                version = line.replace("$version: ", "").trim().to_string();
            } else if line.starts_with("$date: ") {
                date = line.replace("$date: ", "").trim().to_string();
            } else if line.starts_with("$adds: ") {
                // 解析 adds 字段
                adds = Some(
                    line.replace("$adds: ", "")
                        .trim()
                        .trim_matches(&['[', ']'][..])
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                );
            } else if line.starts_with("$subs: ") {
                // 解析 subs 字段
                subs = Some(
                    line.replace("$subs: ", "")
                        .trim()
                        .trim_matches(&['[', ']'][..])
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                );
            } else if line.starts_with("$mods: ") {
                // 解析 mods 字段
                mods = Some(
                    line.replace("$mods: ", "")
                        .trim()
                        .trim_matches(&['[', ']'][..])
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                );
            } else if line.starts_with("$size: ") {
                // 解析 size 字段
                size = line.replace("$size: ", "").trim().parse::<isize>().ok();
            } else {
                changelog_lines.push(line);
            }
        }

        logs.push(ReleaseInfo {
            version,
            date,
            changelog: changelog_lines.join("\n").trim().to_string(),
            size,
            adds,
            subs,
            mods,
        });
    }

    Ok(logs)
}

fn generate_releaseinfo(
    version: &str,
    changelog: &str,
    modlist: &[MSMOD],
    old_modlist: &[MSMOD],
) -> ReleaseInfo {
    let mut size: isize = 0;
    let mut adds = vec![];
    let mut subs = vec![];
    let mut mods = vec![];

    if let Ok(difflist) = MSClient::get_difflist_with(old_modlist, modlist, None) {
        for diff in difflist {
            match (diff.local.is_some(), diff.remote.is_some()) {
                (false, true) => {
                    // 新增
                    if let Some(remote_mod) = diff.remote {
                        size += remote_mod.size as isize;
                        adds.push(remote_mod.path);
                    }
                }
                (true, false) => {
                    // 删除
                    if let Some(local_mod) = diff.local {
                        size -= local_mod.size as isize;
                        subs.push(local_mod.path);
                    }
                }
                (true, true) => {
                    // 修改
                    if let Some(remote_mod) = diff.remote {
                        if let Some(local_mod) = diff.local {
                            size += remote_mod.size as isize - local_mod.size as isize;
                            mods.push(remote_mod.path);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    ReleaseInfo {
        version: version.to_string(),
        date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        changelog: changelog.to_string(),
        size: Some(size),
        adds: Some(adds),
        subs: Some(subs),
        mods: Some(mods),
    }
}

#[tauri::command]
fn generate(version: &str, changelog: &str, title: &str, serverurl: &str) -> Result<(), String> {
    if version.trim().is_empty() || changelog.trim().is_empty() || title.trim().is_empty() {
        return Err("版本、标题和更新日志不能为空".into());
    }
    if !(serverurl.starts_with("https://") || serverurl.starts_with("http://")) {
        return Err("Server URL 必须是 http:// 或 https:// 地址".into());
    }
    let base_url = if serverurl.ends_with('/') {
        serverurl.to_string()
    } else {
        format!("{serverurl}/")
    };
    let root = data_root();
    let data_dir = root.join("data");
    let mods_dir = data_dir.join("mods");
    create_dir_all(&data_dir).map_err(|err| err.to_string())?;

    // Build ModList
    if !mods_dir.exists() {
        return Err("无mod文件夹".into());
    }

    let modlist_url = Some(format!("{}modslist.json", base_url));
    let old_modlist_path = root.join("modslist.json");
    let old_modlist = MSMOD::from_jsonfile(&old_modlist_path.to_string_lossy()).unwrap_or_default();
    let vecmsmod = match MSMOD::from_directory(
        &mods_dir.to_string_lossy(),
        Some(format!("{}data/mods/", base_url).as_str()),
    ) {
        Ok(vecmsmod) => vecmsmod,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    if vecmsmod.is_empty() {
        return Err("没有可用的mod".into());
    }

    // Build logs
    let release_log_path = root.join("release_logs.txt");
    let mut logs = match read_all_logs(&release_log_path) {
        Ok(logs) => logs,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    let releaseinfo = generate_releaseinfo(version, changelog, &vecmsmod, &old_modlist);
    if let Some(size) = releaseinfo.size {
        if size == 0 {
            // todo!("没有更新内容");
        } else {
            logs = update_and_get_logs(releaseinfo, &release_log_path.to_string_lossy())
                .map_err(|err| err.to_string())?;
        }
    }
    if let Err(err) = writetofile(
        &old_modlist_path.to_string_lossy(),
        serde_json::to_string(&vecmsmod).unwrap().as_bytes(),
    ) {
        return Err(err.to_string());
    }

    // Build MetaData
    let mut option_url = None;
    let mut serverlist_url = None;
    let mut launcher_hmcl_url = None;
    let mut launcher_pclce_url = None;
    if data_dir.join("options.txt").exists() {
        option_url = Some(format!("{}data/options.txt", base_url));
    }
    if data_dir.join("servers.dat").exists() {
        serverlist_url = Some(format!("{}data/servers.dat", base_url));
    }
    if data_dir.join("hmcl.exe").exists() {
        launcher_hmcl_url = Some(format!("{}data/hmcl.exe", base_url));
    }
    if data_dir.join("pclce.exe").exists() {
        launcher_pclce_url = Some(format!("{}data/pclce.exe", base_url));
    }
    let config_zip = data_dir.join("config.zip");
    let configpack = match config_zip.exists() {
        true => Some(
            MSMOD::from_file(
                &config_zip,
                &data_dir.to_string_lossy(),
                Some(format!("{}data/", base_url).as_str()),
            )
            .map_err(|err| err.to_string())?,
        ),
        false => None,
    };
    let metadata = MetaData::new(
        option_url,
        serverlist_url,
        configpack,
        launcher_hmcl_url,
        launcher_pclce_url,
    );

    // Build Config
    let config = MSConfig::new(
        base_url,
        modlist_url,
        logs,
        Some(metadata),
        title.to_string(),
    );
    if let Err(err) = writetofile(
        &root.join("info.json").to_string_lossy(),
        serde_json::to_string(&config).unwrap().as_bytes(),
    ) {
        return Err(err.to_string());
    }

    Ok(())
}

#[tauri::command]
fn get_config() -> Option<MSConfig> {
    let path = data_root().join("info.json");
    if !path.exists() {
        return None;
    }
    if let Ok(config) = MSConfig::from_file(&path.to_string_lossy()) {
        return Some(config);
    }
    None
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            generate,
            get_changelog,
            get_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
