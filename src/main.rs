use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let api_key = env::var("DATACAKE_KEY").expect("API_KEY not found in .env");
    let device_id = env::var("DEVICE_ID").expect("DEVICE_ID not found in .env");
}
