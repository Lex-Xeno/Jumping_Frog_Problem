mod frog;
pub mod handler;

pub use handler::*;
use std::{
    fmt::Debug,
    fs::File,
    io::{ErrorKind, Read},
};

/// Reads and parses a config.toml file into a [`Config`]
///
/// # Quits
/// Quits the programme and prints an error if there is an issue with the config file
pub fn get_config() -> Config {
    parse_config(read_config())
}

fn read_config() -> String {
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

    let mut contents = String::new();
    config_file.read_to_string(&mut contents).unwrap();

    contents
}

fn parse_config(raw_config: String) -> Config {
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

pub trait PrettyExpect<T> {
    fn handle(self, message: &str) -> T;
}

impl<T, E: Debug> PrettyExpect<T> for Result<T, E> {
    fn handle(self, message: &str) -> T {
        match self {
            Ok(ok) => ok,
            Err(err) => {
                cliclack::outro_cancel(format!("{}! Error {:?}", message, err))
                    .expect("Cliclack failed to print to stdout!");
                std::process::exit(-1);
            }
        }
    }
}
