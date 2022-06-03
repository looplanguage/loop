use colored::*;
use std::process;

/// Throws an error about the passed flags
///
/// # Template
/// <pre>
/// Unknown Flag -->
///     Flag "-awdaw" is unexpected in this context
/// </pre>
pub fn throw_exception_unknown_flag(unexpected_flag: String) {
    println!("{}", "FlagException --> ".bright_red());
    println!(
        "    {}",
        format!(
            "Flag \"{}\" is unexpected in this context\n",
            unexpected_flag
        )
        .bright_white()
    );

    process::exit(1);
}

pub fn throw_exception_value(flag: String) {
    println!("{}", "FlagException --> ".bright_red());
    println!(
        "    {}",
        format!("This flag can only have one value. Passed flag: {}", flag).bright_white()
    );

    process::exit(1);
}

pub fn throw_exception_unexpected_value(flag: String) {
    println!("{}", "FlagException --> ".bright_red());
    println!(
        "    {}",
        format!("Unexpected flag value: {}", flag).bright_white()
    );

    process::exit(1);
}
