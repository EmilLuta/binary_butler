mod config;
mod runner;

use crate::config::Config;
use crate::runner::Runner;

fn main() {
    let config = Config::new().unwrap();
    println!("{}", config);
    let runner: Runner = Runner::new(config);
    runner.run()
}
