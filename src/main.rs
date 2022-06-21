mod lib;

use std::process::Command;
use crate::lib::util::get_flags;

fn main() {
    let flags = get_flags();

    if let Some(file) = flags.file {
        lib::util::run_file(file);
    } else {
        lib::repl::start();
    }

}
