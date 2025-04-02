

// mod db;
// mod pricecandlemodel;
// mod schema;

// #[path = "../../../utils/helpersutils.rs"]
// mod helpersutils;

// #[path = "../config/bondconfig.rs"]
// mod bondconfig;


// use diesel::prelude::*;
// use db::establish_connection;
// use pricecandlemodel::PriceCandle;
// use bondconfig::{BONDS_PERIOD_ID_MAPPING ,BONDS_SYMBOL_TO_ID_MAPPING };



// async fn main(){
//     let connection = &mut establish_connection();
//     println!("Connected to the database!");
//     for (asset , assetvalue ) in BONDS_SYMBOL_TO_ID_MAPPING{
//         for (period , periodvalue) in BONDS_PERIOD_ID_MAPPING{
//             println!("trying {:?}" , assetvalue);
//             let  response = match investing(assetvalue.to_string() , "P5Y" , periodvalue.to_string()).await {
//                 Ok(data)=>data,
//                 Err(e)=>{
//                     eprintln!("Error to fetch investing : {}",e);
//                 }
//             };



//         }
//     }

// }

