use crate::VideoError;
use std::process::Command;

//pub const COMMANDS_FILE: &str = "ffmpeg_commands.txt";
#[derive(Debug)]
pub struct VideoCommand {
    cmd: Command,
    cmd_name: String, // might be useful even tho it's encoded into the Command.
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

        for arg in args {
            let cmd = cmd.arg(arg);
        }

        let vc = VideoCommand { cmd, cmd_name };
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
#[cfg(test)]
mod tests {
    use super::*;

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
}
