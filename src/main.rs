
pub mod jobs;
pub mod utils;
pub mod configs;
pub mod assets;
pub mod data;
pub mod controllers;

use dotenv::dotenv;
use std::env;
use sea_orm::*;


use crate::jobs::index::executejobs;


#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file
    
    
    executejobs().await;
}