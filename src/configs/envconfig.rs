
use std::{collections::HashMap, fs};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use clap::Parser;


#[derive(Serialize, Deserialize, Debug)]
pub struct EnvConfig {
    pub NETWORK:String,
    pub MAX_PRICE_INTERVAL: u32,
    pub PORT: u16,
}

pub static ENV: Lazy<EnvConfig> = Lazy::new(|| {
    let env_path = "/home/akhil888binoy/developer/rust-price-submitter/src/env.json"; 
    let env_content = fs::read_to_string(env_path).expect("Failed to read env.json");
    println!("ENV content == {:?}", &env_content);
    serde_json::from_str(&env_content).expect("Failed to parse env.json")
});

pub static CHAINID_MAP: Lazy<HashMap<String, i64>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("arbitrum_sepolia".to_string(), 421614);
    map.insert("bitlayer_testnet".to_string(), 200810);
    map.insert("move_testnet".to_string(), 336);
    map.insert("bera_testnet".to_string(), 80084);
    map
});
