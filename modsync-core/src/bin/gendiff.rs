use modsync_core::{msclient::MSClient, msconfig::MSConfig};

#[tokio::main]
async fn main() {
    let config = MSConfig::get_remote_config()
        .await
        .expect("get remote config error");
    let mut client = MSClient::config(&config);
    let client2: &MSClient = client.path("./".into());
    match client2.get_modlist().await {
        Ok(modlist) => {
            match client2.get_difflist(modlist){
                Ok(difflist) => {
                    println!("{:?}", difflist);
                },
                Err(err) => panic!("{}", err),
            }
        }
        Err(err) => panic!("{}", err),
    }

    println!("ok");
}
