#[path = "./pricesubmitter.rs"]
mod pricesubmitter;

use pricesubmitter::submit_prices;
use sea_orm::*;

const DATABASE_URL: &str = "postgresql://neondb_owner:npg_4SDGAJv9YWeu@ep-calm-queen-a5cn21aq-pooler.us-east-2.aws.neon.tech/neondb?sslmode=require";
const DB_NAME: &str = "neondb";


pub async fn executejobs(){
    let db = Database::connect(DATABASE_URL).await.unwrap();
    submit_prices(&db).await;
}
