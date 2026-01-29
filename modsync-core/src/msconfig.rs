use crate::{error::Error, msmod::MSMOD};

use std::{fs::File, io::Read};

use crate::utils::http_get;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReleaseInfo {
    pub version: String,
    pub changelog: String,
    pub date: String,
    pub adds: Option<Vec<String>>,
    pub subs: Option<Vec<String>>,
    pub mods: Option<Vec<String>>,
    pub size: Option<isize>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSConfig {
    pub base_url: String,
    pub release_info: Vec<ReleaseInfo>,
    pub modlist_url: Option<String>,
    pub option_url: Option<String>,
    pub serverlist_url: Option<String>,
    pub configpack: Option<MSMOD>,
    pub title: String,
}
impl MSConfig {
    pub fn new(
        base_url: String,
        modlist_url: Option<String>,
        release_info: Vec<ReleaseInfo>,
        option_url: Option<String>,
        serverlist_url: Option<String>,
        configpack: Option<MSMOD>,
        title: String,
    ) -> MSConfig {
        MSConfig {
            base_url,
            release_info,
            modlist_url,
            option_url,
            serverlist_url,
            configpack,
            title,
        }
    }

    pub fn from_str(json: &str) -> Result<MSConfig, Error> {
        Ok(serde_json::from_str::<MSConfig>(json)?)
    }
    pub fn from_file(filepath: &str) -> Result<MSConfig, Error> {
        let mut file = File::open(filepath)?;
        let mut str: String = "".to_string();
        file.read_to_string(&mut str)?;
        Ok(MSConfig::from_str(str.as_str())?)
    }

    pub fn get_title(&self) -> String {
        self.title.to_string()
    }

    pub async fn get_remote_config(url: &str) -> Result<MSConfig, Error> {
        Ok(serde_json::from_str::<MSConfig>(
            http_get(url).await?.text.as_str(),
        )?)
    }
}
