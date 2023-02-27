mod config;

use config::{read_conf, Config};

#[tokio::main]
async fn main() {
    match read_conf() {
        Ok(config) => {
            println!("Config: {:?}", config);
            overseerr_test(&config).await;
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

async fn overseerr_test(config: &Config) {
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "{}/api/v1/user?take=20&skip=0&sort=created",
            &config.overseerr_url
        ))
        .header("X-API-Key", &config.overseerr_token)
        .send()
        .await;

    match res {
        Ok(res) => {
            println!("Status: {}", res.status());
            println!("Body: {:?}", res.text().await.unwrap());
        }
        Err(err) => println!("Error: {}", err),
    }
}
