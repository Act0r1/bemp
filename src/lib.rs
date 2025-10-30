mod req;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use req::{CallData, Config, QuoteResponse, Token};

use crate::req::struct_of_resp::get_quote;

pub struct QuoteManager {
    pub config: Config,
    pub quotes: Arc<RwLock<HashMap<Token, CallData>>>,
}

impl QuoteManager {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            quotes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start(&self) {
        let quotes = Arc::clone(&self.quotes);
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(std::time::Duration::from_secs(config.expiry_time));
            loop {
                interval.tick().await;
                let _ = get_quote(
                    &config.base_url,
                    &config.sell_tokens,
                    &config.buy_tokens,
                    config.sell_amounts,
                    config.buy_amounts,
                    &config.taker_address,
                    Arc::clone(&quotes),
                )
                .await;
            }
        });
    }
    pub async fn get_quote(&self, sell_token: &str, buy_token: &str) -> Option<CallData> {
        let cache = self.quotes.read().await;
        cache
            .get(&(sell_token.to_string(), buy_token.to_string()))
            .cloned()
    }
    pub fn check() -> Result<Config, anyhow::Error> {
        let read = std::fs::read_to_string("./config.toml").unwrap();
        let token: Config = toml::from_str(&read).expect("config.toml заполнен не правильно");
        Ok(token)
    }
}
