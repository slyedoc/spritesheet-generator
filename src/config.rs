use std::str::FromStr;

use clap::{AppSettings, Clap};

#[derive(Clone, Debug, Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Config {
    #[clap(short, long)]
    pub input_folder: String,
    #[clap(short, long, default_value = "./")]
    pub output_folder: String,
    #[clap(short, long, default_value = "filename")]
    pub name: String,
    #[clap(short, long, default_value = "2")]
    pub border_padding: u32,
    #[clap(long, default_value = "1024")]
    pub max_width: u32,
    #[clap(long, default_value = "1024")]
    pub max_height: u32,
    #[clap(short, long)]
    pub allow_rotation: bool,
    #[clap(short = 'd', long)]
    pub debug_outlines: bool,

    #[clap(short, long, default_value = "ron", possible_values = &["json", "ron"])]
    pub format: Format
}

#[derive(Clone, Debug)]
pub enum Format {
    Json,
    Ron,
}

// Implement the trait
impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "ron" => Ok(Format::Ron),
            _ => Err("no match"),
        }
    }
}