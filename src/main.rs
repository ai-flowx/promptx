mod arg;
mod config;
mod llm;
mod logger;
mod optimizer;

use arg::arg::Argument;
use config::config::Config;
use llm::llm::LLM;
use logger::logger::Logger;
use optimizer::optimizer::Optimizer;
use std::process;

fn main() {
    let mut a = Argument {
        ..Default::default()
    };

    if let Err(err) = a.parse() {
        println!("failed to parse argument: {}", err);
        process::exit(-1);
    }

    let mut c = Config {
        config_file: a.config_file,
        version_info: a.version_info,
        ..Default::default()
    };

    if let Err(err) = c.build() {
        println!("failed to build config: {}", err);
        process::exit(-2);
    }

    let _ = LLM::new(c.config_data.clone());
    let _ = Logger::new("");
    let _ = Optimizer::new(c.config_data.clone());
}
