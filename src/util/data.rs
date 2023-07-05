use crate::api::data::Data;

impl Data {
    // Print data for specified metric
    pub fn print(&self, opt: String, units: String, verbose: bool) {
        match &opt[..] {
            "lat" => println!("{}", self.lat(verbose)),
            "lon" => println!("{}", self.lon(verbose)),
            "id" => println!("{}", self.id(verbose)),
            "main" => println!("{}", self.main(verbose)),
            "description" => println!("{}", self.description(verbose)),
            "icon" => println!("{}", self.icon(verbose)),
            "temp" => println!("{}", self.temp(units, verbose)),
            "feels_like" => println!("{}", self.feels_like(units, verbose)),
            "max_temp" => println!("{}", self.temp_max(units, verbose)),
            "min_temp" => println!("{}", self.temp_min(units, verbose)),
            "pressure" => println!("{}", self.pressure(verbose)),
            "humidity" => println!("{}", self.humidity(verbose)),
            "sea_level" => println!("{}", self.sea_level(verbose)),
            "grnd_level" => println!("{}", self.grnd_level(verbose)),
            "name" => println!("{}", self.name(verbose)),
            _ => println!("Invalid option selected: {}", opt),
        };
    }

    // Coordinates
    pub fn lat(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Latitude: {}", self.coord.lat),
            false => self.coord.lat.to_string(),
        }
    }

    pub fn lon(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Longitude: {}", self.coord.lon),
            false => self.coord.lon.to_string(),
        }
    }

    // Weather
    pub fn id(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Id: {}", self.weather.data.id),
            false => self.weather.data.id.to_string(),
        }
    }

    pub fn main(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Main: {}", self.weather.data.main),
            false => self.weather.data.main.to_string(),
        }
    }

    pub fn description(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Description: {}", self.weather.data.description),
            false => self.weather.data.description.to_string(),
        }
    }

    pub fn icon(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Icon: {}", self.weather.data.icon),
            false => self.weather.data.icon.to_string(),
        }
    }

    // Main
    pub fn temp(&self, units: String, verbose: bool) -> String {
        match verbose {
            true => format!("Temperature: {} {}", self.main.temp, get_temp_units(units)),
            false => self.main.temp.to_string(),
        }
    }

    pub fn feels_like(&self, units: String, verbose: bool) -> String {
        match verbose {
            true => format!("Feels like: {} {}", self.main.feels_like, get_temp_units(units)),
            false => self.main.feels_like.to_string(),
        }
    }

    pub fn temp_min(&self, units: String, verbose: bool) -> String {
        match verbose {
            true => format!("Minimum temperature: {} {}", self.main.temp_min, get_temp_units(units)),
            false => self.main.temp_min.to_string(),
        }
    }

    pub fn temp_max(&self, units: String, verbose: bool) -> String {
        match verbose {
            true => format!("Maximum temperature: {} {}", self.main.temp_max, get_temp_units(units)),
            false => self.main.temp_max.to_string(),
        }
    }

    pub fn pressure(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Pressure: {} hPa", self.main.pressure),
            false => self.main.pressure.to_string(),
        }
    }

    pub fn humidity(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Pressure: {} %", self.main.humidity),
            false => self.main.humidity.to_string(),
        }
    }

    pub fn sea_level(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Sea level pressure: {} hPa", self.main.sea_level.expect("No data for field sea_level")),
            false => self.main.sea_level.expect("No data for field sea_level").to_string(),
        }
    }

    pub fn grnd_level(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Ground level pressure: {} hPa", self.main.grnd_level.expect("No data for field grnd_level")),
            false => self.main.sea_level.expect("No data for field grnd_level").to_string(),
        }
    }

    // Location
    pub fn name(&self, verbose: bool) -> String {
        match verbose {
            true => format!("Location: {}", self.name),
            false => self.name.to_string(),
        }
    }
}

fn get_temp_units(units: String) -> String {
    match &units[..] {
        "standard" => "K".to_string(),
        "metric" => "C".to_string(),
        "imperial" => "F".to_string(),
        _ => format!("Invailid unit type: {}", units),
    }
}
