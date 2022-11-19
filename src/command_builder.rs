use crate::VideoError;
use std::ops::Index;
use std::process::Command;

#[derive(Debug)]
pub struct VideoCommand {
    pub(crate) cmd: Command,
    pub(crate) client_video: String,
    pub(crate) dummy_video: String,
    pub(crate) output_video: String,
}

pub fn update_args_with_substitutions(
    input_vec: &Vec<String>,
    tv: &str,
    bv: &str,
    ov: &str,
) -> Vec<String> {
    //TOP_VID, BOTTOM_VID, OUTVID

    let mut args_clone = input_vec.clone();
    let ix = args_clone.iter().position(|s| s == "TOP_VID").unwrap(); //Should handle this error!
    let value = std::mem::replace(&mut args_clone[ix], tv.to_string());

    let ix = args_clone.iter().position(|s| s == "BOTTOM_VID").unwrap();
    let value = std::mem::replace(&mut args_clone[ix], bv.to_string());

    let ix = args_clone.iter().position(|s| s == "OUTVID").unwrap();
    let value = std::mem::replace(&mut args_clone[ix], ov.to_string());

    args_clone //Is this OK??? or should I not return a ref and just clone vec?
}
pub fn add_arguments_to_command(mut cmd: Command, args: &Vec<String>) -> Command {
    for arg in args {
        let cmd = cmd.arg(arg);
    }
    cmd
}
///Each ffmpeg command is on one line and seperated by ' '.
///
pub fn get_cmd_args(cmds_file_name: &str) -> Result<Vec<Vec<String>>, VideoError> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .flexible(true)
        .from_path(cmds_file_name)?;

    let mut all_args: Vec<Vec<String>> = vec![];
    for result in rdr.records() {
        let mut args = vec![];
        let record = result?;
        for r in record.iter() {
            args.push(r.to_string())
        }
        all_args.push(args)
    }
    Ok(all_args)
}
//--------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    use crate::args::Arguments;
    use std::assert_eq;
    use std::path::Path;

    #[test]
    fn load_commands_all_good() {}
    #[test]
    fn load_commands_one_bad() {
        // //this file has command that starts with 'ffprobe'
        // let file_name = "./testfiles/bad_cmdfile.txt";
        // let cmds = all_commands(file_name);
        // assert!(cmds.is_err());
    }
    #[test]
    fn load_commands_empty_file() {
        // let file_name = "./testfiles/empty_cmdfile.txt";
        // let cmds = all_commands(file_name).unwrap();
        // assert_eq!(cmds.len(), 0);
    }
    #[test]
    fn load_commands_no_file() {
        // let file_name = "./testfiles/nosuchfile.txt";
        // let cmds = all_commands(file_name);
        // assert!(cmds.is_err(), "The file should not exist");
    }
    #[test]
    fn update_arguments() {
        ////TOP_VID, BOTTOM_VID, OUTVID
        let args_vec = vec![
            "-flags".to_string(),
            "2".to_string(),
            "-i".to_string(),
            "TOP_VID".to_string(),
            "x".to_string(),
            "BOTTOM_VID".to_string(),
            "OUTVID".to_string(),
        ];

        let res = update_args_with_substitutions(&args_vec, "top.mp4", "bottom.mp4", "output.mp4");
        assert!(res.contains(&"top.mp4".to_string()));
        assert!(res.contains(&"bottom.mp4".to_string()));
        assert!(res.contains(&"output.mp4".to_string()));

        println!("{:?}", res);
    }
}
