use crate::{error::Error, msmod::MSMOD};

use std::{fs::File, io::Read};

use crate::utils::http_get;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetaData {
    pub options_url: Option<String>,
    pub serverdat_url: Option<String>,
    pub configpack: Option<MSMOD>,
    pub launcher_hmcl_url: Option<String>,
    pub launcher_pclce_url: Option<String>,
}
impl MetaData {
    pub fn new(
        options_url: Option<String>,
        serverdat_url: Option<String>,
        configpack: Option<MSMOD>,
        launcher_hmcl_url: Option<String>,
        launcher_pclce_url: Option<String>,
    ) -> MetaData {
        MetaData {
            options_url,
            serverdat_url,
            configpack,
            launcher_hmcl_url,
            launcher_pclce_url,
        }
    }
}

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
    pub metadata: Option<MetaData>,
    pub title: String,
}
impl MSConfig {
    pub fn new(
        base_url: String,
        modlist_url: Option<String>,
        release_info: Vec<ReleaseInfo>,
        metadata: Option<MetaData>,
        title: String,
    ) -> MSConfig {
        MSConfig {
            base_url,
            release_info,
            modlist_url,
            metadata,
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

    pub async fn get_remote_config(url: &str) -> Result<MSConfig, Error> {
        Ok(serde_json::from_str::<MSConfig>(
            http_get(url).await?.text.as_str(),
        )?)
    }
}
