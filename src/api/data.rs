use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct WeatherData {
    pub data: Weather
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Weather {
    pub id: i32,
    pub main: String, 
    pub description: String, 
    pub icon: String, 
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32, 
    pub temp_min: f32, 
    pub temp_max: f32, 
    pub pressure: i32, 
    pub humidity: i32, 
    pub sea_level: Option<i32>,
    pub grnd_level: Option<i32>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Wind {
    pub speed: f32,
    pub deg: i32, 
    pub gust: f32, 
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Clouds {
    pub all: i32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Sys {
    pub country: String,
    pub sunrise: i32,
    pub sunset: i32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Data {
    pub weather: WeatherData,
    pub base: String,
    pub main: Main,
    pub visibility: i32,
    pub coord: Coord,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i32,
    pub sys: Sys,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}
