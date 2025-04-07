 
use crate::utils::helpersutils::{
    SUPPORTED_PERIODS,
    SUPPORTED_TOKENS,
    SYMBOL_TO_ADDRESS_MAPPING
};

use crate::configs::envconfig::{CHAINID_MAP , ENV};
use crate::DbConnection;
use rocket::{get, State};
use rocket::response::status::BadRequest;
use sea_orm::entity::prelude::*;
use sea_orm::{EntityTrait, QueryFilter};
use sea_orm::*;
use serde_json::json;
use rocket::serde::{json::Json, Deserialize};
use entities::{prelude::*, *};


#[path = "../../entity/src/mod.rs"]
mod entities;



const MAX_LIMIT : u32 = 1000;

#[derive(Deserialize)]
pub struct ParamData{
    pub period : String,
    pub token_symbol:String,
    pub limit : String
}

#[get("/candles?<period>&<token_symbol>&<limit>")] 
pub async fn get_price_candles (
    period: String,
    token_symbol: String,
    limit: Option<u32>,
    db: &State<DbConnection>
) -> Result<Json<JsonValue>, BadRequest<Json<JsonValue>>> {
    // dotenv().ok();
    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let db = match Database::connect(&db_url).await {
    //     Ok(conn) => conn,
    //     Err(_) => {
    //         return Err(BadRequest(Json(json!({"error": "DB connection failed"}))));
    //     }
    // };
    let supportedperiods = SUPPORTED_PERIODS.clone();

    if !supportedperiods.contains(&period.as_str()){
        return Err(BadRequest(Json(json!({
            "error": format!("Unsupported period. Supported: {:?}", supportedperiods)
        }))))
    }

    // let tokenSymbol = param.tokenSymbol.as_str();
    let supported_tokens = match SUPPORTED_TOKENS.get(&ENV.NETWORK){
        Some(data)=>data.clone(),
        None=> {
            return Err(BadRequest(Json(json!({
                "error": "Could not get supported tokens"
            }))));
        }
    };

    if !supported_tokens.contains(&token_symbol.as_str()){
        return Err(BadRequest(Json(json!({
            "error": format!("Unsupported token. Supported: {:?}", supported_tokens)
        }))));
    }

    let token_address = match SYMBOL_TO_ADDRESS_MAPPING.get(&token_symbol){
        Some(data)=>data,
        None=> {
            return Err(BadRequest(Json(json!({
                "error": "Token address not found"
            }))));
        }
    };

    let limit = limit.unwrap_or(MAX_LIMIT).min(MAX_LIMIT);

    // if param.limit.is_empty(){
    //     limit = if param.limit.parse::<u32>().unwrap() > MAX_LIMIT{
    //         MAX_LIMIT
    //     }else{
    //         param.limit.parse::<u32>().unwrap()
    //     };
    // }else{
    //     limit = MAX_LIMIT
    // }

    let chainid = match CHAINID_MAP.get(&ENV.NETWORK){
        Some(data)=>data,
        None=> {
            return Err(BadRequest(Json(json!({
                "error": "Could not get chain ID"
            }))));
        }
    };

    // let candlesData = await 
    let candles_data = PriceCandle::find()
      .filter(price_candle::Column::Token.eq(token_address))
      .filter(price_candle::Column::Period.eq("1d"))
      .filter(price_candle::Column::ChainId.eq(chainid.clone()))
      .order_by_desc(price_candle::Column::Timestamp)
      .limit(limit as u64)
      .all(&db.0)
      .await;

    match candles_data {
        Ok(candle) => {
                let formatted_candle = candle
                .iter()
                .map(|candle| json!([
                    candle.timestamp,
                    candle.open,
                    candle.high,
                    candle.low,
                    candle.close
                ]))
                .collect::<Vec<_>>();
                Ok(Json(json!({
                    "period": period,
                    "candles": formatted_candle,
                })))
            },
        Err(e) => Err(BadRequest(Json(json!({
            "error": format!("DB error: {:?}", e)
        }))))
    }
}
