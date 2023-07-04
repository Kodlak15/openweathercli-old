use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    latitude: String,
    #[arg(long)]
    longitude: String,
    #[arg(long)]
    units: Option<String>,
    #[arg(long)]
    key: String,
}

pub fn construct_uri() -> String {
    let args = Args::parse();

    match args.units {
        Some(units) => {
            format!(
                "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units={}&appid={}",
                args.latitude,
                args.longitude,
                units,
                args.key,
            )
        },
        None => {
            format!(
                "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
                args.latitude,
                args.longitude,
                args.key,
            )
        },
    }
}
