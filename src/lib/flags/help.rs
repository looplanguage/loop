/// Generates this text to print in terminal:
///
/// # Message
/// <pre>
/// Loop Programming Language
///
/// Usage:
///     loop [FLAGS] [FILEPATH]
///
/// Flags:
///     --debug     | -d  ->  Enables debug mode in Loop
///     --benchmark | -b  ->  To time a programs execution
///     --jit       | -j  ->  [UNFINISHED] Enables the Just-In-Time compiler
///     --optimize  | -o  ->  [UNFINISHED] Enables compiled optimisations
///     --help            ->  Prints this helping text
/// </pre>
pub fn generate_help_text() -> Result<String, ()> {
    let mut text = "Loop Programming Language\n\n".to_string();
    text.push_str("Usage:\n");
    text.push_str("    loop [FLAGS] [FILEPATH]\n\n");
    text.push_str("Flags:\n");
    text.push_str("    --debug     | -d  ->  Enables debug mode in Loop\n");
    text.push_str("    --benchmark | -b  ->  To time a programs execution\n");
    text.push_str("    --optimize  | -o  ->  [UNFINISHED] Enables compiled optimisations\n");
    text.push_str("    --lua             ->  Saves the generated lua code to <your_path>.lua\n");
    text.push_str("    --arc             ->  Saves the generated lua code to <your_path>.arc\n");
    text.push_str("    --help            ->  Prints this helping text\n\n");
    text.push_str("For more info go to: https://looplang.org\n");
    Ok(text)
}
