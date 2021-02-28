use serde::Deserialize;
use std::env;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::{error, fmt};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sweep_directory: String,
    pub log_directory: String,
    pub interval_seconds: u64,
    pub ttl_seconds: u64,
}

impl Config {
    const CONFIG_FILE: &'static str = "config.toml";
    const CONFIG_DIRECTORY: &'static str = "binary_butler";

    fn get_config_path() -> String {
        match env::var("XDG_CONFIG_HOME") {
            Ok(val) => {
                let path = shellexpand::full(&val).unwrap();
                return Path::new(&path.to_string())
                    .join(Config::CONFIG_DIRECTORY)
                    .join(Config::CONFIG_FILE)
                    .to_str()
                    .unwrap()
                    .to_string();
            }
            Err(_) => {}
        };
        match env::var("HOME") {
            Ok(val) => {
                let path = shellexpand::full(&val).unwrap();
                Path::new(&path.to_string()).join(".config").join(Config::CONFIG_DIRECTORY).join(Config::CONFIG_FILE).to_str().unwrap().to_string()
            },
            Err(_) => panic!("Can't find $XDG_CONFIG_HOME or $HOME. Don't know where to source config file from."),
        }
    }

    pub fn new(config_path: Option<&str>) -> Result<Config, ConfigError> {
        let config_file = match config_path {
            Some(path) => path.to_string(),
            None => Config::get_config_path(),
        };
        log::debug!("Using config_file path: {:#?}", config_file);
        let file_contents = fs::read_to_string(&config_file).unwrap();
        let mut config: Config = toml::from_str(&file_contents).unwrap();
        config.sweep_directory = shellexpand::full(&config.sweep_directory)
            .unwrap()
            .to_string();
        config.log_directory = shellexpand::full(&config.log_directory)
            .unwrap()
            .to_string();
        if !Path::new(&config.sweep_directory).exists() {
            panic!(
                "There's no file sweep_directory: {}",
                config.sweep_directory
            );
        }
        if !Path::new(&config.log_directory).exists() {
            panic!("There's no file log_directory: {}", config.log_directory);
        }
        log::debug!("Config loaded: {:#?}", config);
        Ok(config)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config: {{\n\tsweep_directory: {},\n\tlog_directory: {},\n\tinterval_seconds: {},\n\tttl_seconds: {}\n}}", self.sweep_directory, self.log_directory, self.interval_seconds, self.ttl_seconds)
    }
}

#[derive(Debug)]
pub struct ConfigError {
    message: String,
}

impl ConfigError {}

impl error::Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConfigError: {}", &self.message)
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> ConfigError {
        ConfigError {
            message: e.to_string(),
        }
    }
}
