use dotenv::dotenv;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("DATACAKE_KEY").expect("DATACAKE_KEY not found in .env");
    let device_id = env::var("DEVICE_ID").expect("DEVICE_ID not found in .env");
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Token {}", api_key).parse()?);
    headers.insert("Content-Type", "application/json".parse()?);

    let query = json!({
        "query": "query($deviceId:String!){device(deviceId:$deviceId){currentMeasurements(allActiveFields:true){value field{verboseFieldName fieldName}modified}}}",
        "variables": {
            "deviceId": device_id
        }
    });

    let request = client
        .request(reqwest::Method::POST, "https://api.datacake.co/graphql/")
        .headers(headers)
        .json(&query);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}
