mod config;
mod overseerr;

use config::Config;

#[tokio::main]
async fn main() {
    if let Err(err) = Config::read_conf() {
        eprintln!("Error reading config: {}", err);
        std::process::exit(1);
    }

    let requests = overseerr::get_requests().await.unwrap();

    println!("{:?}", requests);
}
