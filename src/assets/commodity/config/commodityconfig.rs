

use once_cell::sync::Lazy;
use std::collections::HashMap;


pub static SYMBOL_TO_ID_MAPPING :Lazy<HashMap<&'static str,&'static str>>=Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("XAU", "68");
    map.insert("XAG", "69");
    map.insert("XPD", "1043108");
    map.insert("XPT", "1043107");
    map.insert("NG", "8862");
    map.insert("XBR", "8833");
    map.insert("ZW", "8917");
    map.insert("RC", "8911");
    map
});

pub static PERIOD_ID_MAPPING :Lazy<HashMap<&'static str,&'static str>>=Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("1m", "PT1M");
    map.insert("5m", "PT5M");
    map.insert("15m", "PT15M");
    map.insert("1h", "PT1H");
    map.insert("1d", "P1D");
    map
});

