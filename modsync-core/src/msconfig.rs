use crate::{error::Error, msmod::MSMOD};

use std::{fs::File, io::Read};

use crate::utils::http_get;

pub const CONFIG_SCHEMA_VERSION: u32 = 1;

fn default_schema_version() -> u32 {
    CONFIG_SCHEMA_VERSION
}

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
    /// Absent in legacy documents; those are interpreted as version 1.
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
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
            schema_version: CONFIG_SCHEMA_VERSION,
            base_url,
            release_info,
            modlist_url,
            metadata,
            title,
        }
    }

    pub fn from_json(json: &str) -> Result<MSConfig, Error> {
        Ok(serde_json::from_str::<MSConfig>(json)?)
    }
    pub fn from_file(filepath: &str) -> Result<MSConfig, Error> {
        let mut file = File::open(filepath)?;
        let mut str: String = "".to_string();
        file.read_to_string(&mut str)?;
        MSConfig::from_json(str.as_str())
    }

    pub async fn get_remote_config(url: &str) -> Result<MSConfig, Error> {
        Ok(serde_json::from_str::<MSConfig>(
            http_get(url).await?.text.as_str(),
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_config_defaults_to_schema_version_one() {
        let config = MSConfig::from_json(
            r#"{"base_url":"https://example.test/","release_info":[],"modlist_url":null,"metadata":null,"title":"Example"}"#,
        )
        .expect("legacy config should remain readable");

        assert_eq!(config.schema_version, CONFIG_SCHEMA_VERSION);
    }
}
