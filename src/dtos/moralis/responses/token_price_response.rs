use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenPriceResponse {
    pub usd_price: f64,
}
