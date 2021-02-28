use crate::config::Config;
use content_inspector::{inspect, ContentType};
use is_executable::IsExecutable;
use std::io::Error;
use std::time::{Duration, SystemTime};
use std::{fs, thread, time};
use walkdir::WalkDir;

pub struct Runner {
    config: Config,
}

impl Runner {
    pub fn new(config: Config) -> Runner {
        Runner { config }
    }

    fn sweep(&self) -> Result<i64, Error> {
        let ttl_duration = Duration::new(self.config.ttl_seconds, 0);
        let mut files_removed: i64 = 0;
        for entry in WalkDir::new(&self.config.sweep_directory)
            .into_iter()
            // TODO: Maybe log some errors if we have errors?
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().is_executable())
            .filter(|entry| {
                inspect(&fs::read(entry.path().to_owned()).unwrap()) == ContentType::BINARY
            })
            .filter(|entry| {
                let modified = entry.metadata().unwrap().modified().unwrap();
                SystemTime::now().duration_since(modified).unwrap() >= ttl_duration
            })
        {
            match fs::remove_file(entry.path()) {
                Ok(result) => {
                    log::info!("Removed {:#?} with result {:#?}", entry.path(), result);
                    files_removed += 1;
                }
                Err(e) => {
                    println!("Failed to remove {:#?} due to {:#?}", entry.path(), e);
                }
            }
        }
        Ok(files_removed)
    }
    pub fn run(&self) {
        loop {
            match self.sweep() {
                Ok(files_removed) => {
                    log::info!("Removed {} files", files_removed);
                }
                Err(e) => panic!("{}", e.to_string()),
            }
            thread::sleep(time::Duration::from_millis(
                self.config.interval_seconds * 1000,
            ));
        }
    }
}
