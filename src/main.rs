#![allow(unused)]
#[macro_use]
// extern crate lazy_static;
mod args;

mod command_builder;
mod validation;

use crate::args::Arguments;
use crate::validation::{check_args, folder_exists, COMMANDS_FILE};
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::Command;
use std::thread::available_parallelism;
use std::time::Instant;
use std::{fs, io};
use threadpool::ThreadPool;

use crate::command_builder::{all_commands, VideoCommand};
use std::collections::HashMap;
use std::ops::Index;

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
impl From<csv::Error> for VideoError {
    fn from(e: csv::Error) -> VideoError {
        VideoError {
            reason: e.to_string(),
        }
    }
}
impl Display for VideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

fn files_in_folder(folder: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];

    for path in fs::read_dir(folder)? {
        let path = path?.path();
        files.push(path.to_owned());
    }
    Ok(files)
}

fn process_videos(
    args: &Arguments,
    mut vid_cmd: &mut command_builder::VideoCommand,
) -> Result<(), VideoError> {
    let cores = available_parallelism().unwrap().get();
    let pool = ThreadPool::with_name("worker".into(), cores);

    let client_vids = files_in_folder(&args.client_folder)?; //.unwrap();
    let dummy_vids = files_in_folder(&args.dummies_folder)?;
    let mut total = 0;
    let now = Instant::now();
    let mut all_commands = vec![];
    for cvid in client_vids {
        for ix in 0..args.quantity {
            let vid1 = format!("{}", cvid.display());
            let vid2 = format!("{}", dummy_vids[ix].display());
            let outname = format!("stacked_video{}.mp4", total);
            let arg_vec = vid_cmd.update_args_with_substitutions(
                vid1.as_str(),
                vid2.as_str(),
                outname.as_str(),
            );
            let cmd = Command::new("ffmpeg");
            let mut output = vid_cmd.add_arguments_to_command(cmd, &arg_vec);
            all_commands.push(output);
            total += 1;
        }
    }

    for mut cmd in all_commands {
        pool.execute(move || {
            let output = cmd.status();

            match output {
                Ok(status) => {
                    if status.success() {
                        println!("Stacked video  created.");
                    } else {
                        println!("Failed to create ");
                    }
                }

                Err(e) => {
                    println!("Finishedwith error {:?}", e);
                }
            }
        });
    }

    pool.join();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}

fn main() -> Result<(), VideoError> {
    let args: Arguments = Arguments::parse();
    let res = check_args(&args);
    let out_folder = folder_exists(&args.output_folder);
    match out_folder {
        Ok(_) => {}
        Err(_) => {
            fs::create_dir(&args.output_folder)?;
        }
    }
    //now which cmd is being used

    let mut all_cmds = all_commands(COMMANDS_FILE)?;

    let mut vid_cmd = all_cmds.get_mut(args.command_index).unwrap();

    match res {
        Ok(_) => process_videos(&args, vid_cmd).unwrap(),
        Err(e) => {
            println!("{}", e);
        }
    }
    Ok(())
}
