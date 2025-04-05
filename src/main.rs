use controllers::IncentivesStip::get_dummy_data;
use controllers::PriceCandles::get_price_candles;
use controllers::Prices24h::get_price24h;
use rocket::{get, launch, routes, Build, Rocket};
use sea_orm::DatabaseConnection;
use std::net::Ipv4Addr;
use std::env;
use dotenv::dotenv;
use sea_orm::*;


pub mod jobs;
pub mod utils;
pub mod configs;
pub mod assets;
pub mod data;
pub mod controllers;


use crate::jobs::index::executejobs;

pub struct DbConnection(pub DatabaseConnection);

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

async fn init_db() -> DbConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url)
        .await
        .unwrap();

    DbConnection(db)
}


#[launch]
async fn rocket() -> Rocket<Build> {

    // dotenv().ok(); 

    tokio::spawn(executejobs());

    let port = 8000;
    print_network_info(port);

    let db = init_db().await;

    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let db = Database::connect(&db_url).await.unwrap();

    rocket::build()
        .manage(db)
        .mount("/", routes![hello, get_price_candles, get_price24h, get_dummy_data])
        // .mount("/prices", routes![])
        // .mount("/candles", routes![get_price_candles])
        // .mount("/prices/24h", routes![get_price24h])
}

fn print_network_info(port: u16) {
    let local_address = format!("http://localhost:{}", port);
    println!("Server is running locally at {}", local_address);

    if let Ok(Some(ip)) = get_local_ip() {
        let network_address = format!("http://{}:{}", ip, port);
        println!("Access it on your network at {}", network_address);
    }
}

fn get_local_ip() -> std::io::Result<Option<Ipv4Addr>> {
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    match socket.local_addr()?.ip() {
        std::net::IpAddr::V4(ip) => Ok(Some(ip)),
        _ => Ok(None),
    }
}