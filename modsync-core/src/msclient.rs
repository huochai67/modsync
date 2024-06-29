use std::path::{Path, PathBuf};

use crate::{
    msconfig::MSConfig,
    msmod::MSMOD,
    mstask::{DeleteTask, DownloadTask, MSTask},
    utils::{http_download, http_get},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Kind {
    PLAIN = 0,
    MOD = 1,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MODDiff {
    pub kind: Kind,
    pub name: String,
    pub local: Option<MSMOD>,
    pub remote: Option<MSMOD>,
}

impl MODDiff {
    pub fn new(kind: Kind, name: String, local: Option<MSMOD>, remote: Option<MSMOD>) -> MODDiff {
        MODDiff {
            kind,
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

    pub fn get_path(&self) -> String {
        self.path.as_ref().expect("no path").clone()
    }

    pub async fn get_changelog(
        &self,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        match &self.config.changelog_url {
            Some(changelog_url) => Ok(Some(http_get(changelog_url.as_str()).await?)),
            None => Ok(None),
        }
    }
    pub async fn get_modlist(
        &self,
    ) -> Result<Option<Vec<Option<MSMOD>>>, Box<dyn std::error::Error + Send + Sync>> {
        match &self.config.modlist_url {
            Some(modlist_url) => Ok(Some(serde_json::from_str(
                http_get(modlist_url.as_str()).await?.as_str(),
            )?)),
            None => Ok(None),
        }
    }
    pub async fn get_necessary(
        &self,
    ) -> Result<Option<Vec<Option<MSMOD>>>, Box<dyn std::error::Error + Send + Sync>> {
        match &self.config.necessary_url {
            Some(necessary_url) => Ok(Some(serde_json::from_str(
                http_get(necessary_url.as_str()).await?.as_str(),
            )?)),
            None => Ok(None),
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

    pub fn get_difflist_with(
        path: String,
        _locallist: Vec<Option<MSMOD>>,
        remotelist: Vec<Option<MSMOD>>,
        _necessarylist: Option<Vec<Option<MSMOD>>>,
    ) -> Result<Vec<MODDiff>, Box<dyn std::error::Error + Send>> {
        let mut locallist = _locallist.clone();
        let mut ret: Vec<MODDiff> = vec![];
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
                            Kind::MOD,
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
                            Kind::MOD,
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
                Kind::MOD,
                remotemod.path.clone(),
                None,
                Some(remotemod.clone()),
            ))
        }

        for localmod_ in locallist.iter() {
            if let Some(localmod) = localmod_ {
                ret.push(MODDiff::new(
                    Kind::MOD,
                    localmod.path.clone(),
                    Some(localmod.clone()),
                    None,
                ))
            }
        }

        if let Some(necessarylist) = _necessarylist {
            for _necessary in necessarylist.iter() {
                let necessary = _necessary.as_ref().unwrap();
                let localpathstr = format!("{}/{}", path.as_str(), necessary.path);
                let localpath = Path::new(localpathstr.as_str());
                if localpath.exists() {
                    let localfile = MSMOD::from_file(localpath, path.as_str(), None);
                    if localfile.md5 != necessary.md5 {
                        ret.push(MODDiff::new(
                            Kind::PLAIN,
                            necessary.path.clone(),
                            Some(localfile),
                            _necessary.clone(),
                        ));
                    };
                    continue;
                }
                ret.push(MODDiff::new(
                    Kind::PLAIN,
                    necessary.path.clone(),
                    None,
                    _necessary.clone(),
                ))
            }
        }

        Ok(ret)
    }

    pub async fn get_difflist(&self) -> Result<Vec<MODDiff>, Box<dyn std::error::Error + Send>> {
        let modlist_local = self.get_modlist_local()?;
        match self.get_modlist().await {
            Ok(_modlist_remote) => match _modlist_remote {
                Some(modlist_remote) => {
                    let necessarylist = match self.get_necessary().await {
                        Ok(necessarylist) => necessarylist,
                        Err(_) => None,
                    };
                    Self::get_difflist_with(
                        self.get_path(),
                        modlist_local,
                        modlist_remote,
                        necessarylist,
                    )
                }
                None => panic!("no modlist in config"),
            },
            Err(err) => Err(err),
        }
    }

    pub fn apply_diff(&self, diffs: &[MODDiff]) -> Vec<Box<dyn MSTask + Send + Sync>> {
        let mut tasks: Vec<Box<dyn MSTask + Send + Sync>> = vec![];
        for diff in diffs {
            match diff.kind {
                Kind::PLAIN => {
                    if let Some(local) = &diff.local {
                        tasks.push(Box::new(DeleteTask::build(
                            diff.name.clone(),
                            PathBuf::from(format!(
                                "{}/{}",
                                self.path.as_ref().unwrap().as_str(),
                                local.path.as_str()
                            )),
                        )));
                    }

                    if let Some(remote) = &diff.remote {
                        tasks.push(Box::new(DownloadTask::build(
                            diff.name.clone(),
                            remote.url.as_ref().unwrap().clone(),
                            format!("{}/{}", self.path.as_ref().unwrap(), remote.path.as_str()),
                        )));
                    }
                }
                Kind::MOD => {
                    if let Some(local) = &diff.local {
                        tasks.push(Box::new(DeleteTask::build(
                            diff.name.clone(),
                            PathBuf::from(format!(
                                "{}/mods/{}",
                                self.path.as_ref().unwrap().as_str(),
                                local.path.as_str()
                            )),
                        )));
                    }

                    if let Some(remote) = &diff.remote {
                        tasks.push(Box::new(DownloadTask::build(
                            diff.name.clone(),
                            remote.url.as_ref().unwrap().clone(),
                            format!(
                                "{}/mods/{}",
                                self.path.as_ref().unwrap(),
                                remote.path.as_str()
                            ),
                        )));
                    }
                }
            }
        }
        tasks
    }
}
