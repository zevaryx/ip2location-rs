use clap::Parser;
use serde::Deserialize;
use std::fmt;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    api_key: String,
    ip: String,
}

#[derive(Deserialize)]
struct Ip2Location {
    ip: String,
    country_code: String,
    country_name: String,
    region_name: String,
    city_name: String,
    latitude: f64,
    longitude: f64,
    zip_code: String,
    time_zone: String,
    asn: String,
    #[serde(rename = "as")]
    as_: String,
    is_proxy: bool,
}

#[derive(Deserialize)]
struct ErrorData {
    error_code: i32,
    error_message: String,
}

#[derive(Deserialize)]
struct Error {
    error: ErrorData,
}

impl fmt::Display for Ip2Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "   IP: {}\n City: {}, {}, {} ({})\n  ISP: {} ({})\n  Geo: {}, {}\n   TZ: {}\n  ZIP: {}\nProxy: {}",
            self.ip,
            self.city_name,
            self.region_name,
            self.country_code,
            self.country_name,
            self.as_,
            self.asn,
            self.latitude,
            self.longitude,
            self.time_zone,
            self.zip_code,
            self.is_proxy
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let resp = reqwest::get(format!(
        "https://api.ip2location.io/?key={}&ip={}&format=json",
        cli.api_key, cli.ip
    ))
    .await?;

    match resp.status().as_u16() {
        200 => {
            let result = resp.json::<Ip2Location>().await?;
            println!("{result}");
            Ok(())
        }
        _ => {
            let result = resp.json::<Error>().await?;
            let err_msg = format!(
                "Error Code {}: {}",
                result.error.error_code, result.error.error_message
            );

            Err(err_msg.into())
        }
    }
}
