use crate::VideoError;
use std::ops::Index;
use std::process::Command;

//pub const COMMANDS_FILE: &str = "ffmpeg_commands.txt";
#[derive(Debug)]
pub struct VideoCommand {
    cmd: Command,
    cmd_args_vec: Vec<String>,
    cmd_name: String, // might be useful even tho it's encoded into the Command.
}

impl VideoCommand {
    ///Substitute values for TOP_VID, BOTTOM_VID, OUTVID in the
    /// After doing this the vector returned can then be used in add_arguments_to_command
    /// putting the command in a state where its fit to run.
    pub fn update_args_with_substitutions(&mut self, tv: &str, bv: &str, ov: &str) -> &Vec<String> {
        //TOP_VID, BOTTOM_VID, OUTVID

        let ix = self
            .cmd_args_vec
            .iter()
            .position(|s| s == "TOP_VID")
            .unwrap(); //Should handle this error!

        let value = std::mem::replace(&mut self.cmd_args_vec[ix], tv.to_string());
        let ix = self
            .cmd_args_vec
            .iter()
            .position(|s| s == "BOTTOM_VID")
            .unwrap();
        let value = std::mem::replace(&mut self.cmd_args_vec[ix], bv.to_string());

        let ix = self
            .cmd_args_vec
            .iter()
            .position(|s| s == "OUTVID")
            .unwrap();
        let value = std::mem::replace(&mut self.cmd_args_vec[ix], ov.to_string());

        &self.cmd_args_vec //Is this OK??? or should I not return a ref and just clone vec?
    }
    pub fn add_arguments_to_command(&mut self, mut cmd: Command, args: &Vec<String>) -> Command {
        for arg in args {
            let cmd = cmd.arg(arg);
        }
        cmd
    }
}

pub fn all_commands(cmds_file_name: &str) -> Result<Vec<VideoCommand>, VideoError> {
    let all_args = get_cmd_args(cmds_file_name)?;
    let mut cmds: Vec<VideoCommand> = vec![];

    //for each vector of arguments - i.e. strings
    for mut args in all_args {
        //make a command and set its arguments.
        //Command name is first and is usually ffmpeg
        let cmd_name = args.remove(0);
        let mut cmd = Command::new(&cmd_name);
        //
        // for arg in args {
        //     let cmd = cmd.arg(arg);
        // }

        let vc = VideoCommand {
            cmd,
            cmd_args_vec: args,
            cmd_name,
        };
        cmds.push(vc);
    }
    Ok(cmds)
}

///Each ffmpeg command is on one line and seperated by ' '.
///
fn get_cmd_args(cmds_file_name: &str) -> Result<Vec<Vec<String>>, VideoError> {
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
    fn load_commands() {
        let file_name = "./testfiles/good_cmdfile.txt";
        let cmds = all_commands(file_name).unwrap();
        assert_eq!(cmds.len(), 3);
        assert_eq!(cmds[0].cmd_name, "ffmpeg");
        assert_eq!(cmds[1].cmd_name, "ffmpeg");
        assert_eq!(cmds[2].cmd_name, "ffprobe");
    }
    #[test]
    fn load_commands_empty_file() {
        let file_name = "./testfiles/empty_cmdfile.txt";
        let cmds = all_commands(file_name).unwrap();
        assert_eq!(cmds.len(), 0);
    }
    #[test]
    fn load_commands_no_file() {
        let file_name = "./testfiles/nosuchfile.txt";
        let cmds = all_commands(file_name);
        assert!(cmds.is_err(), "The file should not exist");
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

        let mut vc = VideoCommand {
            cmd: Command::new("dummy"),
            cmd_args_vec: args_vec,
            cmd_name: "".to_string(),
        };
        let res = vc.update_args_with_substitutions("top.mp4", "bottom.mp4", "output.mp4");
        assert!(res.contains(&"top.mp4".to_string()));
        assert!(res.contains(&"bottom.mp4".to_string()));
        assert!(res.contains(&"output.mp4".to_string()));

        println!("{:?}", res);
    }
}
