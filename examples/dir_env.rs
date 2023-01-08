#![deny(warnings)]
use std::env;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();
    if let (Some(dir),Some(port)) = (env::var_os("DIR"),env::var_os("PORT")){
        let port:u16 = port.into_string().unwrap().parse().unwrap(); 
        warp::serve(warp::fs::dir(dir.into_string().unwrap()))
        .run(([127, 0, 0, 1], port))
        .await;
    }else{
        println!("missing DIR and PORT env");
    }
    
}
