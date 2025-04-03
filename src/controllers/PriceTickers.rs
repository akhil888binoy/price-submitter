use crate::data::dummydata::dummyData;
use crate::utils::helpersutils::{
    SUPPORTED_PERIODS,
    SUPPORTED_TOKENS,
    SYMBOL_TO_ADDRESS_MAPPING
};

use crate::configs::envconfig::{CHAINID_MAP , ENV};
use crate::utils::pricesutils::getTokenPricesFiltered;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm::*;
use dotenv::dotenv;
use std::env;
use crate::utils::interfaceutils::AssetPricingInfo2;
use rocket::{get, serde::json::Json};



const MAX_LIMIT : u32 = 1000;

#[get("/tickers")] 
pub async fn getPriceTickers()-> Json<Vec<AssetPricingInfo2>>{
    
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await.unwrap();
    let tickersData = getTokenPricesFiltered(&db).await;

    Json(tickersData)
}
