use modsync_core::{msclient::MSClientBuilder, msconfig::MSConfig};

#[tokio::main]
async fn main() {
    let config = MSConfig::get_remote_config("http://127.0.0.1:8086/info.json")
        .await
        .expect("get remote config error");
    let client = match MSClientBuilder::new().msconfig(config).path("./").build() {
        Ok(client) => client,
        Err(err) => panic!("{:?}", err),
    };

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
