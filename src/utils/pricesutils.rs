use crate::utils::helpersutils::{
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

#[path = "../../entity/src/mod.rs"]
mod entities;

use crate::assets::commodity::config::commodityconfig::{PERIOD_ID_MAPPING, SYMBOL_TO_ID_MAPPING};
use crate::configs::envconfig::{CHAINID_MAP, ENV};
use crate::utils::helpersutils::{
    PERIOD_MAP, PRICE_FETCH_INTERVAL, TOKENS_MAPPINGS, sleep_ms,  
};
use chrono::Utc;
use entities::{prelude::*, *};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder, Set};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::interfaceutils::AssetPricingInfo;
use crate::utils::responseinterfaceutils::{ParclDetails, ParclIdResponse, ParclResponse, PythResponse};
extern crate rand;
use num_bigint::BigInt;
use num_traits::pow;
use rand::Rng;

use super::interfaceutils::AssetPricingInfo2;



#[derive(Debug)]
struct TokenPrice {
    token: String,  // Token address
    close: Decimal, // Latest close price
}


const PRICE_DECIMALS : usize = 4;
const PRECISION : u32 = 10;


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

pub async fn gettokenpricesfromdb( db: &DatabaseConnection)-> Result<HashMap<&str, f32>, DbErr >{
    let mut result = HashMap::new();
    let mut supportedfinaltokens = Vec::new();
    let supportedtokens = match SUPPORTED_TOKENS.get(&ENV.NETWORK){
        Some(data)=> data.clone(),
        None=>panic!("Error : Cannot get tokens")
    };
    
    let supportedrealestatetokens = vec![
    "CLT", "DEN", "MIA", "TPA", "MIAB", 
    "NYC", "LAX", "SAN", "SOLB", "SFO", 
    "LAS", "PIT", "PHL", "AUS", "DFW", 
    "IAH", "ATL", "SEA", "PHX", "CHI", 
    "BOS", "PDX", "WDC", "BKN", "USA", 
    "PARIS", "LCY", "CHIR", "DENR", "USDR"
    ];    

    
    for token in supportedtokens{
        if !supportedrealestatetokens.contains(&token){
            supportedfinaltokens.push(token);
        }
    }
    let mut tokenAddresses = Vec::new();

    for tokenSymbol in supportedfinaltokens.clone(){
        let tokenaddress = match SYMBOL_TO_ADDRESS_MAPPING.get(tokenSymbol){
            Some(data)=>data,
            None=>panic!("Error : Cannot get address")
        };
        tokenAddresses.push(tokenaddress.to_string().to_lowercase());
    }

    let mut realEstateTokenAddress = Vec::new();

    if ENV.NETWORK== "bitlayer_testnet".to_string() {
        for tokenSymbol in supportedrealestatetokens.clone(){
            let realestatetoken = match SYMBOL_TO_ADDRESS_MAPPING.get(tokenSymbol){
                Some(data)=>data,
                None=>panic!("Error : Cannot get token symbol")
            };
            realEstateTokenAddress.push(realestatetoken.to_string().to_lowercase());
        }
    }
    let chainid = match CHAINID_MAP.get(&ENV.NETWORK){
        Some(data)=>data,
        None=>panic!("Error : Cannot get chainid")
    };

      // Get regular token prices (1m period)
      let tokens_data = PriceCandle::find()
      .filter(price_candle::Column::Token.is_in(tokenAddresses.clone()))
      .filter(price_candle::Column::Period.eq("1m"))
      .filter(price_candle::Column::ChainId.eq(chainid.clone()))
      .order_by_desc(price_candle::Column::Timestamp)
      .all(db)
      .await;

  // Group by token and get latest close price
  let mut grouped_tokens = HashMap::new();

    let tokensData = match tokens_data{
        Ok(data)=>data,
        Err(e)=>panic!("Error : Cannot get data from DB")
    };
    
  for candle in tokensData {
        grouped_tokens.entry(candle.token.clone()) // Use token as key
        .or_insert(candle.close); // Store just the close price
}

  // Map to token symbols
  for (token_addr, close) in grouped_tokens {
      if let Some(index) = tokenAddresses.iter().position(|x| x == &token_addr) {
          if let Some(token_symbol) = supportedfinaltokens.get(index) {
              result.insert(token_symbol.clone(), close);
          }
      }
  }


  // Get real estate token prices (1d period)
  let realestatedata = PriceCandle::find()
      .filter(price_candle::Column::Token.is_in(realEstateTokenAddress.clone()))
      .filter(price_candle::Column::Period.eq("1d"))
      .filter(price_candle::Column::ChainId.eq(chainid.clone()))
      .order_by_desc(price_candle::Column::Timestamp)
      .all(db)
      .await;

let real_estate_data = match realestatedata{
    Ok(data)=>data,
    Err(e)=>panic!("Error: Cannot get data form DB")
};

  // Group by token and get latest close price
  let mut grouped_real_estate = HashMap::new();

  for candle in real_estate_data {
      grouped_real_estate.entry(candle.token.clone())
          .or_insert(candle.close);
  }



  // Map to token symbols
  for (token_addr, close) in grouped_real_estate {
      if let Some(index) = realEstateTokenAddress.iter().position(|x| x == &token_addr) {
          if let Some(token_symbol) = supportedrealestatetokens.get(index) {
              result.insert(token_symbol, close);
          }
      }
  }

  Ok(result) 


}

pub async fn calculatePriceDecimals(price : f32)-> Option<usize> {
    if price > 1.0 {
        return Some(PRICE_DECIMALS);
    }else{

        let priceString = price.to_string();
        let trailingZeroes = 0;

    if let Some(startingIndexExponential) =  priceString.find("e"){

        let exponent_char = priceString.chars().nth(startingIndexExponential + 2)?;
        
        let exponent_digit = exponent_char.to_digit(10)? as usize;
        
        Some(exponent_digit - 1 + PRICE_DECIMALS)
    }

    else if  let Some(startingIndex ) =  priceString.find("."){

        let mut trailing_zeroes = 0;
        for c in priceString[startingIndex + 1..].chars() {
            match c {
                '0' => trailing_zeroes += 1,
                _ => break,
            }
        }
    
        Some(trailing_zeroes + PRICE_DECIMALS)
    }
    else {
        return Some(0);
    }
    
    }

}

pub async fn  getTokenPricesFiltered(db: &DatabaseConnection){
    let tokenPrices =  match gettokenpricesfromdb(db).await{
        Ok(data)=>data,
        Err(e)=>panic!("Error : Cannot get token prices form DB")
    }; 

    let mut tokenPricesArray = Vec::new();

    let timestamp = Utc::now();
    for (token , price) in tokenPrices{
        let tokenPricesFiltered = AssetPricingInfo2{
        tokenAddress: SYMBOL_TO_ADDRESS_MAPPING.get(token).unwrap().to_string(),
        tokenSymbol: token.to_string(),
        minPrice: None,
        maxPrice: None,
        updatedAt: timestamp,
        priceDecimals: calculatePriceDecimals(price).await.unwrap() as f32,
        };

         let assetPrice = price ;
        
            let scaled_price = assetPrice * 10_f64.powi(PRECISION as i32);
            if scaled_price.is_finite() {
                scaled_price as u64
            } else {
                0
            }

        
    }


}