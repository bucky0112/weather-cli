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
        Ok((lat, lng)) => match get_weather(lat as u32, lng as u32).await {
            Ok((weather_description, temperature)) => {
                println!("City: {}\nLatitude: {:.2}, Longitude: {:.2}\nCurrent Weather: {}\nTemperature: {:.2}°C",
                city_name, lat, lng, weather_description, temperature);
            }
            Err(e) => println!("Error: {:?}", e),
        },
        Err(e) => println!("Error: {:?}", e),
    }
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

async fn get_weather(lat: u32, lon: u32) -> Result<(String, f64), MyError> {
    dotenv().ok();

    let api_key = env::var("OPEN_WEATHER_API_KEY").expect("OPEN_WEATHER_API_KEY must be set");
    let api_url = env::var("OPEN_WEATHER_API_URL").expect("OPEN_WEATHER_API_URL must be set");
    let url = format!(
        "{}lat={}&lon={}&appid={}&units=metric&exclude=hourly,daily",
        api_url, lat, lon, api_key
    );

    let response: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let weather_description = response["current"]["weather"][0]["description"]
        .as_str()
        .unwrap()
        .to_string();
    let temperature = response["current"]["temp"].as_f64().unwrap();
    Ok((weather_description, temperature))
}
