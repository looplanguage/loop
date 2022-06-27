mod lib;

use crate::lib::util::get_flags;
use std::process::ExitCode;

fn main() -> ExitCode {
    let flags = get_flags();

    if let Some(file) = flags.file {
        if let Err(code) = lib::util::run_file(file) {
            return code;
        }
    } else if let Err(code) = lib::repl::start() {
        return code;
    }

    ExitCode::SUCCESS
}
