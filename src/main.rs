mod req;
use anyhow::bail;
use std::time::Duration;
use tokio::time::{self, Interval};

fn check() -> Result<req::Config, anyhow::Error> {
    let read = std::fs::read_to_string("./config.toml").unwrap();
    let token: req::Config = toml::from_str(&read).expect("config.toml заполнен не правильно");
    Ok(token)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let r = check().unwrap();
    if r.sell_amounts.is_some() && r.buy_amounts.is_some() {
        bail!("Нельзя указывать одновременно sell_amounts и buy_amounts");
    }
    let base_url = r.base_url.clone();
    let sell_tokens = r.sell_tokens.clone();
    let buy_tokens = r.buy_tokens.clone();
    let sell_amounts = r.sell_amounts.map(|v| v * 10u128.pow(18));
    let buy_amounts = r.buy_amounts.map(|v| v * 10u128.pow(18));
    let taker_address = r.taker_address.clone();
    let expiry_time = r.expiry_time;

    // tokio::spawn(async move {
    println!("Check..");
    let mut interval = time::interval(Duration::from_secs(expiry_time));
    loop {
        interval.tick().await;
        req::get_quote(
            &base_url,
            &sell_tokens,
            &buy_tokens,
            sell_amounts,
            buy_amounts,
            &taker_address,
        )
        .await
        .unwrap()
    }
}
