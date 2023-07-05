use crate::{api::data::Data, config::args::Args};

impl Args {
    fn print(&self, data: Data, units: String, verbose: bool) {
        match &self.print {
            Some(opt) => data.print(opt.to_string(), units, verbose),
            None => println!("Nothing to print!")
        }
    }
}

pub fn handle(args: Args, data: Data, units: String, verbose: bool) {
    match &args.print {
        Some(_) => args.print(data, units, verbose),
        None => (),
    }
}
