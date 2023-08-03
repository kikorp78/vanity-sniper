use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, StatusCode,
};
use serde::Deserialize;
use serde_json::json;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Deserialize)]
pub struct UpdateVanityURLSuccessResponse {
    pub code: String,
    pub uses: u16,
}

#[derive(Deserialize)]
pub struct UpdateVanityURLErrorResponse {
    pub message: String,
    pub code: u16,
}

pub enum UpdateVanityURLResult {
    Success(UpdateVanityURLSuccessResponse),
    Error(UpdateVanityURLErrorResponse),
}

pub async fn update_vanity_url(
    client: &Client,
    guild_id: &str,
    code: &str,
    token: &str,
) -> Result<(UpdateVanityURLResult, StatusCode, u128), reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(AUTHORIZATION, HeaderValue::from_str(token).unwrap());

    let body = json!({
        "code": code
    });

    let start_time = Instant::now();

    let response = client
        .patch(format!(
            "https://discord.com/api/v10/guilds/{}/vanity-url",
            guild_id
        ))
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    let end_time = Instant::now();

    let duration_in_ms = end_time.duration_since(start_time).as_millis();

    let status = response.status().clone();

    let body = response.text().await.unwrap();
    let parsed = if status.is_success() {
        UpdateVanityURLResult::Success(serde_json::from_str(&body).unwrap())
    } else {
        UpdateVanityURLResult::Error(serde_json::from_str(&body).unwrap())
    };

    Ok((parsed, status, duration_in_ms))
}

pub fn tokens_file_to_array() -> Result<Vec<String>, std::io::Error> {
    let file = File::open("tokens.txt")?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
        .collect();

    Ok(lines)
}
