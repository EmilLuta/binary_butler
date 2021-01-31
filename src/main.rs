mod config;
mod runner;

use crate::config::Config;
use crate::runner::Runner;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match args.len() {
        1 => Config::new(None).unwrap(),
        3 => {
            if &args[1] != "--config_path" {
                println!("Invalid arguments. You need to run binary_butler --config_path path/to/file or binary_butler");
                return;
            }
            Config::new(Some(&args[2])).unwrap()
        },
        _ => {
            println!("Invalid arguments. You need to run binary_butler --config_path path/to/file or binary_butler");
            return;
        }
    };
    println!("{}", config);
    let runner: Runner = Runner::new(config);
    runner.run()
}
