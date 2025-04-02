
#[path = "../utils/pricesutils.rs"]
mod pricesutils;

#[path = "../utils/helpersutils.rs"]
mod helpersutils;

#[path = "../assets/commodity/config/commodityconfig.rs"]
mod commodityconfig;

#[path = "../assets/bonds/config/bondconfig.rs"]
mod bondconfig;

#[path = "../configs/envconfig.rs"]
mod envconfig;

#[path = "../../entity/src/mod.rs"]
mod entities;

use commodityconfig::{PERIOD_ID_MAPPING, SYMBOL_TO_ID_MAPPING};
use envconfig::{CHAINID_MAP, ENV};
use helpersutils::{
    PERIOD_MAP, PRICE_FETCH_INTERVAL, SYMBOL_TO_ADDRESS_MAPPING, TOKENS_MAPPINGS, sleep_ms,
};
use pricesutils::get_token_prices;
use entities::{prelude::*, *};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};


pub async fn store_price_candle(
    db: &DatabaseConnection,
    timestamp_val: u64,
    token_val: String,
    price: f64,
    period_val: String,
    interval: u64,
) -> Result<(), DbErr> {
    let current_timestamp = (timestamp_val / interval) * interval;
    let icurrent_timestamp = current_timestamp as i64;

    let network_chain_id = CHAINID_MAP
        .get(&ENV.NETWORK)
        .expect("Expected CHAINID_MAP key");

    // Check if the candle exists
    let existing_candle = PriceCandle::find()
        .filter(price_candle::Column::ChainId.eq(*network_chain_id))
        .filter(price_candle::Column::Token.eq(&token_val))
        .filter(price_candle::Column::Timestamp.eq(icurrent_timestamp))
        .filter(price_candle::Column::Period.eq(&period_val))
        .one(db)
        .await?;

    match existing_candle {
        Some(candle) => {
            let mut candle: price_candle::ActiveModel = candle.into();
            candle.close = Set(price as f32);
            candle.high = Set(f32::max(candle.high.unwrap(), price as f32));
            candle.low = Set(f32::min(candle.low.unwrap(), price as f32));

            candle.update(db).await?;
        }
        None => {
            let last_timestamp = icurrent_timestamp - interval as i64;
            let mut open_val = price;

            // Fetch the last candle
            let last_candle = PriceCandle::find()
                .filter(price_candle::Column::ChainId.eq(*network_chain_id))
                .filter(price_candle::Column::Token.eq(&token_val))
                .filter(price_candle::Column::Timestamp.eq(last_timestamp))
                .filter(price_candle::Column::Period.eq(&period_val))
                .one(db)
                .await?;

            if let Some(last_candle) = last_candle {
                open_val = (price + last_candle.close as f64) / 2.0;

                // Update the last candle's close value
                let mut last_candle: price_candle::ActiveModel = last_candle.into();
                last_candle.close = Set(open_val as f32);
                last_candle.update(db).await?;
            }

            let high_val = f64::max(open_val, price);
            let low_val = f64::min(open_val, price);

            // Create a new candle
            let new_candle = price_candle::ActiveModel {
                token: Set(token_val),
                open: Set(open_val as f32),
                high: Set(high_val as f32),
                low: Set(low_val as f32),
                close: Set(price as f32),
                timestamp: Set(icurrent_timestamp),
                period: Set(period_val),
                chain_id: Set(*network_chain_id),
                ..Default::default()
            };

            new_candle.insert(db).await?;
        }
    }

    Ok(())
}

pub async fn submit_new_prices_to_db(db: &DatabaseConnection, prices: HashMap<String, f64>) -> Result<(), DbErr> {
    let tokens = TOKENS_MAPPINGS
        .get(&ENV.NETWORK)
        .expect("Cannot get tokens from token mappings");

    let timestamp_val = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    for token_val in tokens.iter() {
        if let Some(price) = prices.get(&token_val.key) {
            for (period_key, period_val) in PERIOD_MAP.iter() {
                store_price_candle(
                    db,
                    timestamp_val,
                    token_val.address.to_string(),
                    *price,
                    period_key.to_string(),
                    *period_val,
                )
                .await?;
            }
        }
    }

    Ok(())
}

pub async fn submit_prices(db: &DatabaseConnection) -> Result<(), DbErr> {
    loop {
        let prices = match get_token_prices().await {
            Ok(prices) => prices,
            Err(e) => {
                eprintln!("Error getting Pyth prices: {}", e);
                continue;
            }
        };
        submit_new_prices_to_db(db, prices).await?;
        println!("Submitting price to db...");
    }
}

