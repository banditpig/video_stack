use crate::VideoError;
use std::fmt::{Display, Formatter};
use std::process::Command;

#[derive(Debug)]
pub struct VideoCommand {
    pub(crate) cmd: Command,
    pub(crate) client_video: String,
    pub(crate) dummy_video: String,
    pub(crate) output_video: String,
    pub(crate) ix: i32,
}
impl Display for VideoCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Combining {} and  {} to create {}",
            self.client_video, self.dummy_video, self.output_video
        )
    }
}
pub fn update_args_with_substitutions(
    input_vec: &Vec<String>,
    tv: &str,
    bv: &str,
    ov: &str,
) -> Result<Vec<String>, VideoError> {
    let mut args_clone = input_vec.clone();
    let ix = args_clone
        .iter()
        .position(|s| s == "TOP_VID")
        .ok_or(VideoError {
            reason: "Missing 'TOP_VID' placeholder in command".to_string(),
        })?;
    let _ = std::mem::replace(&mut args_clone[ix], tv.to_string());

    let ix = args_clone
        .iter()
        .position(|s| s == "BOTTOM_VID")
        .ok_or(VideoError {
            reason: "Missing 'BOTTOM_VID' placeholder in command".to_string(),
        })?;

    let _ = std::mem::replace(&mut args_clone[ix], bv.to_string());

    let ix = args_clone
        .iter()
        .position(|s| s == "OUT_VID")
        .ok_or(VideoError {
            reason: "Missing 'OUT_VID' placeholder in command".to_string(),
        })?;
    let _ = std::mem::replace(&mut args_clone[ix], ov.to_string());

    Ok(args_clone)
}
pub fn add_arguments_to_command(mut cmd: Command, args: &Vec<String>) -> Command {
    for arg in args {
        cmd.arg(arg);
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
            if r != "" {
                args.push(r.to_string())
            }
        }
        all_args.push(args)
    }
    match all_args.len() {
        0 => Err(VideoError {
            reason: "Commands file is empty".to_string(),
        }),
        _ => Ok(all_args),
    }
}
//--------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn get_cmd_args_all_good() {
        let file_name = "./testfiles/good_cmdfile.txt";
        let all_args = get_cmd_args(file_name);
        assert!(all_args.is_ok());
        let args = all_args.unwrap();
        assert_eq!(3, args.len());
        assert_eq!("ffmpeg", args[0][0]);
        assert_eq!("ffmpeg", args[1][0]);
        assert_eq!("ffmpeg", args[2][0]);
    }

    #[test]
    fn get_cmd_args_empty_file() {
        let file_name = "./testfiles/empty_cmdfile.txt";
        let all_args = get_cmd_args(file_name);
        assert!(all_args.is_err());
    }
    #[test]
    fn get_cmd_args_no_file() {
        let file_name = "./testfiles/nosuchfile.txt";
        let all_args = get_cmd_args(file_name);
        assert!(all_args.is_err());
    }
    #[test]
    fn update_arguments() {
        ////TOP_VID, BOTTOM_VID, OUT_VID

        let args_vec = vec![
            "-flags".to_string(),
            "2".to_string(),
            "-i".to_string(),
            "TOP_VID".to_string(),
            "x".to_string(),
            "BOTTOM_VID".to_string(),
            "OUT_VID".to_string(),
        ];

        let res = update_args_with_substitutions(&args_vec, "top.mp4", "bottom.mp4", "output.mp4")
            .unwrap();
        assert!(res.contains(&"top.mp4".to_string()));
        assert!(res.contains(&"bottom.mp4".to_string()));
        assert!(res.contains(&"output.mp4".to_string()));

        println!("{:?}", res);
    }
}
