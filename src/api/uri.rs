use crate::config::config::Config;

pub fn construct_uri(config: &Config) -> String {
    format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units={}&appid={}",
        config.lat,
        config.lon,
        config.units,
        config.key,
    )
}
