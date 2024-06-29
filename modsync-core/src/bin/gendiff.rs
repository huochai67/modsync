use modsync_core::{msclient::MSClient, msconfig::MSConfig};

#[tokio::main]
async fn main() {
    let config = MSConfig::get_remote_config()
        .await
        .expect("get remote config error");
    let mut client = MSClient::config(&config);
    let client2: &MSClient = client.path("./".into());

    if let Ok(locallist) = client2.get_modlist_local() {
        println!("{:?}", locallist);
    }
    match client2.get_difflist().await {
        Ok(difflist) => {
            println!("{:?}", difflist);
        }
        Err(err) => panic!("{}", err),
    }

    println!("ok");
}
