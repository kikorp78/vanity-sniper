use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, StatusCode,
};
use serde_json::json;

pub async fn update_vanity_url(
    client: &Client,
    guild_id: &str,
    code: &str,
    token: &str,
) -> Result<StatusCode, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert(AUTHORIZATION, HeaderValue::from_str(token).unwrap());

    let body = json!({
        "code": code
    });

    let response = client
        .patch(format!(
            "https://discord.com/api/v10/guilds/{}/vanity-url",
            guild_id
        ))
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    Ok(response.status())
}

pub fn tokens_file_to_array() -> Result<Vec<String>, std::io::Error> {
    let file = File::open("tokens.txt")?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.is_empty())
        .collect();

    Ok(lines)
}
