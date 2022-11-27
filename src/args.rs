use crate::VideoError;
use clap::*;
use serde_derive::Deserialize;
use std::fmt::{Display, Formatter};
use std::fs;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// does testing things
    File {
        /// lists test values
        #[arg(short, long)]
        name: String,
    },
    Values {
        #[clap(short, long)]
        client_folder: String,
        #[clap(short, long)]
        dummies_folder: String,
        #[clap(short, long)]
        output_folder: String,
        #[clap(short, long)]
        quantity: usize,
        #[clap(short, long)]
        index_of_command: usize,
    },
}

#[derive(Deserialize, Parser, Default, Debug)]
#[clap(author = "BanditPig", version, about)]
/// Utility to vertically stack two video files using ffmpeg.
pub struct Arguments {
    #[clap(short, long)]
    pub client_folder: String,
    #[clap(short, long)]
    pub dummies_folder: String,
    #[clap(short, long)]
    pub output_folder: String,
    #[clap(short, long)]
    pub quantity: usize,
    #[clap(short, long)]
    pub index_of_command: usize,
}
impl Display for Arguments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " Client folder {}\n Dummies folder {}\n Output folder {}\n Quantity {}\n Command index {}\n",
               self.client_folder, self.dummies_folder, self.output_folder, self.quantity, self.index_of_command)
    }
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub args: Arguments,
}
impl Configuration {
    pub fn new(name: &str) -> Result<Self, VideoError> {
        let contents = fs::read_to_string(name)?;
        let config: Configuration = toml::from_str(&contents)?;
        Ok(config)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use claim::*;
    #[test]
    fn config_ok() {
        let config = Configuration::new("testfiles/dummyconfig/good.toml");
        assert_ok!(&config);

        let cfg = config.unwrap().args;
        assert_eq!(cfg.client_folder, "client");
        assert_eq!(cfg.dummies_folder, "dummies");
        assert_eq!(cfg.output_folder, "output");
        assert_eq!(cfg.quantity, 1);
        assert_eq!(cfg.index_of_command, 0);
    }
    #[test]
    fn config_no_file() {
        let config = Configuration::new("NOFILE");
        assert_err!(&config);
    }

    #[test]
    fn config_missing_args_section() {
        let config = Configuration::new("testfiles/dummyconfig/no_args_section.toml");
        assert_err!(&config);
    }
    #[test]
    fn config_syntax_error() {
        let config = Configuration::new("testfiles/dummyconfig/bad_syntax.toml");
        assert_err!(&config);
    }
    #[test]
    fn config_missing_args_error() {
        let config = Configuration::new("testfiles/dummyconfig/missing_args.toml");
        assert_err!(&config);
    }
}
