use rocket::{launch, routes, get, State};
use sea_orm::DatabaseConnection;
use std::net::Ipv4Addr;
use tokio::time::{interval, Duration};
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

#[get("/world")]
pub async fn index()->String{
    "Hello".to_string()
}

#[launch]
async fn rocket() -> _ {

    dotenv().ok(); 

    tokio::spawn(executejobs());


    let port = 8000;
    print_network_info(port);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await.unwrap();

    rocket::build()
        .manage(db)
        .mount("/world", routes![index])
    
    
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