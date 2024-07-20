use crate::error::Error;

use std::{fs::File, io::Read};

use crate::utils::http_get;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSConfig {
    pub base_url: String,
    pub changelog_url: Option<String>,
    pub modlist_url: Option<String>,
    pub option_url: Option<String>,
    pub serverlist_url: Option<String>,
    pub necessary_url: Option<String>,
    pub force_sync_server_list: bool,
    pub title: String,
}
impl MSConfig {
    pub fn new(
        base_url: String,
        changelog_url: Option<String>,
        modlist_url: Option<String>,
        necessary_url: Option<String>,
        option_url: Option<String>,
        serverlist_url: Option<String>,
        force_sync_server_list: bool,
        title: String,
    ) -> MSConfig {
        MSConfig {
            base_url,
            changelog_url,
            modlist_url,
            necessary_url,
            option_url,
            serverlist_url,
            force_sync_server_list,
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

    pub async fn get_remote_config() -> Result<MSConfig, Error> {
        Ok(serde_json::from_str::<MSConfig>(
            http_get("https://ms.nicefish4520.com/info.json")
                .await?
                .text
                .as_str(),
        )?)
    }
}
