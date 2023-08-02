use std::time::Duration;

mod utils;

const GUILD_ID: &'static str = "";
const VANITY_CODE: &'static str = "";
const INTERVAL_IN_MS: u64 = 5000;

#[tokio::main]
async fn main() {
    let tokens = utils::tokens_file_to_array().unwrap_or_else(|err| {
        println!("Could not read the tokens file: {}", err);
        std::process::exit(-1);
    });

    let client = reqwest::Client::new();
    for token in &tokens {
        match utils::update_vanity_url(&client, GUILD_ID, VANITY_CODE, token).await {
            Ok(status) => println!("Request sent, status: {}", status),
            Err(err) => eprintln!("Request error: {}", err),
        }
        tokio::time::sleep(Duration::from_millis(INTERVAL_IN_MS)).await;
    }
}
