use super::QUOTES;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub sell_tokens: String,
    pub buy_tokens: String,
    pub sell_amounts: Option<u128>,
    pub buy_amounts: Option<u128>,
    pub taker_address: String,
    pub base_url: String,
    pub expiry_time: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GasFee {
    pub native: String,
    pub usd: f64,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    pub amount: String,
    pub decimals: u32,
    pub priceUsd: Option<f64>,
    pub symbol: String,
    pub price: Option<f64>,
    pub priceBeforeFee: Option<f64>,
    pub minimumAmount: Option<String>,
    pub amountBeforeFee: Option<String>,
    pub deltaFromExpected: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct QuoteToSign {
    pub partner_id: Option<u64>,
    pub expiry: u64,
    pub taker_address: String,
    pub maker_address: String,
    pub maker_nonce: String,
    pub taker_token: String,
    pub maker_token: String,
    pub taker_amount: String,
    pub maker_amount: String,
    pub receiver: String,
    pub packed_commands: String,
}
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Tx {
    to: String,
    value: String,
    data: String,
    from: String,
    gas: i32,
    gasPrice: i64,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct QuoteResponse {
    pub requestId: Option<String>,
    #[serde(rename = "type")]
    pub quote_type: Option<String>,
    pub status: String,
    pub quoteId: String,
    pub chainId: u64,
    pub approvalType: Option<String>,
    pub nativeToken: Option<String>,
    pub taker: Option<String>,
    pub receiver: Option<String>,
    pub expiry: u64,
    pub slippage: Option<f64>,
    pub gasFee: GasFee,
    pub buyTokens: HashMap<String, TokenInfo>,
    pub sellTokens: HashMap<String, TokenInfo>,
    pub settlementAddress: Option<String>,
    pub approvalTarget: Option<String>,
    pub requiredSignatures: Vec<String>,
    pub priceImpact: Option<f64>,
    pub partnerFee: Option<HashMap<String, String>>,
    pub warnings: Vec<String>,
    pub info: Option<String>,
    pub makers: Option<Vec<String>>,
    pub toSign: QuoteToSign,
    pub onchainOrderType: String,
    pub tx: Tx,
}

pub async fn get_quote(
    base_url: &str,
    sell_tokens: &str,
    buy_tokens: &str,
    sell_amounts: Option<u128>,
    buy_amounts: Option<u128>,
    taker_address: &str,
) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let mut params = HashMap::new();

    params.insert("sell_tokens", sell_tokens.to_string());
    params.insert("buy_tokens", buy_tokens.to_string());
    params.insert("taker_address", taker_address.to_string());
    params.insert("gasless", "false".to_string());
    params.insert("skip_validation", "true".to_string());

    if let Some(s) = buy_amounts {
        params.insert("buy_amounts", s.to_string());
    }
    if let Some(s) = sell_amounts {
        params.insert("sell_amounts", s.to_string());
    }

    let res = client
        .get(format!("{}/v3/quote", base_url))
        .query(&params)
        .send()
        .await?;

    if !res.status().is_success() {
        let txt = res.text().await?;
        panic!("Error response: {}", txt);
    }

    let parsed: QuoteResponse = res.json().await?;
    // println!("{:?}", parsed);
    {
        let mut cache = QUOTES.write().await;
        cache
            .entry((buy_tokens.to_string(), sell_tokens.to_string()))
            .or_default()
            .push(parsed.tx.data.clone());
    }
    println!("QUOTES: {:?}", QUOTES.read().await);
    Ok(())

    // Ok(parsed)
}
