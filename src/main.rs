use clap::Parser;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
enum MyError {
    MissingField(String),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> MyError {
        MyError::ReqwestError(err)
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    #[clap(short, long)]
    city: String,
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    process_city(&opts.city).await;
}

async fn process_city(city_name: &str) {
    match get_coordinates(city_name).await {
        Ok((lat, lng)) => println!("lat: {}, lng: {}", lat, lng),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("Processing city: {}", city_name);
}

async fn get_coordinates(city_name: &str) -> Result<(f64, f64), MyError> {
    dotenv().ok();

    let api_key = env::var("GOOGLE_MAPS_API_KEY").expect("GOOGLE_MAPS_API_KEY must be set");
    let url = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
        city_name, api_key
    );

    let response: serde_json::Value = reqwest::get(&url).await?.json().await?;

    let lat = response["results"][0]["geometry"]["location"]["lat"]
        .as_f64()
        .ok_or(MyError::MissingField(
            "Latitude is missing or not a float".to_string(),
        ))?;
    let lng = response["results"][0]["geometry"]["location"]["lng"]
        .as_f64()
        .ok_or(MyError::MissingField(
            "Longitude is missing or not a float".to_string(),
        ))?;

    Ok((lat, lng))
}
