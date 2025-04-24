use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::Value;
use std::env;

pub async fn handle() -> Result<f64> {
    let client = Client::new();
    let moralis_key = env::var("MORALIS_API_KEY")?;
    let url = "https://solana-gateway.moralis.io/token/mainnet/So11111111111111111111111111111111111111112/price";

    let response = client
        .get(url)
        .header("accept", "application/json")
        .header("X-API-Key", moralis_key)
        .send()
        .await?;

    let body: Value = response.json().await?;
    let price = body["usdPrice"]
        .as_f64()
        .ok_or_else(|| anyhow!("Failed to parse usdPrice"))?;

    Ok(price)
}
