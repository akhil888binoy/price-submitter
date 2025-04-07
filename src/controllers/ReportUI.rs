use rocket::{get, serde::json::Json};


#[get("/report/ui")]
pub async fn report_ui() -> Json<bool> {
    Json(true)
}