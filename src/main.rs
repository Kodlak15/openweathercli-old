use clap::Parser;
use reqwest;
use reqwest::{Response, Error};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let uri = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units={}&appid={}",
        args.latitude,
        args.longitude,
        args.units,
        args.key,
    );

    // Find a way to parse json into struct
    let res = get_response(uri).await.unwrap();
    println!("response = {:?}", res.text().await);
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    latitude: String,
    #[arg(long)]
    longitude: String,
    #[arg(long)]
    units: String,
    #[arg(long)]
    key: String,
}

#[derive(Deserialize)]
struct Coord {
    longitude: f32,
    latitude: f32,
}

#[derive(Deserialize)]
struct Data {
    coords: Coord,
}

async fn get_response(uri: String) -> Result<Response, Error> {
    reqwest::get(uri).await
}
