use clap::Parser;
use reqwest::StatusCode;
use std::time::Duration;

mod logger;
mod utils;

/// Simple, fast and efficient Discord vanity sniper written in Rust
#[derive(Parser)]
struct Args {
    /// ID of the guild to set the code in
    #[arg(short, long)]
    guild_id: String,

    /// Code to snipe
    #[arg(short, long)]
    code: String,

    /// Interval between requests in milliseconds
    #[arg(short, long, default_value_t = 5000)]
    interval: u64,
}

#[tokio::main]
async fn main() {
    logger::set_global_logger()
        .unwrap_or_else(|err| eprintln!("Failed to set the global logger: {}", err));

    let args = Args::parse();

    let tokens = utils::tokens_file_to_array().unwrap_or_else(|err| {
        log::error!("Could not read the tokens file: {}", err);
        std::process::exit(-1);
    });
    if tokens.len() < 1 {
        log::error!("No tokens are provided. Create a tokens.txt file and put as many tokens as you want. (One line = one token)");
        std::process::exit(0);
    }

    log::info!("Loaded {} tokens.", tokens.len());

    let client = reqwest::Client::new();
    let mut blacklisted_tokens: Vec<String> = vec![];
    let mut req: u32 = 0;

    loop {
        for token in &tokens {
            if blacklisted_tokens.contains(token) {
                continue;
            }
            req += 1;
            match utils::update_vanity_url(&client, &args.guild_id, &args.code, token).await {
                Ok(result) => {
                    let success = perform_action(
                        req,
                        &args.code,
                        token,
                        &tokens,
                        &mut blacklisted_tokens,
                        result,
                    )
                    .await;
                    if !success {
                        tokio::time::sleep(Duration::from_millis(args.interval)).await;
                        continue;
                    }
                }
                Err(err) => log::error!("Request error: {}", err),
            }
            tokio::time::sleep(Duration::from_millis(args.interval)).await;
        }
    }
}

async fn perform_action(
    req: u32,
    vanity_code: &str,
    token: &str,
    tokens: &Vec<String>,
    blacklisted_tokens: &mut Vec<String>,
    (result, status, duration_in_ms): (utils::UpdateVanityURLResult, StatusCode, u128),
) -> bool {
    match result {
        utils::UpdateVanityURLResult::Success(_) => {
            log::info!(
                "Successfully sniped the vanity code: {} (req number: {}, duration: {}ms)",
                vanity_code,
                req,
                duration_in_ms,
            );
            std::process::exit(0);
        }
        utils::UpdateVanityURLResult::Error(utils::UpdateVanityURLErrorResponse {
            message: _,
            code,
        }) => {
            if status == 401 {
                blacklisted_tokens.push(token.to_string());
                log::error!(
                    "Token not valid and blacklisted from further requests: {} (req number: {}, duration: {}ms)",
                    token,
                    req,
                    duration_in_ms,
                );
                if tokens.len() == blacklisted_tokens.len() {
                    log::info!("No valid tokens are left.");
                    std::process::exit(0);
                }
                return false;
            }
            match code {
                10004 => {
                    log::error!(
                        "User is not in this guild. (req number: {}, duration: {}ms, token: {})",
                        req,
                        duration_in_ms,
                        token,
                    );
                    true
                }
                50035 => {
                    log::error!(
                        "Vanity is taken, continuing. (req number: {}, duration: {}ms)",
                        req,
                        duration_in_ms
                    );
                    true
                }
                _ => {
                    log::error!(
                        "Unknown error. (req number: {}, duration: {}ms, status: {}, code: {})",
                        req,
                        duration_in_ms,
                        status,
                        code,
                    );
                    true
                }
            }
        }
    }
}
