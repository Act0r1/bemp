pub mod struct_of_resp;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use std::collections::HashMap;


#[allow(unused_imports)]
pub use struct_of_resp::{get_quote, Config};

#[allow(dead_code)]
type Token = (String, String);
#[allow(dead_code)]
type CallData = String;

#[allow(dead_code)]
static QUOTES: Lazy<RwLock<HashMap<Token, Vec<CallData>>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

