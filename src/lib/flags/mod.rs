//! Flag library for CLI arguments passed to Loop
use crate::lib::config::ConfigInternal;
use crate::lib::exception::flag;
use std::ffi::OsStr;
use std::path::Path;
use std::process;
mod arc;
mod benchmark;
mod debug;
mod help;
mod lua;
mod optimize;

/// Value: `true` -> is enabled<br>
/// Value: `false` -> is not enabled<br>
/// Value: `None` -> unspecified<br>
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum FlagTypes {
    Help(String), // When user does -h or --help it gives all the flags and closes program
    File(String),
    Debug(Option<bool>),
    Lua(Option<bool>),
    Arc(Option<bool>),
    Benchmark(Option<bool>),
    Optimize(Option<bool>), // There are no optimizations yet, this is for the near future
}

pub fn build_flags() -> Flags {
    Flags {
        flags: ConfigInternal {
            enable_telemetry: None,
            debug_mode: None,
            lua_output: None,
            arc_output: None,
            enable_benchmark: None,
            enable_optimize: None,
        },
        file: None,
    }
}

pub struct Flags {
    pub flags: ConfigInternal,
    pub file: Option<String>,
}

impl Flags {
    pub fn parse_flags(&mut self, args: Vec<String>) -> i32 {
        let mut i: i32 = 0;
        for arg in args.clone() {
            let flag = Flags::get_flag(self, arg.as_str(), arg.eq(args.last().unwrap()));
            if let Ok(e) = flag {
                match e {
                    FlagTypes::Optimize(b) => self.flags.enable_optimize = b,
                    FlagTypes::Debug(b) => self.flags.debug_mode = b,
                    FlagTypes::Benchmark(b) => self.flags.enable_benchmark = b,
                    FlagTypes::Lua(b) => self.flags.lua_output = b,
                    FlagTypes::Arc(b) => self.flags.arc_output = b,
                    FlagTypes::File(f) => self.file = Some(f),
                    FlagTypes::Help(h) => {
                        // Prints help message and exists program
                        println!("{}", h);
                        process::exit(0);
                    }
                }
            }
            i += 1;
        }

        i
    }

    fn get_flag(&mut self, string: &str, is_last: bool) -> Result<FlagTypes, ()> {
        let flag_arguments: Vec<&str> = string.split('=').collect();

        if flag_arguments.len() > 2 {
            // Program quits
            flag::throw_exception_value(string.to_string());
        }
        if flag_arguments.len() == 2 {
            return match flag_arguments[0] {
                "--debug" | "-d" => debug::debug_flag_with_param(flag_arguments[1]),
                "--benchmark" | "-b" => benchmark::benchmark_flag_with_param(flag_arguments[1]),
                "--optimize" | "-o" => optimize::optimize_flag_with_param(flag_arguments[1]),
                _ => self.handle_unknown_flag(string.to_string(), is_last),
            };
        }
        match flag_arguments[0] {
            "--debug" | "-d" => debug::debug_flag(),
            "--benchmark" | "-b" => benchmark::benchmark_flag(),
            "--optimize" | "-o" => optimize::optimize_flag(),
            "--lua" => lua::lua_flag(),
            "--arc" => arc::arc_flag(),
            "--help" => {
                if let Ok(e) = help::generate_help_text() {
                    return Ok(FlagTypes::Help(e));
                }
                // Will never be reached, because: "help::generate_help_text()", will always return Ok()
                Err(())
            }
            _ => self.handle_unknown_flag(string.to_string(), is_last),
        }
    }

    fn handle_unknown_flag(&self, string: String, is_last: bool) -> Result<FlagTypes, ()> {
        if !is_last {
            flag::throw_exception_unknown_flag(string);
            return Err(());
        }
        let ext = Path::new(string.as_str())
            .extension()
            .and_then(OsStr::to_str);
        if ext.is_some() && (ext.unwrap() == "loop" || ext.unwrap() == "lp") {
            return Ok(FlagTypes::File(string.to_string()));
        }
        // Program quits, will never reach the Err return
        flag::throw_exception_unknown_flag(string.to_string());
        Err(())
    }
}
