 
use crate::data::dummydata::dummyData;
use crate::utils::helpersutils::{
    SUPPORTED_PERIODS,
    SUPPORTED_TOKENS,
    SYMBOL_TO_ADDRESS_MAPPING
};

use crate::configs::envconfig::{CHAINID_MAP , ENV};
use crate::utils::pricesutils::getTokenPricesFiltered;
use rocket::post;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm::*;
use dotenv::dotenv;
use std::env;
use crate::utils::interfaceutils::AssetPricingInfo2;
use rocket::serde::{json::Json, Deserialize};
use entities::{prelude::*, *};


#[path = "../../entity/src/mod.rs"]
mod entities;



const MAX_LIMIT : u32 = 1000;

#[derive(Deserialize)]
pub struct ParamData{
    pub period : String,
    pub tokenSymbol:String,
    pub limit : String
}

#[post("/candles", data="<param>")] 
pub async fn getPriceCandles (param : Json<ParamData>) {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await.unwrap();
    let period = param.period.as_str();
    let supportedperiods = SUPPORTED_PERIODS.clone();

    if !supportedperiods.contains(&period){
        //return 400 status with message
    }



    let tokenSymbol = param.tokenSymbol.as_str();
    let supportedTokens = match SUPPORTED_TOKENS.get(&ENV.NETWORK){
        Some(data)=>data.clone(),
        None=>panic!("Error : Cannot get supported tokens ")
    };
    if supportedTokens.contains(&tokenSymbol){
                //return 400 status with message
    }

    let tokenAddress = match SYMBOL_TO_ADDRESS_MAPPING.get(tokenSymbol){
        Some(data)=>data,
        None=> panic!("Error: Cannot get address")
    };

    let mut limit: u32 = 0;

    if param.limit.is_empty(){
        limit = if param.limit.parse::<u32>().unwrap() > MAX_LIMIT{
            MAX_LIMIT
        }else{
            param.limit.parse::<u32>().unwrap()
        };
    }else{
        limit = MAX_LIMIT
    }

    



}
