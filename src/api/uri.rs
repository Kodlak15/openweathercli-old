use clap::Parser;
use serde::Deserialize;
use serde_yaml;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Yaml {
    pub lat: Option<f32>,
    pub lon: Option<f32>,
    pub units: Option<String>,
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    lat: Option<String>,
    #[arg(long)]
    lon: Option<String>,
    #[arg(long)]
    units: Option<String>,
    #[arg(long)]
    key: String,
    #[arg(long)]
    option: Option<String>,
}

pub fn construct_uri() -> String {
    // Open the configuration file and get the cli arguments
    let f = std::fs::File::open("config.yaml").expect("Could not open file: config.yaml");
    let yaml: Yaml = serde_yaml::from_reader(f).expect("Could not read values from file: config.yaml");
    let args = Args::parse();

    // Get the latitude, longitude, and units
    // If cli arguments are passed, prefer to use those
    let lat = match args.lat {
        Some(lat) => lat,
        None => yaml.lat.unwrap().to_string(),
    };
    let lon = match args.lon {
        Some(lon) => lon,
        None => yaml.lon.unwrap().to_string(),
    };
    let units = match args.units {
        Some(units) => units,
        None => yaml.units.unwrap().to_string(),
    };

    // Return the uri string
    format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units={}&appid={}",
        lat,
        lon,
        units,
        args.key,
    )
}
