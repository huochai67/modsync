use std::{fs::File, io::Read};

use crate::utils::http_get;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSConfig {
    pub base_url: String,
    pub changelog_url: String,
    pub modlist_url: String,
    pub option_url: String,
    pub serverlist_url: String,
    pub force_sync_server_list: bool,
    pub title: String,
}
impl MSConfig {
    pub fn new(
        base_url: String,
        changelog_url: String,
        modlist_url: String,
        option_url: String,
        serverlist_url: String,
        force_sync_server_list: bool,
        title: String,
    ) -> MSConfig {
        MSConfig {
            base_url,
            changelog_url,
            modlist_url,
            option_url,
            serverlist_url,
            force_sync_server_list,
            title,
        }
    }

    pub fn from_str(json: &str) -> Result<MSConfig, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str::<MSConfig>(json)?)
    }
    pub fn from_file(filepath: &str) -> Result<MSConfig, Box<dyn std::error::Error>> {
        let mut file = File::open(filepath)?;
        let mut str: String = "".to_string();
        file.read_to_string(&mut str)?;
        Ok(MSConfig::from_str(str.as_str())?)
    }

    pub fn has_changelog(&self) -> bool {
        self.changelog_url != "null"
    }
    pub fn has_modlist(&self) -> bool {
        self.modlist_url != "null"
    }
    pub fn has_option(&self) -> bool {
        self.option_url != "null"
    }
    pub fn has_serverlist(&self) -> bool {
        self.serverlist_url != "null"
    }

    pub fn get_title(&self) -> String {
        self.title.to_string()
    }

    pub async fn get_remote_config() -> Result<MSConfig, Box<dyn std::error::Error + Send + Sync>> {
        Ok(serde_json::from_str::<MSConfig>(
            http_get("https://ms.nicefish4520.com/info.json")
                .await?
                .as_str(),
        )?)
    }
}
