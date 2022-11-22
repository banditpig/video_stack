#[macro_use]
mod args;

extern crate lazy_static;
mod command_builder;
mod validation;
use crate::args::Arguments;
use crate::command_builder::{
    add_arguments_to_command, get_cmd_args, update_args_with_substitutions, VideoCommand,
};
use crate::validation::{check_args, folder_exists, video_files_in_folder, COMMANDS_FILE};
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;

use rayon::prelude::*;
use std::fmt::{Display, Formatter};
use std::process::Command;
use std::time::{Duration, Instant};
use std::{error, fs, io};

lazy_static! {
    pub static ref EXTENSIONS: Vec<&'static str> = {
        let mut m = Vec::new();
        m.push("avi");
        m.push("mkv");
        m.push("mov");
        m.push("mp4");
        m.push("mpg");
        m.push("mpeg");
        m.push("wmv");
        m
    };
}
#[derive(Debug, Clone)]
pub struct VideoError {
    pub reason: String,
}
impl From<std::io::Error> for VideoError {
    fn from(e: io::Error) -> VideoError {
        VideoError {
            reason: e.to_string(),
        }
    }
}
impl From<csv::Error> for VideoError {
    fn from(e: csv::Error) -> VideoError {
        VideoError {
            reason: e.to_string(),
        }
    }
}

impl Display for VideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "aaa") //self.reason)
    }
}
impl error::Error for VideoError {
    fn description(&self) -> &str {
        &self.reason
    }
}
fn process_videos(args: &Arguments, vid_cmd_args: &Vec<String>) -> Result<(), VideoError> {
    let all_commands = create_video_commands(args, vid_cmd_args)?;
    run_all_commands(all_commands)
}

fn create_video_commands(
    args: &Arguments,
    vid_cmd_args: &Vec<String>,
) -> Result<Vec<VideoCommand>, VideoError> {
    //

    let client_vids = video_files_in_folder(&args.client_folder)?; //.unwrap();
    let dummy_vids = video_files_in_folder(&args.dummies_folder)?;
    let mut total = 0;

    let mut all_commands = vec![];
    for cvid in client_vids {
        for ix in 0..args.quantity {
            let vid1 = format!("{}", cvid.display());
            let vid2 = format!("{}", dummy_vids[ix].display());
            let outname = format!("{}/stacked_video{}.mp4", args.output_folder, total);
            let mut arg_vec = update_args_with_substitutions(
                vid_cmd_args,
                vid1.as_str(),
                vid2.as_str(),
                outname.as_str(),
            )?;

            let cmd_name = arg_vec.remove(0); //eg ffmpeg
            let cmd = add_arguments_to_command(Command::new(cmd_name), &arg_vec);
            let video_cmd = VideoCommand {
                cmd,
                client_video: vid1,
                dummy_video: vid2,
                output_video: outname,
                ix: total,
            };
            all_commands.push(video_cmd);
            total += 1;
        }
    }
    Ok(all_commands)
}
fn run_all_commands(all_commands: Vec<VideoCommand>) -> Result<(), VideoError> {
    let total = all_commands.len();
    println!("Creating {} stacked videos.", total);
    let now = Instant::now();

    let m = MultiProgress::new();
    let sty =
        ProgressStyle::with_template("{spinner:.white} [{elapsed}] {bar:40.cyan/green}  {msg}")
            .unwrap()
            .progress_chars("##-");
    let error_sty =
        ProgressStyle::with_template("{spinner:.red} [{elapsed}] {bar:40.red/green}  {msg}")
            .unwrap()
            .progress_chars("***");

    all_commands.into_par_iter().for_each(|mut video_cmd| {
        let pb = m.add(ProgressBar::new(1));
        pb.set_style(sty.clone());

        let d = Duration::from_millis(50);
        pb.enable_steady_tick(d);
        pb.set_message(format!("Creating {}", video_cmd.output_video));
        let res = video_cmd.cmd.status();
        match res {
            Ok(_) => {
                pb.finish_with_message(format!("Video {} is complete", video_cmd.output_video));
            }
            Err(e) => {
                pb.set_style(error_sty.clone());
                pb.finish_with_message(format!(
                    "Error {} creating {}.",
                    e.to_string(),
                    video_cmd.output_video
                ));
            }
        }
    });
    let elapsed = now.elapsed();
    println!("Time taken: {:.2?} and {} videos created.", elapsed, total);

    Ok(())
}

fn run() -> Result<(), VideoError> {
    let args: Arguments = Arguments::parse();
    let _ = check_args(&args)?;
    let out_folder = folder_exists(&args.output_folder);
    match out_folder {
        Ok(_) => {}
        Err(_) => {
            fs::create_dir(&args.output_folder)?;
        }
    }

    let mut all_cmd_args = get_cmd_args(COMMANDS_FILE)?;
    let vid_cmd_args = all_cmd_args.get_mut(args.index_of_command).unwrap();
    process_videos(&args, vid_cmd_args)?;

    Ok(())
}
fn main() {
    let res = run();
    match res {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(e) => {
            println!("Problem creating videos:\n{}", e.reason);
            std::process::exit(1);
        }
    }

    // if let Err(err) = run() {
    //     eprintln!("Error: {:?}", err);
    //     std::process::exit(1);
    // }
}
