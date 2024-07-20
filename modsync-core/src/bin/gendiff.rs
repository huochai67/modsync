use modsync_core::{msclient::MSClientBuilder, msconfig::MSConfig};

#[tokio::main]
async fn main() {
    let config = MSConfig::get_remote_config()
        .await
        .expect("get remote config error");
    let client = MSClientBuilder::new().msconfig(config).path("./".into()).build();

    if let Ok(locallist) = client.get_modlist_local() {
        println!("{:?}", locallist);
    }
    match client.get_difflist().await {
        Ok(difflist) => {
            println!("{:?}", difflist);
        }
        Err(err) => panic!("{}", err),
    }

    println!("ok");
}
