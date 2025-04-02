

use crate::jobs::pricesubmitter::submit_prices;
use sea_orm::*;
use dotenv::dotenv;
use std::env;


pub async fn executejobs(){
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await.unwrap();
    submit_prices(&db).await;
}
