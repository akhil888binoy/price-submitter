
use once_cell::sync::Lazy;
use std::collections::HashMap;


pub static BONDS_SYMBOL_TO_ID_MAPPING :Lazy<HashMap<&'static str,&'static str>>=Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("US1YTX", "23700");
    map.insert("US5YTX", "23703");
    map.insert("AU5YTRR", "23873");
    map.insert("GB5YTRR", "23668");
    map.insert("IN5YTRR", "24009");
    map
});


pub static BONDS_PERIOD_ID_MAPPING :Lazy<HashMap<&'static str,&'static str>>=Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("1m", "PT1M");
    map.insert("5m", "PT5M");
    map.insert("15m", "PT15M");
    map.insert("1h", "PT1H");
    map.insert("1d", "P1D");
    map
});

