
use sea_orm::*;

#[path = "./jobs/index.rs"]
mod index;

use index::executejobs;


#[tokio::main]
async fn main() {
    executejobs().await;
}