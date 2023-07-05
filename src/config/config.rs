use clap::Parser;
use super::args::Args;
use super::yaml::get_yaml;
use crate::api::data::Data;

pub struct Config {
    pub lat: f32,
    pub lon: f32,
    pub units: String,
    pub key: String,
    pub actions: Actions,
    pub verbose: bool,
}

pub struct Actions {
    pub print: Option<String>,
}

impl Config {
    pub fn actions(&self, data: &Data) {
        match &self.actions.print {
            Some(opts) => {
                let opts = opts.split(",").collect::<Vec<&str>>();
                for opt in opts {
                    self.actions.print(data, opt.to_string(), self.units[..].to_string(), self.verbose)
                }
            },
            None => (),
        };
    }
}

impl Actions {
    fn print(&self, data: &Data, opt: String, units: String, verbose: bool) {
        data.print(opt, units, verbose)
    }
}

// Get the configuration via the command line arguments and config file
// Use command line arguments over configuration file if arguments passed
pub fn get_config(fname: String) -> Config {
    let yaml = get_yaml(fname);
    let args = Args::parse();

    let lat = match args.lat {
        Some(lat) => lat,
        None => yaml.lat.unwrap(),
    };

    let lon = match args.lon {
        Some(lon) => lon,
        None => yaml.lon.unwrap(),
    };

    let units = match args.units {
        Some(units) => units,
        None => yaml.units.unwrap(),
    };

    let key = match args.key {
        Some(key) => key,
        None => yaml.key.unwrap(),
    };

    // Get actions to be completed
    let actions = Actions {
        print: args.print, 
    };
    
    let verbose = args.verbose; 

    Config { 
        lat, 
        lon, 
        units, 
        key,
        actions,
        verbose,
    }
}
