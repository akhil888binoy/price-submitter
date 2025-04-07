use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone)]
pub struct AssetPricingInfo {
    pub token_address: String,
    pub token_symbol: String,
    pub min_price: String,
    pub max_price: String,
    pub update_at: u64,
}

#[derive(Clone)]
pub struct AssetInfo {
    pub token_address: String,
    pub token_decimals: u64,
}

#[derive(Debug, Serialize)]
pub struct AssetPricingInfo2 {
    pub tokenAddress: String,
    pub tokenSymbol: String,
    pub minPrice: Option<String>,
    pub maxPrice: Option<String>,
    pub updatedAt: chrono::DateTime<Utc>,
    pub priceDecimals : f32
}

#[derive(Debug, Serialize)]
pub struct PriceCandleResponse {
    pub period: String,
}

#[derive(Debug, Serialize)]
pub struct Price24HResponse {
    pub _id : String,
    pub high: f32,
    pub low: f32,
    pub open: f32,
    pub close: f32
}

#[derive(Debug, Serialize)]
pub struct DummyData {
    pub lp: FeatureStats,
    pub migration: FeatureStats,
    pub trading: FeatureStats
}

#[derive(Debug, Serialize)]
pub struct FeatureStats {
    pub is_active: bool
}

impl Default for DummyData {
    fn default() -> Self {
        Self { 
            lp: FeatureStats { is_active: false }, 
            migration: FeatureStats { is_active: false }, 
            trading: FeatureStats { is_active: false } }
    }
}