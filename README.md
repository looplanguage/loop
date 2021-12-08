<h1 align="center">Loop</h1>
    <p align="center">
       A Modern Type-Safe Programming Language<br>
       <a href="https://looplang.org/">Website</a> |
       <a href="https://looplang.org/docs">Documentation</a> |
       <a href="https://discord.gg/T3tqQBTyJA">Discord</a> |
       <a href="https://looplang.atlassian.net/jira/software/c/projects/LOOP/issues">Jira Board</a><br>
    </p>
<br>

**Note:** Loop is still in development, we are in the early days in the development. This means that the language is not production ready.

In this repository, you will find the lexer, parser, compiler and virtual
machine.<br>

## Get started (usage)

1. Download a pre-built binary from [downloads.looplang.org](https://downloads.looplang.org)
2. Run `./loop` to start the REPL environment or
3. Run `./loop FILENAME.lp` to run a specific file.

## Get Started (development)

1. Make sure you have [Rust](https://www.rust-lang.org/) installed
1. You also need a working version of LLVM12, with an environment variable "LLVM_SYS_120_PREFIX" pointing towards the build directory
   1. Note: on Windows you can't use the installer provided by LLVM due to needing llvm-config.exe, instead you need to compile it yourself. Or you can use our script to install LLVM on your system automaticly. Paste: `iwr -useb cdn.looplang.org/llvm/install.ps1 | iex` in PowerShell.
1. Clone the repository `git clone https://github.com/looplanguage/loop`
1. Enter the repository `cd loop`
1. Run the command `cargo run`
1. The Loop shell should now popup

Go to our [Jira](https://looplang.atlassian.net/jira/software/c/projects/LOOP/issues) board to see all the issues and tasks to work on.

## Guidelines

If you want to contribute to one of the projects, it is recommenced to read two things: [development guidelines](https://github.com/looplanguage/.github/issues/1) and the [contributor guidelines](https://looplang.org/contributor_guidelines). 

## Code of Conduct

To read our code of conduct, go to the [website](https://looplang.org/conduct).

##

<p align="center">Go to the website for more information.</p>
