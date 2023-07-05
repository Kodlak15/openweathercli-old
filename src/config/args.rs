use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub lat: Option<f32>,
    #[arg(long)]
    pub lon: Option<f32>,
    #[arg(long)]
    pub units: Option<String>,
    #[arg(long)]
    pub key: Option<String>,
    #[arg(long)]
    pub print: Option<String>,
    #[arg(long, action)]
    pub verbose: bool,
}
