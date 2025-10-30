pub mod struct_of_resp;

pub type Token = (String, String);
pub type CallData = Vec<String>;


#[allow(unused_imports)]
pub use struct_of_resp::{QuoteResponse, Config};

