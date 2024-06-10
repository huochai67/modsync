use modsync_core::{msclient::MSClient, msconfig::MSConfig, msmod::MSMOD};

fn main() {
    match MSMOD::from_directory("./mods", None) {
        Ok(vecmsmod) => {
            println!("{:?}", vecmsmod);
        }
        Err(err) => panic!("{}", err),
    };


    println!("ok");
}
