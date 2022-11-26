use crate::VideoError;
use clap::*;
use serde_derive::Deserialize;
use std::fmt::{Display, Formatter};
use std::fs;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    File {
        /// lists test values
        #[arg(short, long)]
        name: String,
    },
    Values {
        #[clap(short, long)]
        client_folder: String,
        #[clap(short, long)]
        dummies_folder: String,
        #[clap(short, long)]
        output_folder: String,
        #[clap(short, long)]
        quantity: usize,
        #[clap(short, long)]
        index_of_command: usize,
    },
}

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

#[derive(Deserialize)]
pub struct Configuration {
    pub args: Arguments,
}
impl Configuration {
    pub fn new(fname: &str) -> Result<Self, VideoError> {
        let contents = fs::read_to_string(fname)?;
        let config: Configuration = toml::from_str(&contents)?;
        Ok(config)
    }
}
