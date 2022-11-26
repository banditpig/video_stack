use crate::VideoError;
use clap::*;
use serde_derive::Deserialize;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Deserialize, Parser, Default, Debug)]
#[clap(author = "BanditPig", version, about)]
/// Utility to vertically stack two video files using ffmpeg.

pub struct Arguments {
    #[clap(short, long)]
    pub client_folder: String,
    #[clap(short, long)]
    pub dummies_folder: String,
    #[clap(short, long)]
    pub output_folder: String,
    #[clap(short, long)]
    pub quantity: usize,
    #[clap(short, long)]
    pub index_of_command: usize,
}
impl Display for Arguments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " Client folder {}\n Dummies folder {}\n Output folder {}\n Quantity {}\n Command index {}\n",
               self.client_folder, self.dummies_folder, self.output_folder, self.quantity, self.index_of_command)
    }
}

// fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     write!(f, "({}, {})", self.x, self.y)
// }
#[derive(Deserialize)]
pub struct Configuration {
    pub args: Arguments,
}
impl Configuration {
    pub fn new() -> Result<Self, VideoError> {
        let contents = fs::read_to_string("app.toml")?;
        let config: Configuration = toml::from_str(&contents)?;
        Ok(config)
    }
}
