use colored::*;

pub fn throw_runtime_exception(message: String, extra_message: Option<String>) {
    println!("{}", "RuntimeError -->".bright_red());
    println!(
        "\t{}: {}",
        "Message".bright_blue(),
        format!("{}\n", message).bright_white()
    );

    if extra_message.is_some() {
        println!(
            "\t{}: {}",
            "Note".bright_blue(),
            format!("{}\n", extra_message.unwrap()).bright_white()
        );
    }
    std::process::exit(1)
}
