//! Persistent configuration library that is for Loop itself
use crate::get_flags;
use crate::lib::config::LoadType::{FirstRun, Normal};
use crate::lib::exception::runtime::RuntimeException;
use crate::lib::exception::Exception;
use dirs::home_dir;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::Path;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let cfg = match load_config() {
        FirstRun(cfg) => cfg,
        Normal(cfg) => cfg,
    };

    let flags = get_flags();

    // Setting up the config
    let mut config: Config = Config {
        // Currently no telemetry in Loop, kind of redundant
        enable_telemetry: cfg.enable_telemetry.unwrap_or(false),
        debug_mode: false,
        enable_benchmark: false,
        enable_optimize: false,
    };

    // The flags go over the config file.
    // If config has: "jit_enabled = true" and flag has: "jit_enabled = false",
    // Than the config will disable JIT

    if flags.flags.debug_mode.is_some() {
        config.debug_mode = flags.flags.debug_mode.unwrap();
    } else if cfg.debug_mode.is_some() {
        config.debug_mode = cfg.debug_mode.unwrap();
    }

    if flags.flags.enable_benchmark.is_some() {
        config.enable_benchmark = flags.flags.enable_benchmark.unwrap();
    } else if cfg.enable_benchmark.is_some() {
        config.enable_benchmark = cfg.enable_benchmark.unwrap();
    }

    if flags.flags.enable_optimize.is_some() {
        config.enable_optimize = flags.flags.enable_optimize.unwrap();
    } else if cfg.enable_optimize.is_some() {
        config.enable_optimize = cfg.enable_optimize.unwrap();
    }

    config
});

pub struct Config {
    pub enable_telemetry: bool,
    pub debug_mode: bool,
    pub enable_benchmark: bool,
    pub enable_optimize: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ConfigInternal {
    pub enable_telemetry: Option<bool>,
    pub debug_mode: Option<bool>,
    pub enable_benchmark: Option<bool>,
    pub enable_optimize: Option<bool>,
}

impl Default for ConfigInternal {
    fn default() -> Self {
        ConfigInternal {
            enable_telemetry: Some(false),
            debug_mode: Some(false),
            enable_benchmark: Some(false),
            enable_optimize: Some(false),
        }
    }
}

pub enum LoadType {
    FirstRun(ConfigInternal),
    Normal(ConfigInternal),
}

pub fn load_config() -> LoadType {
    let mut load = TryLoadConfig { attempts: 0 };

    let config = load.load_config_internal();

    if config.is_err() {
        println!(
            "Unable to load config from (after 3 attempts): {}/.loop/config.toml",
            home_dir().unwrap().to_str().unwrap()
        );
        return Normal(ConfigInternal::default());
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

        let directory = format!("{}/.loop", home.unwrap().to_str().unwrap());

        let config_file = format!("{}/config.toml", directory);

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

            let file = File::create(Path::new(format!("{}/config.toml", dir.clone()).as_str()));

            if file.is_err() {
                return Result::Err(Exception::Runtime(RuntimeException::UnableToWriteFile(
                    content.err().unwrap(),
                )));
            }

            let config_content = toml::to_string(&ConfigInternal::default()).unwrap();
            let err = file.unwrap().write_all(config_content.as_bytes());
            if err.is_err() {
                return Result::Err(Exception::Runtime(RuntimeException::UnableToWriteFile(
                    err.err().unwrap(),
                )));
            }

            self.attempts += 1;

            return self.load_config_internal();
        }

        let config: ConfigInternal = toml::from_str(content.ok().unwrap().as_str()).unwrap();

        if self.attempts > 0 {
            return Result::Ok(FirstRun(config));
        }

        Result::Ok(Normal(config))
    }
}
