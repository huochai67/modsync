use crate::{
    error::Error,
    msconfig::{MetaData, ReleaseInfo},
};
use std::sync::Arc;

use crate::{
    msconfig::MSConfig,
    msmod::MSMOD,
    utils::{http_download, http_get},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Kind {
    PLAIN = 0,
    MOD = 1,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum DiffType {
    MODIFIED = 0,
    NEWED = 1,
    DELETED = 2,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MODDiff {
    pub kind: Kind,
    pub name: String,
    pub difftype: DiffType,
    pub local: Option<MSMOD>,
    pub remote: Option<MSMOD>,
}

impl MODDiff {
    pub fn new(kind: Kind, name: String, local: Option<MSMOD>, remote: Option<MSMOD>) -> MODDiff {
        MODDiff {
            kind,
            name,
            difftype: match (local.is_some(), remote.is_some()) {
                (true, true) => DiffType::MODIFIED,
                (false, true) => DiffType::NEWED,
                (true, false) => DiffType::DELETED,
                (false, false) => panic!("both local and remote modinfo are none"),
            },
            local,
            remote,
        }
    }
}

#[derive(Clone)]
pub struct MSClient {
    inner: Arc<MSClientRef>,
}

pub struct MSClientRef {
    path: Option<String>,
    msconfig: MSConfig,
}

struct ClientBuilderConfig {
    path: Option<String>,
    remoteconfig: Option<MSConfig>,
}
pub struct MSClientBuilder {
    config: ClientBuilderConfig,
}

impl MSClientBuilder {
    pub fn new() -> MSClientBuilder {
        MSClientBuilder {
            config: ClientBuilderConfig {
                path: None,
                remoteconfig: None,
            },
        }
    }

    pub fn msconfig(mut self, config: MSConfig) -> MSClientBuilder {
        self.config.remoteconfig = Some(config);
        self
    }

    pub fn path(mut self, path: String) -> MSClientBuilder {
        self.config.path = Some(path);
        self
    }

    pub fn build(self) -> Result<MSClient, Error> {
        match self.config.remoteconfig {
            Some(config) => Ok(MSClient {
                inner: Arc::new(MSClientRef {
                    path: self.config.path,
                    msconfig: config,
                }),
            }),
            None => Err(Error::BuilderNoMSConfig),
        }
    }
}
impl MSClient {
    pub fn get_path(&self) -> Option<String> {
        self.inner.as_ref().path.clone()
    }

    pub fn get_config(&self) -> MSConfig {
        self.inner.as_ref().msconfig.clone()
    }

    pub fn get_title(&self) -> String {
        self.get_config().title
    }

    pub fn get_release_info(&self) -> Vec<ReleaseInfo> {
        self.get_config().release_info
    }

    pub async fn get_modlist(&self) -> Result<Vec<MSMOD>, Error> {
        match &self.inner.as_ref().msconfig.modlist_url {
            Some(modlist_url) => Ok(serde_json::from_str(
                http_get(modlist_url.as_str()).await?.text.as_str(),
            )?),
            None => Err(Error::MSConfigNoModListUrl),
        }
    }

    pub fn get_metadata(&self) -> Option<MetaData> {
        self.get_config().metadata
    }

    pub fn get_configpack(&self) -> Option<MSMOD> {
        match self.get_metadata() {
            Some(metadata) => metadata.configpack,
            None => None,
        }
    }
    pub fn get_options(&self) -> Option<String> {
        match self.get_metadata() {
            Some(metadata) => metadata.options_url,
            None => None,
        }
    }
    pub fn get_serverdat(&self) -> Option<String> {
        match self.get_metadata() {
            Some(metadata) => metadata.serverdat_url,
            None => None,
        }
    }
    pub fn get_launcher_hmcl(&self) -> Option<String> {
        match self.get_metadata() {
            Some(metadata) => metadata.launcher_hmcl_url,
            None => None,
        }
    }
    pub fn get_launcher_pclce(&self) -> Option<String> {
        match self.get_metadata() {
            Some(metadata) => metadata.launcher_pclce_url,
            None => None,
        }
    }

    pub async fn sync_serverdat(&self) -> Result<(), Error> {
        match self.get_serverdat() {
            Some(serverdat_url) => Ok(http_download(
                serverdat_url.as_str(),
                format!("{}/servers.dat", self.get_path().unwrap()).as_str(),
            )
            .await?),
            None => Err(Error::MSConfigNoServerDatUrl),
        }
    }
    pub async fn sync_options(&self) -> Result<(), Error> {
        match self.get_options() {
            Some(option_url) => Ok(http_download(
                option_url.as_str(),
                format!("{}/options.txt", self.get_path().unwrap()).as_str(),
            )
            .await?),
            None => Err(Error::MSConfigNoOptionsUrl),
        }
    }
    pub async fn sync_hcml(&self) -> Result<(), Error> {
        match self.get_launcher_hmcl() {
            Some(hmcl_url) => Ok(http_download(
                hmcl_url.as_str(),
                format!("{}/../HMCL.exe", self.get_path().unwrap()).as_str(),
            )
            .await?),
            None => Err(Error::MSConfigNoHMCLUrl),
        }
    }
    pub async fn sync_pclce(&self) -> Result<(), Error> {
        match self.get_launcher_pclce() {
            Some(pclce_url) => Ok(http_download(
                pclce_url.as_str(),
                format!("{}/../PCL-CE.exe", self.get_path().unwrap()).as_str(),
            )
            .await?),
            None => Err(Error::MSConfigNoPCLCEUrl),
        }
    }

    pub fn get_modlist_local(&self) -> Result<Vec<MSMOD>, Error> {
        let modspath = format!("{}/mods", self.get_path().unwrap().as_str());
        let _ = std::fs::create_dir_all(modspath.as_str());
        MSMOD::from_directory(modspath.as_str(), None)
    }

    pub fn get_difflist_with(
        locallist: &[MSMOD],
        remotelist: &[MSMOD],
        necessarylist: Option<&[MSMOD]>,
    ) -> Result<Vec<MODDiff>, Error> {
        let mut diffs = Vec::new();
        // An index is consumed exactly once. This prevents a modified file from
        // subsequently appearing as a deletion in the same sync plan.
        let mut matched = vec![false; locallist.len()];

        for remote in remotelist {
            let find_unmatched = |predicate: &dyn Fn(&MSMOD) -> bool| {
                locallist
                    .iter()
                    .enumerate()
                    .find(|(index, local)| !matched[*index] && predicate(local))
                    .map(|(index, _)| index)
            };

            let local_index = find_unmatched(&|local| local.md5 == remote.md5)
                .or_else(|| {
                    remote.modid.as_ref().and_then(|modid| {
                        find_unmatched(&|local| local.modid.as_ref() == Some(modid))
                    })
                })
                .or_else(|| find_unmatched(&|local| local.path == remote.path));

            match local_index {
                Some(index) => {
                    matched[index] = true;
                    if locallist[index].md5 != remote.md5 {
                        diffs.push(MODDiff::new(
                            Kind::MOD,
                            remote.path.clone(),
                            Some(locallist[index].clone()),
                            Some(remote.clone()),
                        ));
                    }
                }
                None => diffs.push(MODDiff::new(
                    Kind::MOD,
                    remote.path.clone(),
                    None,
                    Some(remote.clone()),
                )),
            }
        }

        for (index, local) in locallist.iter().enumerate() {
            let required =
                necessarylist.is_some_and(|items| items.iter().any(|item| item.md5 == local.md5));
            if !matched[index] && !required {
                diffs.push(MODDiff::new(
                    Kind::MOD,
                    local.path.clone(),
                    Some(local.clone()),
                    None,
                ));
            }
        }

        Ok(diffs)
    }

    pub async fn get_difflist(&self) -> Result<Vec<MODDiff>, Error> {
        let modlist_local = self.get_modlist_local()?;
        match self.get_modlist().await {
            Ok(modlist_remote) => {
                // let necessarylist = self.get_necessary().await?;
                Self::get_difflist_with(
                    // self.inner.as_ref().path.as_ref().unwrap().into(),
                    &modlist_local,
                    &modlist_remote,
                    None,
                )
            }
            Err(err) => Err(err),
        }
    }

    // pub async fn apply_diff(
    //     &self,
    //     diffs: &[MODDiff],
    // ) -> Result<Vec<Box<dyn MSTask + Send>>, Error> {
    //     let mut tasks: Vec<Box<dyn MSTask + Send>> = vec![];
    //     let client = reqwest::Client::builder()
    //         .timeout(Duration::from_secs(10))
    //         .build()?;
    //     for diff in diffs {
    //         //优先使用本地路径
    //         let modpath = if let Some(localmod) = &diff.local {
    //             localmod.path.as_ref()
    //         } else if let Some(remotemod) = &diff.remote {
    //             remotemod.path.as_ref()
    //         } else {
    //             panic!("apply a moddiff which dont contain a vaild modinfo");
    //             #[allow(unreachable_code)]
    //             ""
    //         };

    //         //根据类型生成全路径
    //         let fullpath = match diff.kind {
    //             Kind::PLAIN => format!(
    //                 "{}/{}",
    //                 self.inner.as_ref().path.as_ref().unwrap().as_str(),
    //                 modpath
    //             ),
    //             Kind::MOD => format!(
    //                 "{}/mods/{}",
    //                 self.inner.as_ref().path.as_ref().unwrap().as_str(),
    //                 modpath
    //             ),
    //         };

    //         if let Some(_local) = &diff.local {
    //             //存在本地与远程文件，进行下载覆盖
    //             if let Some(remote) = &diff.remote {
    //                 let cc: reqwest::Client = client.clone();
    //                 tasks.push(Box::new(DownloadTask::build(
    //                     cc,
    //                     diff.name.clone(),
    //                     remote.url.as_ref().unwrap().clone(),
    //                     fullpath,
    //                 )))
    //             } else {
    //                 //不存在远程文件，直接删除
    //                 tasks.push(Box::new(DeleteTask::build(
    //                     diff.name.clone(),
    //                     fullpath.into(),
    //                 )))
    //             }
    //         }
    //     }
    //     Ok(tasks)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mod_file(path: &str, md5: &str) -> MSMOD {
        MSMOD::new(md5.to_string(), path.to_string(), 1, None, None, None)
    }

    #[test]
    fn diff_identifies_add_modify_and_delete() {
        let local = vec![
            mod_file("same.jar", "a"),
            mod_file("removed.jar", "b"),
            mod_file("changed.jar", "c"),
        ];
        let remote = vec![
            mod_file("same.jar", "a"),
            mod_file("added.jar", "d"),
            mod_file("changed.jar", "e"),
        ];
        let diffs = MSClient::get_difflist_with(&local, &remote, None).unwrap();

        assert_eq!(diffs.len(), 3);
        assert!(diffs
            .iter()
            .any(|diff| diff.name == "added.jar" && matches!(diff.difftype, DiffType::NEWED)));
        assert!(diffs
            .iter()
            .any(|diff| diff.name == "removed.jar" && matches!(diff.difftype, DiffType::DELETED)));
        assert!(diffs
            .iter()
            .any(|diff| diff.name == "changed.jar" && matches!(diff.difftype, DiffType::MODIFIED)));
        assert!(!diffs
            .iter()
            .any(|diff| diff.name == "changed.jar" && matches!(diff.difftype, DiffType::DELETED)));
    }
}
