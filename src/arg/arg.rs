use clap::{Arg, Command};
use std::error::Error;

static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Default)]
pub struct Argument {
    pub config_file: String,
    pub version_info: String,
}

impl Argument {
    pub fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        let matches = Command::new("vecx")
            .version(VERSION)
            .arg(
                Arg::new("config_file")
                    .short('c')
                    .long("config-file")
                    .value_name("FILE")
                    .help("Config file")
                    .default_value("config.yml")
                    .required(true),
            )
            .get_matches();

        let config_file = matches.get_one::<String>("config_file").unwrap();
        self.config_file = config_file.to_string();

        self.version_info = VERSION.to_string();

        Ok(())
    }
}
