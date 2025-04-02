use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PythResponse {
    pub binary: Binary,
    pub parsed: Vec<Parsed>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Binary {
    pub encoding: String,
    pub data: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parsed {
    pub id: String,
    pub price: Price,
    pub ema_price: EmaPrice,
    pub metadata: MetaData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub price: String,
    pub conf: String,
    pub expo: i32,
    pub publish_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmaPrice {
    pub price: String,
    pub conf: String,
    pub expo: i64,
    pub publish_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub slot: u64,
    pub proof_available_time: u64,
    pub prev_publish_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParclIdResponse {
    pub ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParclDetails {
    pub parcl_id: String,
    pub name: String,
    pub current_price: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParclResponse {
    pub info: Info,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub parcl_id: i64,
    pub name: String,
    pub current_price: f64,
    pub price_history: Vec<PriceHistory>,
    pub currency: String,
    pub metric: String,
    pub price_feed_stats: PriceFeedStats,
    pub market: String,
    pub total_area: f64,
    pub total_pop: i64,
    pub median_age: i64,
    pub median_income: i64,
    pub state: String,
    pub financials: Financials,
    pub current_inventory: CurrentInventory,
    pub absorption_rate_history: Option<String>,
    pub sales_history: Vec<SalesHistory>,
    pub listings_history: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceHistory {
    pub price: f64,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceFeedStats {
    pub pct_change_1_day: f64,
    pub dollar_change_1_day: f64,
    pub pct_change_7_day: f64,
    pub dollar_change_7_day: f64,
    pub pct_change_30_day: f64,
    pub dollar_change_30_day: f64,
    pub pct_change_60_day: f64,
    pub dollar_change_60_day: f64,
    pub pct_change_90_day: f64,
    pub dollar_change_90_day: f64,
    pub pct_change_180_day: f64,
    pub dollar_change_180_day: f64,
    pub pct_change_365_day: f64,
    pub dollar_change_365_day: f64,
    pub high_52_wk: f64,
    pub low_52_wk: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Financials {
    pub sharpe_ratio: f64,
    pub annual_volatility: f64,
    pub beta: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentInventory {
    pub condo: i64,
    pub townhouse: i64,
    pub total_units: i64,
    pub single_family: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SalesHistory {
    pub date: String,
    pub sales: i64,
}
