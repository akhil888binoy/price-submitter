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
