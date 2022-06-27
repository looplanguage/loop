mod lib;

use crate::lib::util::get_flags;
use std::process::ExitCode;

fn main() -> Result<(), ExitCode> {
    let flags = get_flags();

    if let Some(file) = flags.file {
        lib::util::run_file(file)
    } else {
        lib::repl::start()
    }
}
