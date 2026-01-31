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
                format!("{}/option.txt", self.get_path().unwrap()).as_str(),
            )
            .await?),
            None => Err(Error::MSConfigNoOptionsUrl),
        }
    }
    pub async fn sync_hcml(&self) -> Result<(), Error> {
        match self.get_launcher_hmcl() {
            Some(hmcl_url) => Ok(http_download(
                hmcl_url.as_str(),
                format!("{}/hmcl.exe", self.get_path().unwrap()).as_str(),
            )
            .await?),
            None => Err(Error::MSConfigNoHMCLUrl),
        }
    }
    pub async fn sync_pclce(&self) -> Result<(), Error> {
        match self.get_launcher_pclce() {
            Some(pclce_url) => Ok(http_download(
                pclce_url.as_str(),
                format!("{}/PCL-CE.exe", self.get_path().unwrap()).as_str(),
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
        locallist: &Vec<MSMOD>,
        remotelist: &Vec<MSMOD>,
        necessarylist: Option<&Vec<MSMOD>>,
    ) -> Result<Vec<MODDiff>, Error> {
        let mut ret: Vec<MODDiff> = vec![];
        let mut copy_locallist = locallist.clone();
        for remotemod_ in remotelist.iter() {
            let mut ok = false;
            for localmod in copy_locallist.iter() {
                //优先通过md5校验
                if localmod.md5 == remotemod_.md5 {
                    ok = true;
                    break;
                }

                //其次通过modid校验
                if remotemod_.modid.is_some() {
                    if localmod.modid == remotemod_.modid {
                        ret.push(MODDiff::new(
                            Kind::MOD,
                            remotemod_.path.clone(),
                            Some(localmod.clone()),
                            Some(remotemod_.clone()),
                        ));
                        ok = true;
                        break;
                    }
                }
                //最后通过路径校验
                if localmod.path == remotemod_.path {
                    ret.push(MODDiff::new(
                        Kind::MOD,
                        remotemod_.path.clone(),
                        Some(localmod.clone()),
                        Some(remotemod_.clone()),
                    ));
                    ok = true;
                    break;
                }
            }
            if !ok {
                ret.push(MODDiff::new(
                    Kind::MOD,
                    remotemod_.path.clone(),
                    None,
                    Some(remotemod_.clone()),
                ))
            } else {
                //删除已经匹配的本地mod，减少后续循环开销
                copy_locallist.retain(|x| x.md5 != remotemod_.md5);
            }
        }

        for localmod in copy_locallist.iter() {
            let mut ok = false;
            if let Some(necessarylist) = necessarylist {
                for necessary in necessarylist.iter() {
                    if localmod.md5 == necessary.md5 {
                        ok = true;
                        break;
                    }
                }
            }

            if !ok {
                ret.push(MODDiff::new(
                    Kind::MOD,
                    localmod.path.clone(),
                    Some(localmod.clone()),
                    None,
                ));
            }
        }

        Ok(ret)
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
