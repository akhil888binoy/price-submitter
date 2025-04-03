use std::collections::HashMap;

use once_cell::sync::Lazy;


pub static dummyData: Lazy<HashMap<String, HashMap<String, bool>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    let mut lp_map = HashMap::new();
    lp_map.insert("isActive".to_string(), false);
    
    let mut migration_map = HashMap::new();
    migration_map.insert("isActive".to_string(), false);
    
    let mut trading_map = HashMap::new();
    trading_map.insert("isActive".to_string(), false);
    
    map.insert("lp".to_string(), lp_map);
    map.insert("migration".to_string(), migration_map);
    map.insert("trading".to_string(), trading_map);
    
    map
});
