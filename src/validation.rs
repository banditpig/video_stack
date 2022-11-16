//Do we need to check that the files in
//client and dummy are actually video files?

use crate::args::Arguments;
use crate::VideoError;
use std::fs;
use std::path::Path;
fn folder_exists(folder: &str) -> Result<(), VideoError> {
    //println!("Check {:?}", Path::new(folder).try_exists());
    match Path::new(folder).exists() {
        true => Ok(()),
        false => Err(VideoError {
            reason: format!("Directory {} does not exist", folder),
        }),
    }
}
// ;
fn count_contained_files(folder: &str) -> Result<usize, VideoError> {
    folder_exists(folder)?;
    Ok(fs::read_dir(folder).unwrap().count())
}

//Make sure the three folders exist.
//make sure that the number of video files
//in the dummies_folder is >= quantity
pub fn check_args(args: &Arguments) -> Result<(), VideoError> {
    folder_exists(&args.client_folder)?;
    folder_exists(&args.dummies_folder)?;
    folder_exists(&args.output_folder)?;

    let file_count = count_contained_files(&args.dummies_folder)?;
    if file_count < args.quantity {
        Err(VideoError {
            reason: format!("Not enough dummy files {} does not exist", file_count),
        })
    } else {
        Ok(())
    }
}
