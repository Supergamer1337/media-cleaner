mod config;
mod overseerr;

use config::read_conf;

#[tokio::main]
async fn main() {
    match read_conf() {
        Ok(config) => {
            let requests = overseerr::get_requests(&config).await.unwrap();
            println!("{:?}", requests);
        }
        Err(err) => {
            println!(
                "Failed to read config file with the following error: {}",
                err
            );
            println!("Make sure you have a config.yaml file in the same directory as the executable and that it is formatted correctly.");
        }
    }
}
