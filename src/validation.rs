//Do we need to check that the files in
//client and dummy are actually video files?

use crate::args::Arguments;
use crate::VideoError;
use clap::{arg, command};
use std::fs;
use std::fs::read_to_string;
use std::path::Path;
//Make sure the three folders exist.
//make sure that the number of video files
//in the dummies_folder is >= quantity
pub fn check_args(args: &Arguments) -> Result<(), VideoError> {
    folder_exists(&args.client_folder)?;
    folder_exists(&args.dummies_folder)?;
    //maybe this does not need to exist and can be created?
    folder_exists(&args.output_folder)?;
    file_exist("ffmpeg_commands.txt")?;
    command_index_ok(args.command_index)?;
    enough_dummy_files(args.quantity, &args.dummies_folder)?;
    Ok(())
}
fn folder_exists(folder: &str) -> Result<(), VideoError> {
    match Path::new(folder).exists() {
        true => Ok(()),
        false => Err(VideoError {
            reason: format!("Directory {} does not exist", folder),
        }),
    }
}

fn count_contained_files(folder: &str) -> Result<usize, VideoError> {
    folder_exists(folder)?;
    Ok(fs::read_dir(folder).unwrap().count())
}
fn file_exist(fname: &str) -> Result<(), VideoError> {
    if std::path::Path::new(fname).exists() {
        Ok(())
    } else {
        Err(VideoError {
            reason: format!(
                "The file of commands, {},  can't be found.\n \
            The file should be in the same folder as this application.",
                fname
            ),
        })
    }
}

fn enough_dummy_files(quantity: usize, dummies_folder: &String) -> Result<(), VideoError> {
    let file_count = count_contained_files(dummies_folder)?;
    if file_count < quantity {
        Err(VideoError {
            reason: format!("Not enough dummy files {}  exist", file_count),
        })
    } else {
        Ok(())
    }
}

fn command_index_ok(ix: usize) -> Result<(), VideoError> {
    let text = read_to_string("ffmpeg_commands.txt")?;
    let line_count = text.split("\n").count();
    if ix >= line_count {
        Err(VideoError {
            reason: format!("Not enough dummy files {} does not exist", line_count),
        })
    } else {
        Ok(())
    }
}
