use crate::config::Config;
use is_executable::IsExecutable;
use std::io::Error;
use std::{thread, time, fs};
use walkdir::WalkDir;
use content_inspector::{inspect, ContentType};

pub struct Runner {
    config: Config,
}

impl Runner {
    pub fn new(config: Config) -> Runner {
        Runner { config }
    }

    fn sweep(&self) -> Result<bool, Error> {
        for entry in WalkDir::new(&self.config.sweep_directory)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().is_executable())
            .filter(|entry| inspect(&fs::read(entry.path().to_owned()).unwrap()) == ContentType::BINARY)
        {
            match fs::remove_file(entry.path()) {
                Ok(result) => {println!("Removed {:#?} with result {:#?}", entry.path(), result);},
                Err(e) => {println!("Failed to remove {:#?} due to {:#?}", entry.path(), e);},
            }
        }
        Ok(true)
    }
    pub fn run(&self) {
        loop {
            match self.sweep() {
                Ok(_) => {}
                Err(e) => panic!("{}", e.to_string()),
            }
            thread::sleep(time::Duration::from_millis(
                self.config.interval_seconds * 1000,
            ));
        }
    }
}
