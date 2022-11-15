use clap::*;

#[derive(Debug)]
pub struct Args {
    pub client_folder: String,
    pub dummies_folder: String,
    pub output_folder: String,
    pub quantity: usize,
}
impl Args {
    pub fn parse() -> Self {
        let matches = App::new("stackvids")
            .arg(
                Arg::with_name("client_folder")
                    .help("folder of client videos")
                    .long("clients")
                    .short('c')
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("dummies_folder")
                    .help("folder of dummy videos")
                    .long("dummies")
                    .short('d')
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output_folder")
                    .help("folder output videos")
                    .long("output")
                    .short('o')
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("quantity")
                    .help("how many stacks to make for each customer video")
                    .long("quantity")
                    .short('q')
                    .required(true)
                    .takes_value(true),
            )
            .get_matches();

        let client_folder = matches
            .value_of("client_folder")
            .unwrap_or_default()
            .to_string();
        let dummies_folder = matches
            .value_of("dummies_folder")
            .unwrap_or_default()
            .to_string();
        let output_folder = matches
            .value_of("output_folder")
            .unwrap_or_default()
            .to_string();
        let quantity = matches
            .value_of("quantity")
            .unwrap_or_default()
            .to_string()
            .parse::<usize>()
            .unwrap();

        Args {
            client_folder,
            dummies_folder,
            output_folder,
            quantity,
        }
    }
}
