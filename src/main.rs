#[macro_use]
mod args;
use license::data::License;
use license::data::LicenseError;
use license::data::LicenseError::*;
extern crate lazy_static;
mod command_builder;
mod validation;
use crate::args::{Arguments, Configuration};
use crate::command_builder::{
    add_arguments_to_command, get_cmd_args, update_args_with_substitutions, VideoCommand,
};
use crate::validation::{check_args, folder_exists, video_files_in_folder, COMMANDS_FILE};
use clap::Parser;
use flexi_logger::FileSpec;
use flexi_logger::Logger;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use log::{debug, error, info};
use rayon::prelude::*;
use std::fmt::{Display, Formatter};
use std::process::Command;
use std::time::{Duration, Instant};
use std::{error, fs, io};

lazy_static! {
    pub static ref EXTENSIONS: Vec<&'static str> = {
        let m = vec!["avi", "mov", "mp4", "mpg", "mpeg", "wmv"];
        m
    };
}
#[derive(Debug, Clone)]
pub struct VideoError {
    pub reason: String,
}
impl From<io::Error> for VideoError {
    fn from(e: io::Error) -> VideoError {
        VideoError {
            reason: e.to_string(),
        }
    }
}
impl From<toml::de::Error> for VideoError {
    fn from(e: toml::de::Error) -> VideoError {
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
impl From<LicenseError> for VideoError {
    fn from(e: LicenseError) -> VideoError {
        match e {
            DateFormat(d) => VideoError { reason: d },
            JSONIncorrect(d) => VideoError { reason: d },
            FileError(d) => VideoError { reason: d },
            SigningProblem(d) => VideoError { reason: d },
            UserDataError(d) => VideoError { reason: d },
        }

        // VideoError {
        //     reason: e.to_string(),
        // }
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
    debug!("Using ffmepg command {:?}", vid_cmd_args);
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
        debug!("Running command {}", video_cmd);
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
                error!(
                    "Error {} creating {}.",
                    e.to_string(),
                    video_cmd.output_video
                );
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

fn check_licence() -> Result<(), VideoError> {
    let l: Result<License, LicenseError> = License::from_file("video_stacker.lic");
    match l {
        Ok(lic) => lic.check_license()?,
        Err(e) => {
            return Err(VideoError::from(e));
        }
    }

    Ok(())
}
fn run() -> Result<(), VideoError> {
    check_licence()?;

    let config = Configuration::new()?;

    let args: Arguments = config.args; //Arguments::parse();
    println!("Using setting:\n{}", args);
    check_args(&args)?;
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
    Logger::try_with_str("debug")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("log_files") // create files in folder ./log_files
                .basename("video_stack")
                .suppress_timestamp(),
        )
        .start()
        .expect("TODO: panic message");
    info!("Logging");

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
}
