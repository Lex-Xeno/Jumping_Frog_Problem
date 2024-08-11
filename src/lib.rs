pub mod frog_manager;
use std::{
    fs::File,
    io::{ErrorKind, Read},
};

const PRINT_ERROR: &str = "Cliclack failed to print to stdout";

pub fn read_config() -> String {
    let read_prog = cliclack::spinner();
    read_prog.start("Reading file 'config.toml'...");
    let mut config_file = match File::open("config.toml") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                read_prog.error("Reading file 'config.toml' failed!");

                cliclack::log::error("The config file was not found!").expect(PRINT_ERROR);
                cliclack::outro_cancel("Failed whilst reading the config file.")
                    .expect(PRINT_ERROR);
                std::process::exit(1);
            }
            _ => {
                read_prog.error("Reading file 'config.toml' failed!");

                cliclack::log::error(format!("error: {:?}", error.kind())).expect(PRINT_ERROR);
                cliclack::outro_cancel("Failed whilst reading the config file.")
                    .expect(PRINT_ERROR);
                std::process::exit(1);
            }
        },
    };

    let mut contents = String::with_capacity(config_file.metadata().unwrap().len() as usize);
    config_file.read_to_string(&mut contents).unwrap();
    read_prog.stop("File 'config.toml' red successfully!");

    contents
}

pub fn parse_config(raw_config: String) -> Config {
    let parse_prog = cliclack::spinner();
    parse_prog.start("Parsing file 'config.toml'...");
    let config = match toml::from_str::<Config>(&raw_config) {
        Ok(data) => data,
        Err(error) => {
            parse_prog.error("Parsing file 'config.toml' failed!");

            if error.message().contains("missing") {
                cliclack::log::error(error.message()).expect(PRINT_ERROR);
            } else if let Some(span) = error.span() {
                cliclack::log::error(format!(
                    "{} at characters {} to {}",
                    error.message(),
                    span.start,
                    span.end
                ))
                .expect(PRINT_ERROR);
            } else {
                cliclack::log::error(format!("{} at an unknown location", error.message()))
                    .expect(PRINT_ERROR);
            }
            cliclack::outro_cancel("Failed whilst parsing the config file.").expect(PRINT_ERROR);
            std::process::exit(1);
        }
    };
    parse_prog.stop("File 'config.toml' parsed successfully!");

    config
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub group_size: usize,
    pub total_groups: usize,
}
