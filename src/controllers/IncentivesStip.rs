use rocket::{get, serde::json::Json};

use crate::utils::interfaceutils::DummyData;


#[get("/incentives/stip")]
pub async fn get_dummy_data() -> Json<DummyData>{
    let dummy_data = DummyData::default();

    Json(dummy_data)
}