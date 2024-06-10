use crate::{
    msconfig::MSConfig,
    msmod::MSMOD,
    mstask::{DownloadTask},
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
        http_get(self.config.changelog_url.as_str()).await
    }
    pub async fn get_modlist(
        &self,
    ) -> Result<Vec<MSMOD>, Box<dyn std::error::Error + Send + Sync>> {
        let modlist: Vec<MSMOD> =
            serde_json::from_str(http_get(self.config.modlist_url.as_str()).await?.as_str())?;
        Ok(modlist)
    }
    pub async fn get_option(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        http_get(self.config.option_url.as_str()).await
    }
    pub async fn get_serverlist(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        http_get(self.config.serverlist_url.as_str()).await
    }

    pub async fn sync_serverlist(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match http_download(
            self.config.serverlist_url.as_str(),
            format!("{}/servers.dat", self.path.as_ref().unwrap()).as_str(),
        )
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn sync_option(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match http_download(
            self.config.option_url.as_str(),
            format!("{}/option.txt", self.path.as_ref().unwrap()).as_str(),
        )
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn get_modlist_local(&self) -> Result<Vec<MSMOD>, Box<dyn std::error::Error + Send>> {
        let modspath = format!("{}/mods", self.path.as_ref().unwrap());
        let _ = std::fs::create_dir_all(modspath.as_str());
        MSMOD::from_directory(modspath.as_str(), None)
    }

    pub fn get_difflist(
        &self,
        remotelist: Vec<MSMOD>,
    ) -> Result<Vec<MODDiff>, Box<dyn std::error::Error + Send>> {
        let mut ret: Vec<MODDiff> = vec![];
        match self.get_modlist_local() {
            Ok(locallist) => {
                for localmod in locallist.iter() {
                    let mut ok = false;
                    for remotemod in remotelist.iter() {
                        if remotemod.md5 == localmod.md5 {
                            ok = true;
                            break;
                        }
                    }
                    if !ok {
                        ret.push(MODDiff::new(
                            localmod.path.clone(),
                            Some(localmod.clone()),
                            None,
                        ))
                    }
                }

                for remotemod in remotelist.iter() {
                    let mut ok = false;
                    for localmod in locallist.iter() {
                        if remotemod.md5 == localmod.md5 {
                            ok = true;
                            break;
                        }
                    }
                    if !ok {
                        ret.push(MODDiff::new(
                            remotemod.path.clone(),
                            None,
                            Some(remotemod.clone()),
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
                let _ = std::fs::remove_file(format!("{}/mods/{}", self.path.as_ref().unwrap().as_str(), local.path.as_str()));
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
