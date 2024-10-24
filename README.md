# 🛰️ Helium CO2 Monitor

**Helium CO2 Monitor** is a Rust-based application designed to monitor indoor CO2 concentration levels in real-time using the **Datacake API**. It helps you maintain optimal air quality in spaces like offices by sending alerts to Slack when CO2 levels exceed a predefined threshold, indicating the need for better ventilation. 🌱

## 📋 Features
- Monitors real-time CO2 levels via **Datacake API** integration.
- Sends alerts to **Slack** when CO2 levels surpass a configurable threshold.
- Customizable Slack messages with dynamic CO2 values.
- Easy configuration via a YAML file.
- Command-line options for custom configuration files.

## 🔧 How It Works

1. **CO2 Monitoring**: The program fetches real-time CO2 levels from the Datacake API, connected to your device. Alternatively, you can set it up as a webhook directly from the Helium Console for more seamless integration.
2. **Threshold Checking**: If the CO2 concentration exceeds the defined threshold (e.g., 600 ppm), an alert message is sent to the specified Slack channel.
3. **Slack Notification**: The message template is customizable, allowing dynamic insertion of the CO2 value (replacing `%value%` with the actual reading). This way, your team can be notified instantly if air quality needs attention.

## 🚀 Getting Started

### Prerequisites

Before running the program, make sure you have:
- A valid **Datacake API key** and a **device ID** that monitors CO2 levels.
- A **Slack bot token** with permission to post messages in the specified channel.

### 🛠️ Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/xd-Abi/helium-co2-monitor.git
   cd helium-co2-monitor
   ```

2. Install the dependencies:
   ```bash
   cargo build
   ```

3. Update the `config.yml` file (or create a new one) with your credentials and settings.

    ```yaml
    slack:
        token: xoxb-your-slack-bot-token
        channel: office
        message: 📢 The office CO2 concentration has exceeded...
    datacake:
        key: your-datacake-api-key
        device: your-datacake-device-id
    threshold: 600
    ```

### 🏃 Usage

By default, the program will use the `config.yml` file located in the same directory. You can run the application using:

```bash
cargo run
```

If you'd like to specify a custom configuration file:

```bash
cargo run -- --config path/to/your/config.yml
```

This will monitor the CO2 levels in real-time and send a Slack notification if levels exceed the threshold defined in your configuration.

## 🛡️ Safety Recommendations

According to the [HSE Guidelines](https://www.hse.gov.uk/ventilation/using-co2-monitors.htm), it's crucial to maintain indoor CO2 levels below 1000 ppm to ensure proper ventilation and air quality. CO2 levels above 1500 ppm may indicate poor air quality, and action should be taken to improve ventilation.

## 🤖 Technologies Used

Here is the revised section with the requested links in **bold**:

- **[Rust](https://www.rust-lang.org/)**: High-performance, memory-safe programming language.
- **[Datacake API](https://docs.datacake.de/)**: For real-time CO2 concentration data.
- **[Slack API](https://api.slack.com/)**: For sending notifications to your Slack workspace.
- **[DecentLab DL-IAM](https://cdn.decentlab.com/download/datasheets/Decentlab-DL-IAM-datasheet.pdf)**: A precise CO2, temperature, and humidity sensor for real-time monitoring.

## 📘 Documentation & Resources

- [Datacake API Documentation](https://docs.datacake.de/)
- [Slack API Documentation](https://api.slack.com/)
- [HSE Ventilation Guidelines](https://www.hse.gov.uk/ventilation/using-co2-monitors.htm)
- [DecentLab DL-IAM](https://cdn.decentlab.com/download/datasheets/Decentlab-DL-IAM-datasheet.pdf)
