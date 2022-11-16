use clap::*;

#[derive(Parser, Default, Debug)]
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
}
