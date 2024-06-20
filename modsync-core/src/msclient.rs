use crate::{
    msconfig::MSConfig,
    msmod::MSMOD,
    mstask::DownloadTask,
    utils::{http_download, http_get},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MODDiff {
    pub name: String,
    pub local: Option<MSMOD>,
    pub remote: Option<MSMOD>,
}

impl MODDiff {
    pub fn new(name: String, local: Option<MSMOD>, remote: Option<MSMOD>) -> MODDiff {
        MODDiff {
            name,
            local,
            remote,
        }
    }
}

pub struct MSClient<'a> {
    config: &'a MSConfig,
    path: Option<String>,
}

impl MSClient<'_> {
    pub fn config<'a>(config: &'a MSConfig) -> MSClient<'a> {
        MSClient { config, path: None }
    }

    pub fn path(&mut self, path: String) -> &MSClient {
        self.path = Some(path);
        self
    }

    pub async fn get_changelog(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match &self.config.changelog_url {
            Some(changelog_url) => http_get(changelog_url.as_str()).await,
            None => Err(Box::from("config dont contain changelog url".to_string())),
        }
    }
    pub async fn get_modlist(
        &self,
    ) -> Result<Vec<Option<MSMOD>>, Box<dyn std::error::Error + Send + Sync>> {
        match &self.config.modlist_url {
            Some(modlist_url) => Ok(serde_json::from_str(
                http_get(modlist_url.as_str()).await?.as_str(),
            )?),
            None => Err(Box::from("config dont contain modlist url".to_string())),
        }
    }
    pub async fn get_option(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match &self.config.option_url {
            Some(option_url) => http_get(option_url.as_str()).await,
            None => Err(Box::from("config dont contain option url".to_string())),
        }
    }
    pub async fn get_serverlist(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match &self.config.serverlist_url {
            Some(serverlist_url) => http_get(serverlist_url.as_str()).await,
            None => Err(Box::from("config dont contain serverlist url".to_string())),
        }
    }

    pub async fn sync_serverlist(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match &self.config.serverlist_url {
            Some(serverlist_url) => Ok(http_download(
                serverlist_url.as_str(),
                format!("{}/servers.dat", self.path.as_ref().unwrap()).as_str(),
            )
            .await?),
            None => Err(Box::from("config dont contain serverlist url".to_string())),
        }
    }
    pub async fn sync_option(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match &self.config.option_url {
            Some(option_url) => Ok(http_download(
                option_url.as_str(),
                format!("{}/option.txt", self.path.as_ref().unwrap()).as_str(),
            )
            .await?),
            None => Err(Box::from("config dont contain option url".to_string())),
        }
    }

    pub fn get_modlist_local(
        &self,
    ) -> Result<Vec<Option<MSMOD>>, Box<dyn std::error::Error + Send>> {
        let modspath = format!("{}/mods", self.path.as_ref().unwrap());
        let _ = std::fs::create_dir_all(modspath.as_str());
        MSMOD::from_directory(modspath.as_str(), None)
    }

    pub fn get_difflist(
        &self,
        remotelist: Vec<Option<MSMOD>>,
    ) -> Result<Vec<MODDiff>, Box<dyn std::error::Error + Send>> {
        let mut ret: Vec<MODDiff> = vec![];
        match self.get_modlist_local() {
            Ok(mut locallist) => {
                for remotemod_ in remotelist.iter() {
                    let remotemod = remotemod_.as_ref().unwrap();

                    let mut ok = false;
                    for localmod_ in locallist.iter_mut() {
                        if let Some(localmod) = localmod_.as_ref() {
                            if localmod.md5 == remotemod.md5 {
                                *localmod_ = None;
                                ok = true;
                                break;
                            }

                            if localmod.modid == remotemod.modid {
                                ret.push(MODDiff::new(
                                    remotemod.path.clone(),
                                    Some(localmod_.as_ref().unwrap().clone()),
                                    Some(remotemod.clone()),
                                ));
                                *localmod_ = None;
                                ok = true;
                                break;
                            }

                            if localmod.path == remotemod.path {
                                ret.push(MODDiff::new(
                                    remotemod.path.clone(),
                                    Some(localmod_.as_ref().unwrap().clone()),
                                    Some(remotemod.clone()),
                                ));
                                *localmod_ = None;
                                ok = true;
                                break;
                            }
                        }
                    }
                    if ok {
                        continue;
                    }

                    ret.push(MODDiff::new(
                        remotemod.path.clone(),
                        None,
                        Some(remotemod.clone()),
                    ))
                }

                for localmod_ in locallist.iter() {
                    if let Some(localmod) = localmod_ {
                        ret.push(MODDiff::new(
                            localmod.path.clone(),
                            Some(localmod.clone()),
                            None,
                        ))
                    }
                }

                Ok(ret)
            }
            Err(err) => Err(err),
        }
    }

    pub fn apply_diff(&self, diffs: &[MODDiff]) -> Vec<DownloadTask> {
        let mut tasks: Vec<DownloadTask> = vec![];
        for diff in diffs {
            if let Some(local) = &diff.local {
                let _ = std::fs::remove_file(format!(
                    "{}/mods/{}",
                    self.path.as_ref().unwrap().as_str(),
                    local.path.as_str()
                ));
            }

            if let Some(remote) = &diff.remote {
                tasks.push(DownloadTask::build(
                    diff.name.clone(),
                    remote.url.as_ref().unwrap().clone(),
                    format!(
                        "{}/mods/{}",
                        self.path.as_ref().unwrap(),
                        remote.path.as_str()
                    ),
                ));
            }
        }
        tasks
    }
}
