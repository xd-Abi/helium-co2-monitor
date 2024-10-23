use dotenv::dotenv;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the required environment variables
    let api_key = env::var("DATACAKE_KEY").expect("DATACAKE_KEY not found in .env");
    let device_id = env::var("DEVICE_ID").expect("DEVICE_ID not found in .env");
    let slack_token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN not found in .env");
    let slack_channel = env::var("SLACK_CHANNEL").expect("SLACK_CHANNEL not found in .env");

    // Create a reqwest client
    let client = Client::builder().build()?;

    // Set up headers for the Datacake API request
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Token {}", api_key).parse()?);
    headers.insert("Content-Type", "application/json".parse()?);

    // Construct the GraphQL query
    let query = json!({
        "query": "query($deviceId:String!){device(deviceId:$deviceId){currentMeasurements(allActiveFields:true){value field{verboseFieldName fieldName}modified}}}",
        "variables": {
            "deviceId": device_id
        }
    });

    // Send the request to the Datacake API
    let response = client
        .post("https://api.datacake.co/graphql/")
        .headers(headers)
        .json(&query)
        .send()
        .await?;

    // Parse the response body as JSON
    let body = response.text().await?;
    let json_response: Value = serde_json::from_str(&body)?;

    // Extract the CO2 concentration measurement
    let mut co2_message = String::from("No CO2 measurements found.");
    if let Some(measurements) = json_response["data"]["device"]["currentMeasurements"].as_array() {
        if let Some(co2_value) = measurements
            .iter()
            .find(|m| m["field"]["fieldName"] == "CO2_CONCENTRATION")
            .and_then(|m| m["value"].as_f64())
        {
            co2_message = format!("CO2 Concentration Value: {:.2} ppm", co2_value);
            println!("{}", co2_message); // Print out the measurement
        }
    }

    // Set up headers for the Slack API request
    let mut slack_headers = HeaderMap::new();
    slack_headers.insert("Authorization", format!("Bearer {}", slack_token).parse()?);
    slack_headers.insert("Content-Type", "application/json".parse()?);

    // Construct the Slack message payload
    let slack_message = json!({
        "channel": slack_channel,
        "text": co2_message
    });

    // Send the message to Slack
    let slack_response = client
        .post("https://slack.com/api/chat.postMessage")
        .headers(slack_headers)
        .json(&slack_message)
        .send()
        .await?;

    // Check if the Slack message was sent successfully
    let slack_body = slack_response.text().await?;
    println!("Slack Response: {}", slack_body);

    Ok(())
}
