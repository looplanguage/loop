<h1 align="center">Loop</h1>
<p align="center">
   A Modern Type-Safe Programming Language<br>
   <a href="https://looplang.org/">Website</a> |
   <a href="https://looplang.org/docs">Documentation</a> |
   <a href="https://downloads.looplang.org">Downloads</a> |
   <a href="https://discord.gg/T3tqQBTyJA">Discord</a> |
   <a href="https://looplang.atlassian.net/jira/software/c/projects/LOOP/issues">Jira Board</a><br>
</p>

> **Note:** Loop is still in its early days of development, and thus is not production-ready.

## Development Roadmap

Loop 0.2.0 will be released at the beginning of summer 2022. The main goal is to make sure that everything you want
can be written in Loop; These are the developments we will be working on:

 - [ ] Foreign Function Interface to call Rust functions in Loop.
 - [ ] Finalizing compilation to DLang.
 - [ ] Expanding native language features: structs, http, etc.
 - [ ] Beginning of standard library
 - [ ] Significantly improving documentation

## Get started (usage)

1. Download a pre-built binary from [downloads.looplang.org](https://downloads.looplang.org)
2. Run `./loop` to start the REPL environment or
3. Run `./loop FILENAME.lp` to run a specific file.

## Get Started (development)

1. Make sure you have [Rust](https://www.rust-lang.org/) installed
2. You also need a DLang compiler, get one that works on your platform: https://dlang.org/download.html
3. Clone the repository `git clone https://gitlab.com/looplanguage/loop`
4. Enter the repository `cd loop`
5. Run the command `cargo run`
6. The Loop shell should now popup

Go to our [Jira](https://looplang.atlassian.net/jira/software/c/projects/LOOP/issues) board to see all the issues and tasks to work on.

To see code documentation, run the command: `cargo doc --open`. A browser tab will open with generated documentation from code comments.

### Compiler Walkthrough

> **Note:** We are currently working on a technical document that is way more detailed and comprehensive than the one below.

Parsing is the first thing that happens when a Loop program 
is ran. The Lexer converts the input program to tokens and 
the parser parses that to an abstract syntax tree (AST). The 
compiler runs through the AST and generates [DLang](https://github.com/dlang) code, a DLang compiler will compile that to a executable.

### Folder structure

The Loop compiler is fully contained in the `./src` folder

```
loop
|
|--docs
|  |
|  |  adr              Is the archive of all major design and architecture choices
|
|--examples            Contains example programs for Loop
|  |  ....
|
|--scripts             Contains scripts which we might need for e.g CI/CD 
|  |  ....
|
|--tests               Integration tests for the pipeline
|  |  ....
|  |--end2end_test.py  The file to add all the tests to
|  
|--src                 Loop compiler source code
|  |
|  |--compiler
|  |  | ....           Files and folders regarding compiler code
|  |  |
|  |  |--tests         Containing all tests from compiler
|  |
|  |--lexer
|  |  | ....           Files and folders regarding lexer code
|  |  |
|  |  |--tests         Containing all tests from lexer
|  |
|  |--lib
|  |  | ....           "lib" contains all first-party libraries that are used in Loop.
|  |                   The individual folders could be used independently from Loop.
|  |
|  |--parser
|  |  | ....           Files and folders regarding parser code
|  |  |
|  |  |--tests         Containing all tests from parser
|  |
|  |  main.rs          Entry point of compiler
|
|  build.rs            *Compiling non-rust code (DLang compiler)
```
<sup>*Better explanation: https://doc.rust-lang.org/cargo/reference/build-scripts.html</sup>
## Guidelines

If you want to contribute to one of the projects, it is recommenced to read two things: [development guidelines](https://gitlab.com/looplanguage/loop/-/wikis/Loop-Language-Development-Guidelines) and the [contributor guidelines](https://looplang.org/contributor_guidelines). 

## Code of Conduct

To read our code of conduct, go to the [website](https://looplang.org/conduct).

##

<p align="center">Go to the <a href="https://looplang.org/">website</a> for more information.</p>
