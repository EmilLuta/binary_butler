mod config;
mod runner;
use simple_logger::SimpleLogger;

use crate::runner::Runner;

use log;
use std::env;

static USAGE: &str = "Invalid arguments. You need to run `binary_butler --config_path path/to/file` or `binary_butler`";

fn main() {
    SimpleLogger::new().init().unwrap();
    let args: Vec<String> = env::args().collect();
    let runner = match args.len() {
        1 => Runner::new(None),
        3 => {
            if &args[1] != "--config_path" {
                log::error!("{}", USAGE);
                return;
            }
            Runner::new(Some(String::from(&args[2])))
        }
        _ => {
            log::error!("{}", USAGE);
            return;
        }
    };
    runner.run()
}
