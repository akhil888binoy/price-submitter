use std::collections::HashMap;

use entities::{prelude::PriceCandle, price_candle::{self, Model}};
use rocket::{get, response::status::BadRequest, serde::json::Json, State};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde_json::json;
use sea_orm::JsonValue;
use crate::{configs::envconfig::{CHAINID_MAP, ENV}, utils::{helpersutils::{SUPPORTED_TOKENS, SYMBOL_TO_ADDRESS_MAPPING}, interfaceutils::Price24HResponse}, DbConnection};

#[path = "../../entity/src/mod.rs"]
mod entities;

#[get("/prices/candles")]
pub async fn get_price24h (
    db: &State<DbConnection>
) -> Result<
        Json<Vec<Price24HResponse>>,
        BadRequest<Json<JsonValue>>
    >{

    let chain_id = match CHAINID_MAP.get(&ENV.NETWORK) {
        Some(id) => id,
        None => {
            return Err(BadRequest(Json(json!({
                "error": "Could not get chain ID"
            }))))
        }
    };

    let supported_tokens = match SUPPORTED_TOKENS.get(&ENV.NETWORK) {
        Some(token) => token.clone(),
        None => {
            return Err(BadRequest(Json(json!(
                {
                    "error": "Could not get supported tokens"
                }
            ))))
        }
    };

    let token_addresses = supported_tokens
        .iter()
        .map(|token_symbol|
            SYMBOL_TO_ADDRESS_MAPPING.get(token_symbol.to_owned()).expect("Token not found").to_lowercase()
        )
        .collect::<Vec<_>>();

    // let token_addresses1 = ["0x67f17cca1337C4bFaa844139f934908ecf984422", "0xCbD0bcE5FE20f5683C84CbA45E645e6F2ffea5cC", "0x5D7Ee3e6a2465780B2a6cB0970E0Ca85D1063530", "0x6aa8143ADa3A3A1273da05742Ecf5DCcC911900D"];

    let raw_data = PriceCandle::find()
        .filter(price_candle::Column::Token.is_in(token_addresses.clone()))
        .filter(price_candle::Column::Period.eq("1d"))
        .filter(price_candle::Column::ChainId.eq(chain_id.clone()))
        .order_by_desc(price_candle::Column::Timestamp)
        .all(&db.0)
        .await
        .map_err(|e| BadRequest(Json(json!({"error": format!("Error getting data24hr: {:?}", e)}))))?;
    
    let mut grouped_data: HashMap<String, Model> = HashMap::new();
    
    for candle in raw_data {
        grouped_data.entry(candle.token.clone()).or_insert(candle);
    }

    let data24H = grouped_data
        .into_iter()
        .map(|(token, candle)| {
            let token_symbol = supported_tokens[token_addresses.iter().position(|x| x == &token).unwrap()];
            Price24HResponse {
                _id: token_symbol.to_string(),
                high: candle.high,
                low: candle.low,
                open: candle.open,
                close: candle.close,
            }
        })
        .collect::<Vec<Price24HResponse>>();
    
    Ok(Json(data24H))
}