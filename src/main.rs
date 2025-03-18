mod framework;
mod commands;
mod get_build_api;
mod types;

use dotenvy::dotenv;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if let Err(e) = framework::run().await {
        eprintln!("Error: {}", e);
    }
}
