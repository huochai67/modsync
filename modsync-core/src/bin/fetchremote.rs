use modsync_core::{msclient::MSClient, msconfig::MSConfig};

#[tokio::main]
async fn main() {
    if let Ok(config) = MSConfig::get_remote_config().await {
        if let Ok(modlist) = MSClient::config(&config).get_modlist().await {
            println!("{:?}", modlist);
        }
    }
}
