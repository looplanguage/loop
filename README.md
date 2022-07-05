<h1 align="center">Loop</h1>
<p align="center">

Bringing Cloud Tools to a “Provider-less” Programming Language<br>
<a href="https://looplang.org/">Website</a> |
<a href="https://docs.looplang.org">Documentation</a> |
<a href="https://downloads.looplang.org">Downloads</a> |
<a href="https://discord.gg/T3tqQBTyJA">Discord</a> |
<a href="https://looplang.atlassian.net/jira/software/c/projects/LOOP/issues">Jira Board</a><br>
</p>

> **Note:** Loop is still in its early days of development, and thus is not production-ready.   1

## Get started (usage)

1. Download a pre-built binary from [downloads.looplang.org](https://downloads.looplang.org)
2. Run `./loop` to start the REPL environment or
3. Run `./loop FILENAME.loop` to run a specific file.

## Get Started (development)

1. Make sure you have [Rust](https://www.rust-lang.org/) installed
2. Clone the repository `git clone https://gitlab.com/looplanguage/loop`
3. Enter the repository `cd loop`
4. Run the command `cargo run`
5. The Loop shell should now popup

Go to our [Jira](https://looplang.atlassian.net/jira/software/c/projects/LOOP/issues) board to see all the issues and tasks to work on.

To see code documentation, run the command: `cargo doc --open`. A browser tab will open with generated documentation from code comments. You can also go to: https://docs.looplang.org/internal to find more architectural documentation of the interpreter.

### Project structure

Below you will find the project structure for Loop, to find a more detailed explanation of the interpreter itself go the [internal documentation](https://docs.looplang.org/internal/architecture).

- `examples/*`: Examples of Loop code as reference
- `script/*`: Scripts to use in I.E. the pipeline
- `tests/*`: End2End tests for Loop
- `src/lib/*`: Utility functions, library-esque, rutime code
- `src/picasso/*`: The crate which compiles Loop to Arc
- `src/vinci/*`: The crate to parse Arc code
- `src/sanzio/*`: The to-lua-compiler and interpreter crate


## Guidelines

If you want to contribute to one of the projects, it is recommenced to read two things: [development guidelines](https://gitlab.com/looplanguage/loop/-/wikis/Loop-Language-Development-Guidelines) and the [contributor guidelines](https://looplang.org/contributor_guidelines).

## Code of Conduct

To read our code of conduct, go to the [website](https://looplang.org/conduct).

##

<p align="center">Go to the <a href="https://looplang.org/">website</a> for more information.</p>
