use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};
use std::fs;

#[derive(Debug, Deserialize)]
struct DataCakeConfig {
    key: String,
}

#[derive(Debug, Deserialize)]
struct SlackConfig {
    token: String,
    channel: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    datacake: DataCakeConfig,
    device: String,
    threshold: f64,
    slack: SlackConfig,
}

fn main() {
    let config = load_config("config.yml");
    let co2_value = measure_co2(&config.datacake.key, &config.device);
    println!("CO2 Concentration Value: {:.2} ppm", co2_value);

    if co2_value > config.threshold {
        println!("CO2 concentration exceeds threshold! Sending Slack message...");
        send_slack_alert(
            &config.slack.token,
            &config.slack.channel,
            co2_value,
            &config.slack.message,
        );
    }
}

fn load_config(file_path: &str) -> Config {
    // Read the configuration file and expect it to succeed, panic if it fails
    let config_content = fs::read_to_string(file_path).expect(
        "Failed to read configuration file. Make sure 'config.yml' exists and is readable.",
    );

    // Parse the YAML content and expect successful parsing, panic if it fails
    let config: Config = serde_yaml::from_str(&config_content)
        .expect("Failed to parse configuration file. Ensure it is in correct YAML format.");

    config
}

fn measure_co2(api_key: &str, device_id: &str) -> f64 {
    let client = Client::builder()
        .build()
        .expect("Failed to build HTTP client.");

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Token {}", api_key)
            .parse()
            .expect("Invalid API key."),
    );
    headers.insert(
        "Content-Type",
        "application/json"
            .parse()
            .expect("Invalid content type header."),
    );

    let query = json!({
        "query": "query($deviceId:String!){device(deviceId:$deviceId){currentMeasurements(allActiveFields:true){value field{verboseFieldName fieldName}modified}}}",
        "variables": {
            "deviceId": device_id
        }
    });

    let response = client
        .post("https://api.datacake.co/graphql/")
        .headers(headers)
        .json(&query)
        .send()
        .expect("Failed to send request to Datacake API.");

    let body = response
        .text()
        .expect("Failed to read response body from Datacake API.");

    let json_response: Value =
        serde_json::from_str(&body).expect("Failed to parse response JSON from Datacake API.");

    if let Some(measurements) = json_response["data"]["device"]["currentMeasurements"].as_array() {
        if let Some(co2_value) = measurements
            .iter()
            .find(|m| m["field"]["fieldName"] == "CO2_CONCENTRATION")
            .and_then(|m| m["value"].as_f64())
        {
            return co2_value;
        }
    }

    panic!("CO2 measurement not found in the response from Datacake API.");
}

fn send_slack_alert(token: &str, slack_channel: &str, co2_value: f64, message: &str) {
    let client = Client::builder()
        .build()
        .expect("Failed to build HTTP client.");

    let mut slack_headers = HeaderMap::new();
    slack_headers.insert(
        "Authorization",
        format!("Bearer {}", token)
            .parse()
            .expect("Invalid Slack token."),
    );
    slack_headers.insert(
        "Content-Type",
        "application/json"
            .parse()
            .expect("Invalid content type header."),
    );

    let slack_message = json!({
        "channel": slack_channel,
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "Hey there, 👋 @channel"
                }
            },
            {
                "type": "divider"
            },
            {
                "type": "section",
                "text": {
                    "type": "plain_text",
                    "text": message,
                    "emoji": true
                }
            },
            {
                "type": "context",
                "elements": [
                    {
                        "type": "plain_text",
                        "text": format!("🌡️ Current CO2 Level: {:.2}ppm.", co2_value),
                        "emoji": true
                    }
                ]
            }
        ]
    });

    client
        .post("https://slack.com/api/chat.postMessage")
        .headers(slack_headers)
        .json(&slack_message)
        .send()
        .expect("Failed to send message to Slack.");
}
