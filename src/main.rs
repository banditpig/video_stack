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

use crate::command_builder::{
    add_arguments_to_command, get_cmd_args, update_args_with_substitutions, VideoCommand,
};

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

fn process_videos(args: &Arguments, vid_cmd_args: &Vec<String>) -> Result<(), VideoError> {
    let all_commands = create_video_commands(args, vid_cmd_args)?;
    run_all_commands(all_commands)
}

fn files_in_folder(folder: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];

    for path in fs::read_dir(folder)? {
        let path = path?.path();
        files.push(path.to_owned());
    }
    Ok(files)
}
fn create_video_commands(
    args: &Arguments,
    vid_cmd_args: &Vec<String>,
) -> Result<Vec<VideoCommand>, VideoError> {
    let client_vids = files_in_folder(&args.client_folder)?; //.unwrap();
    let dummy_vids = files_in_folder(&args.dummies_folder)?;
    let mut total = 0;

    let mut all_commands = vec![];
    for cvid in client_vids {
        for ix in 0..args.quantity {
            let vid1 = format!("{}", cvid.display());
            let vid2 = format!("{}", dummy_vids[ix].display());
            let outname = format!("stacked_video{}.mp4", total);
            let mut arg_vec = update_args_with_substitutions(
                vid_cmd_args,
                vid1.as_str(),
                vid2.as_str(),
                outname.as_str(),
            );

            let cmd_name = arg_vec.remove(0); //eg ffmpeg
            let cmd = add_arguments_to_command(Command::new(cmd_name), &arg_vec);
            let video_cmd = VideoCommand {
                cmd,
                client_video: vid1,
                dummy_video: vid2,
                output_video: outname,
            };
            all_commands.push(video_cmd);
            total += 1;
        }
    }
    Ok(all_commands)
}

fn run_all_commands(all_commands: Vec<VideoCommand>) -> Result<(), VideoError> {
    let cores = available_parallelism().unwrap().get();
    let pool = ThreadPool::with_name("worker".into(), cores);
    let now = Instant::now();
    for mut video_cmd in all_commands {
        pool.execute(move || {
            println!("Running command... {}", video_cmd);

            let output = video_cmd.cmd.status();
            match output {
                Ok(status) => {
                    if status.success() {
                        println!("Stacked video {} created.", video_cmd.output_video);
                    } else {
                        println!("Failed to create {} ", video_cmd.output_video);
                    }
                }

                Err(e) => {
                    println!("Finished with error {:?}", e);
                }
            }
        });
    }

    pool.join();
    let elapsed = now.elapsed();
    println!("Time taken: {:.2?}", elapsed);
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

    let mut all_cmd_args = get_cmd_args(COMMANDS_FILE)?;
    let vid_cmd_args = all_cmd_args.get_mut(args.index_of_command).unwrap();
    match res {
        Ok(_) => process_videos(&args, vid_cmd_args).unwrap(),
        Err(e) => {
            println!("{}", e);
        }
    }
    Ok(())
}
