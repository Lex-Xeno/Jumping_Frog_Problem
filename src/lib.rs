mod frog_manager;
pub use frog_manager::*;
use std::{
    fs::File,
    io::{ErrorKind, Read},
};

pub fn read_config() -> String {
    let mut config_file = match File::open("config.toml") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                cliclack::log::error("The config file was not found!").unwrap();
                cliclack::outro_cancel("Failed whilst reading the config file.").unwrap();
                std::process::exit(1);
            }
            _ => {
                cliclack::log::error(format!("error: {:?}", error.kind())).unwrap();
                cliclack::outro_cancel("Failed whilst reading the config file.").unwrap();
                std::process::exit(1);
            }
        },
    };

    let mut contents = String::with_capacity(config_file.metadata().unwrap().len() as usize);
    config_file.read_to_string(&mut contents).unwrap();

    contents
}

pub fn parse_config(raw_config: String) -> Config {
    let config = match toml::from_str::<Config>(&raw_config) {
        Ok(data) => data,
        Err(error) => {
            if error.message().contains("missing") {
                cliclack::log::error(error.message()).unwrap();
            } else if let Some(span) = error.span() {
                cliclack::log::error(format!(
                    "{} at characters {} to {}",
                    error.message(),
                    span.start,
                    span.end
                ))
                .unwrap();
            } else {
                cliclack::log::error(format!("{} at an unknown location", error.message()))
                    .unwrap();
            }
            cliclack::outro_cancel("Failed whilst parsing the config file.").unwrap();
            std::process::exit(1);
        }
    };

    config
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub total_frogs: usize,
    pub threads: usize,
}
