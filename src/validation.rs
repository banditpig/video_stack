//Do we need to check that the files in
//client and dummy are actually video files?

use crate::args::Arguments;
use crate::VideoError;
use clap::{arg, command};
use std::fs;
use std::fs::read_to_string;
use std::path::Path;
pub const COMMANDS_FILE: &str = "ffmpeg_commands.txt";
//Make sure the three folders exist.
//make sure that the number of video files
//in the dummies_folder is >= quantity
pub fn check_args(args: &Arguments) -> Result<(), VideoError> {
    folder_exists(&args.client_folder)?;
    folder_exists(&args.dummies_folder)?;
    //maybe this does not need to exist and can be created?
    folder_exists(&args.output_folder)?;
    file_exist("ffmpeg_commands.txt")?;
    command_index_ok(args.index_of_command, COMMANDS_FILE)?;
    enough_dummy_files(args.quantity, &args.dummies_folder.as_str())?;
    Ok(())
}
pub fn folder_exists(folder: &str) -> Result<(), VideoError> {
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

fn enough_dummy_files(quantity: usize, dummies_folder: &str) -> Result<(), VideoError> {
    let file_count = count_contained_files(dummies_folder)?;
    if file_count < quantity {
        Err(VideoError {
            reason: format!("Not enough dummy files {}  exist", file_count),
        })
    } else {
        Ok(())
    }
}

fn command_index_ok(ix: usize, cmd_file: &str) -> Result<(), VideoError> {
    let text = read_to_string(cmd_file)?;
    let line_count = text.split("\n").count();
    if ix >= line_count {
        Err(VideoError {
            reason: format!(
                "Command index {} is too large. Should be <= {}",
                ix, line_count
            ),
        })
    } else {
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use std::assert_eq;
    use std::path::Path;

    #[test]
    fn folder_exists_test() {
        //happy path
        let res = folder_exists("./testfiles");
        assert!(res.is_ok(), "The folder does exist");
        let res = folder_exists("./NOSUCHFOLDRE");
        assert!(res.is_err(), "The folder does exist");
    }
    #[test]
    fn file_exists_test() {
        //happy path
        let res = file_exist("./testfiles/good_cmdfile.txt");
        assert!(res.is_ok(), "The file does exist");
        let res = file_exist("./testfiles/NOSUCHFILE.txt");
        assert!(res.is_err(), "The folder does exist");
    }
    #[test]
    fn command_index() {
        let res = command_index_ok(0, "./testfiles/good_cmdfile.txt");
        assert!(res.is_ok());

        let res = command_index_ok(4, "./testfiles/good_cmdfile.txt");
        assert!(res.is_err());
    }

    #[test]
    fn dummy_files() {
        let res = enough_dummy_files(3, "./testfiles");
        assert!(res.is_ok());
        let res = enough_dummy_files(4, "./testfiles");
        assert!(res.is_err());
    }
}
