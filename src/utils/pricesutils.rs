#[path = "./interfaceutils.rs"]
mod interfaceutils;

#[path = "./responseinterfaceutils.rs"]
mod responseinterfaceutils;

#[path = "../configs/envconfig.rs"]
mod envconfig;
#[path = "./helpersutils.rs"]
mod helpersutils;

use helpersutils::{
    // BINANCE_KEYS,
    // BINANCE_SYMBOL_MAP,
    // KUCOIN_KEYS,
    // KUCOIN_SYMBOL_MAP,
    // MEXC_KEYS,
    // MEXC_SYMBOL_MAP,
    // GATE_KEYS,
    // GATE_SYMBOL_MAP,
    PRICES_MAPPINGS,
    PYTH_ID,
    // OKX_KEYS,
    // OKX_SYMBOL_MAP,
    // KRAKEN_KEYS,
    // KRAKEN_SYMBOL_MAP,
    // BYBIT_KEYS,
    // BYBIT_SYMBOL_MAP,
    PYTH_ID_TO_TOKEN_MAPPING,
    SUPPORTED_TOKENS,
    SYMBOL_TO_ADDRESS_MAPPING,
    SYMBOL_TO_DECIMAL_MAPPING,
};
use interfaceutils::AssetPricingInfo;
use responseinterfaceutils::{ParclDetails, ParclIdResponse, ParclResponse, PythResponse};
use std::collections::HashMap;
extern crate rand;
use envconfig::ENV;
use num_bigint::BigInt;
use num_traits::pow;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_pyth_price_url() -> String {
    let mut pyth_url = String::from("https://hermes.pyth.network/v2/updates/price/latest?");
    let pyth_ids = match PYTH_ID.get(&ENV.NETWORK) {
        Some(ids) => ids,
        None => return pyth_url,
    };

    for (i, id) in pyth_ids.iter().enumerate() {
        if i > 0 {
            pyth_url.push('&');
        }
        pyth_url.push_str(&format!("ids[]={}", id.to_string()));
    }

    pyth_url
}

pub async fn get_pyth_prices() -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut result: HashMap<String, f64> = HashMap::new();
    
    let pyth_id_to_token_mapping = match PYTH_ID_TO_TOKEN_MAPPING.get(&ENV.NETWORK) {
        Some(ids) => ids,
        None => panic!("No mapping found for the given network"),
    };

    let client = reqwest::Client::new();
    let response = client.get(get_pyth_price_url()).send().await?;

    if response.status() != reqwest::StatusCode::OK {
        eprintln!(
            "Failed to retrieve data. Status code: {}",
            response.status()
        );
        return Err("Failed to retrieve data".into());
    }

    let response_data: PythResponse = response.json().await?;

    for price_data in response_data.parsed.iter() {
        if let Some(token) = pyth_id_to_token_mapping.get(&*price_data.id) {
            let adjusted_price = (price_data.price.price.parse::<f64>().unwrap())
                * (10f64).powi(price_data.price.expo);
            result.insert(token.to_string(), adjusted_price);

            if token.to_string() == "BTC" {
                result.insert(
                    "WBTC".to_string(),
                    price_data.price.price.parse::<f64>().unwrap() / 10f64.powi(8),
                );
            }
            if token.to_string() == "ETH" {
                result.insert(
                    "WETH".to_string(),
                    price_data.price.price.parse::<f64>().unwrap() / 10f64.powi(8),
                );
            }
        }
    }

    Ok(result)
}

pub async fn fetch_all_parcl_ids() -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let url = "https://parcl-api.com/v1/metadata/parcl-ids";
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .header("Origin", "https://app.parcl.co")
        .header("Referer", "https://app.parcl.co/")
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        eprintln!(
            "Failed to retrieve data. Status code: {}",
            response.status()
        );
        return Err("Failed to retrieve data".into());
    }

    let response_data: ParclIdResponse = response.json().await?;
    Ok(response_data.ids)
}

#[derive(serde::Serialize)]
pub struct Param {
    pub id: String,
}

pub async fn fetch_parcl_details(
    parcl_ids: Vec<String>,
) -> Result<HashMap<String, ParclDetails>, Box<dyn std::error::Error>> {
    let url = "https://parcl-api.com/v1/real-estate-data/parcl-info";
    let client = reqwest::Client::new();
    let mut parcl_map: HashMap<String, ParclDetails> = HashMap::new();

    for parcl_id in parcl_ids {
        let params = Param {
            id: parcl_id.clone(),
        };

        let response = client
            .get(url)
            .query(&params)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
            .header("Origin", "https://app.parcl.co")
            .header("Referer", "https://app.parcl.co/")
            .send()
            .await?;

        if response.status() != reqwest::StatusCode::OK {
            eprintln!(
                "Failed to retrieve data for Parcel ID {}. Status code: {}",
                parcl_id,
                response.status()
            );
            continue;
        }

        let response_data: ParclResponse = response.json().await?;
        let parcl_details = ParclDetails {
            parcl_id: parcl_id.clone(),
            name: response_data.info.name,
            current_price: response_data.info.current_price.to_string(),
        };

        parcl_map.insert(parcl_id, parcl_details);
    }

    Ok(parcl_map)
}

pub async fn gathertokenprices() -> Result<HashMap<String, Vec<f64>>, Box<dyn std::error::Error>> {
    let responses = match get_pyth_prices().await {
        Ok(data) => data,
        Err(e) => {
            panic!("Error getting Pyth prices: {}", e);
        }
    };

    let mut prices: HashMap<String, Vec<f64>> = match PRICES_MAPPINGS.get(ENV.NETWORK.as_str()) {
        Some(map) => map
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect(), // Convert keys to `String`
        None => {
            eprintln!("No price mappings found for network: {}", ENV.NETWORK);
            return Err("Network not found".into());
        }
    };

    const BITLAYER_NOT_SUPPORTED_BLUE_CHIPS: [&str; 1] = ["ETH"];

    for (key, value) in responses.iter() {
        let key_str = key.to_string(); // Convert borrowed key to owned `String`

        if ENV.NETWORK == "bitlayer_testnet" {
            if !BITLAYER_NOT_SUPPORTED_BLUE_CHIPS.contains(&key_str.as_str()) {
                if let Some(vec) = prices.get_mut(&key_str) {
                    vec.push(*value);
                }
            }
        } else {
            if let Some(vec) = prices.get_mut(&key_str) {
                vec.push(*value);
            }
        }
    }

    Ok(prices)
}

pub async fn get_token_prices() -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let prices = match gathertokenprices().await {
        Ok(data) => data,
        Err(e) => {
            panic!("Error getting Pyth prices: {}", e);
        }
    };

    let mut rng = rand::rng();
    let mut result: HashMap<String, f64> = HashMap::new();

    for (key, token_prices) in prices.iter() {
        let mut token_prices = token_prices.clone();

        if token_prices.len() > 2 {
            token_prices.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            token_prices.pop();
            token_prices.remove(0);
        }

        let mut price_sum: f64 = 0.0;
        let mut weight_sum: u32 = 0;

        for price in &token_prices {
            let weight = rng.random_range(10..20);
            weight_sum += weight;
            price_sum += price * weight as f64;
        }

        if price_sum == 0.0 {
            println!("Token skipped: {}", key);
            continue;
        }

        result.insert(key.clone(), price_sum / weight_sum as f64);
    }

    Ok(result)
}


pub async fn get_token_prices_filtered() -> Result<Vec<AssetPricingInfo>, Box<dyn std::error::Error>>
{
    let token_prices = match get_token_prices().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error getting token prices: {}", e);
            return Err(e.into());
        }
    };

    let mut token_prices_array: Vec<AssetPricingInfo> = Vec::new();

    for (asset, value) in token_prices {
        if !value.is_nan() {
            let precision = 10;
            let asset_price = (value * 10f64.powi(precision as i32)) as i64;
            let decimals = match SYMBOL_TO_DECIMAL_MAPPING.get(&asset) {
                Some(d) => *d,
                None => {
                    eprintln!("No decimals found for asset: {}", asset);
                    continue;
                }
            };

            // Ensure decimals >= precision to avoid negative exponents
            if decimals < precision {
                eprintln!(
                    "Decimals ({}) cannot be less than precision ({}) for asset: {}",
                    decimals, precision, asset
                );
                continue;
            }
            let subdecprec = decimals - precision;
            let token_price =
                BigInt::from(asset_price) * pow(BigInt::from(10), subdecprec as usize);

            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let token_prices_filtered = AssetPricingInfo {
                token_address: SYMBOL_TO_ADDRESS_MAPPING
                    .get(&asset)
                    .unwrap_or(&"".to_string())
                    .to_string(),
                token_symbol: asset,
                min_price: token_price.to_string(),
                max_price: token_price.to_string(),
                update_at: current_time,
            };

            token_prices_array.push(token_prices_filtered);
        }
    }

    Ok(token_prices_array)
}
