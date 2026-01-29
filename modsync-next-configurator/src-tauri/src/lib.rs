use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::vec;

use modsync_core::msclient::MSClient;
use modsync_core::msconfig::MSConfig;
use modsync_core::msconfig::ReleaseInfo;
use modsync_core::msmod::MSMOD;

fn writetofile(filepath: &str, data: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filepath)?
        .write(data)?)
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
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(new_entry.as_bytes())?;

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
    modlist: &Vec<MSMOD>,
    old_modlist: &Vec<MSMOD>,
) -> ReleaseInfo {
    let mut size: isize = 0;
    let mut adds = vec![];
    let mut subs = vec![];
    let mut mods = vec![];

    if let Ok(difflist) = MSClient::get_difflist_with(&old_modlist, &modlist, None) {
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

    return ReleaseInfo {
        version: version.to_string(),
        date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        changelog: changelog.to_string(),
        size: Some(size),
        adds: Some(adds),
        subs: Some(subs),
        mods: Some(mods),
    };
}

#[tauri::command]
fn generate(version: &str, changelog: &str, title: &str, serverurl: &str) -> Result<(), String> {
    let _ = create_dir_all("./data/data");

    if !Path::new("./data/data/mods").exists() {
        return Err("无mod文件夹".into());
    }

    let modlist_url = Some(format!("{}modslist.json", serverurl));
    let old_modlist = match MSMOD::from_jsonfile("./data/modslist.json") {
        Ok(old_modlist) => old_modlist,
        Err(_) => vec![],
    };
    let vecmsmod = match MSMOD::from_directory(
        "./data/data/mods",
        Some(format!("{}data/mods/", serverurl).as_str()),
    ) {
        Ok(vecmsmod) => vecmsmod,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    let releaseinfo = generate_releaseinfo(version, changelog, &vecmsmod, &old_modlist);
    if let Some(size) = releaseinfo.size {
        if size == 0 {
            return Err("你真的改了什么吗".into());
        }
    }
    if let Err(err) = writetofile(
        "./data/modslist.json",
        serde_json::to_string(&vecmsmod).unwrap().as_bytes(),
    ) {
        return Err(err.to_string());
    }

    let logs = match update_and_get_logs(releaseinfo, "./data/release_logs.txt") {
        Ok(logs) => logs,
        Err(err) => return Err(err.to_string()),
    };

    // let mut neccesary_url = None;
    // if Path::new("./data/data/necessary").exists() {
    //     neccesary_url = Some(format!("{}necessary.json", serverurl));
    //     match MSMOD::from_directory(
    //         "./data/data/necessary",
    //         Some(format!("{}data/necessary/", serverurl).as_str()),
    //     ) {
    //         Ok(vecmsmod) => {
    //             if let Err(err) = writetofile(
    //                 "./data/necessary.json",
    //                 serde_json::to_string(&vecmsmod).unwrap().as_bytes(),
    //             ) {
    //                 return Err(err.to_string());
    //             }
    //         }
    //         Err(err) => {
    //             return Err(err.to_string());
    //         }
    //     }
    // }

    let mut option_url = None;
    let mut serverlist_url = None;
    // if Path::new("./data/changelog.txt").exists() {
    //     changelog_url = Some(format!("{}changelog.txt", serverurl));
    // }
    if Path::new("./data/data/options.txt").exists() {
        option_url = Some(format!("{}data/options.txt", serverurl));
    }
    if Path::new("./data/data/servers.dat").exists() {
        serverlist_url = Some(format!("{}data/servers.dat", serverurl));
    }
    let configpack = match Path::new("./data/data/config.zip").exists() {
        true => Some(MSMOD::from_file(
            Path::new("./data/data/config.zip"),
            "./data/data/",
            Some(format!("{}data/mods/", serverurl).as_str()),
        )),
        false => None,
    };
    let config = MSConfig::new(
        serverurl.to_string(),
        modlist_url,
        logs,
        option_url,
        serverlist_url,
        configpack,
        title.to_string(),
    );
    if let Err(err) = writetofile(
        "./data/info.json",
        serde_json::to_string(&config).unwrap().as_bytes(),
    ) {
        return Err(err.to_string());
    }

    Ok(())
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
