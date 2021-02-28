mod config;
mod runner;
use simple_logger::SimpleLogger;

use crate::config::Config;
use crate::runner::Runner;

use log;
use std::env;

static USAGE: &str = "Invalid arguments. You need to run `binary_butler --config_path path/to/file` or `binary_butler`";

fn main() {
    SimpleLogger::new().init().unwrap();
    let args: Vec<String> = env::args().collect();
    let config = match args.len() {
        1 => Config::new(None).unwrap(),
        3 => {
            if &args[1] != "--config_path" {
                log::error!("{}", USAGE);
                return;
            }
            Config::new(Some(&args[2])).unwrap()
        }
        _ => {
            log::error!("{}", USAGE);
            return;
        }
    };
    let runner: Runner = Runner::new(config);
    runner.run()
}
