use crate::lib::config::LoadType::{FirstRun, Normal};
use crate::lib::exception::runtime::RuntimeException;
use crate::lib::exception::Exception;
use dirs::home_dir;
use serde::Deserialize;
use serde::Serialize;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub enable_telemetry: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enable_telemetry: true,
        }
    }
}

pub enum LoadType {
    FirstRun(Config),
    Normal(Config),
}

pub fn load_config() -> LoadType {
    let mut load = TryLoadConfig { attempts: 0 };

    let config = load.load_config_internal();

    if config.is_err() {
        println!(
            "Unable to load config from (after 3 attempts): {}\\.loop\\config.toml",
            home_dir().unwrap().to_str().unwrap()
        );
        return Normal(Config::default());
    }

    config.ok().unwrap()
}

struct TryLoadConfig {
    attempts: u8,
}

impl TryLoadConfig {
    pub fn load_config_internal(&mut self) -> Result<LoadType, Exception> {
        let home = dirs::home_dir();

        if home.is_none() {
            return Result::Err(Exception::Runtime(RuntimeException::NoHomeFolderDetected));
        }

        let directory = format!("{}\\.loop", home.unwrap().to_str().unwrap());

        let config_file = format!("{}\\config.toml", directory);

        let content = read_to_string(config_file);

        if content.is_err() {
            if self.attempts > 2 {
                return Result::Err(Exception::Runtime(RuntimeException::UnableToReadFile(
                    content.err().unwrap(),
                )));
            }

            let dir = directory;
            let path = Path::new(dir.as_str());
            let err = create_dir_all(path);
            if err.is_err() {
                return Result::Err(Exception::Runtime(RuntimeException::UnableToWriteFile(
                    err.err().unwrap(),
                )));
            }

            let file = File::create(Path::new(format!("{}\\config.toml", dir.clone()).as_str()));

            if file.is_err() {
                return Result::Err(Exception::Runtime(RuntimeException::UnableToWriteFile(
                    content.err().unwrap(),
                )));
            }

            let config_content = toml::to_string(&Config::default()).unwrap();
            let err = file.unwrap().write_all(config_content.as_bytes());
            if err.is_err() {
                return Result::Err(Exception::Runtime(RuntimeException::UnableToWriteFile(
                    err.err().unwrap(),
                )));
            }

            self.attempts += 1;

            return self.load_config_internal();
        }

        let config: Config = toml::from_str(content.ok().unwrap().as_str()).unwrap();

        if self.attempts > 0 {
            return Result::Ok(FirstRun(config));
        }

        Result::Ok(Normal(config))
    }
}
