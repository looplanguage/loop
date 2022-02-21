<h1 align="center">Loop</h1>
<p align="center">
   A Modern Type-Safe Programming Language<br>
   <a href="https://looplang.org/">Website</a> |
   <a href="https://looplang.org/docs">Documentation</a> |
   <a href="https://downloads.looplang.org">Downloads</a> |
   <a href="https://discord.gg/T3tqQBTyJA">Discord</a> |
   <a href="https://looplang.atlassian.net/jira/software/c/projects/LOOP/issues">Jira Board</a><br>
</p>

> **Note:** Loop is still in development, we are in the early days in the development. This means that the language is not production ready.

## Development Roadmap

For the second release of Loop 0.2.0, this version will be released at the beginning of the summer 2022. 
We want to make sure that everything you can do everything in Loop. 
This will require a couple of developments, these are listed below:

 -[ ] Foreign Function Interface to call Rust functions in Loop.
 -[ ] Finalizing JIT-compiler.
 -[ ] Expanding native language features: structs, http, etc.

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

### Compiler Walkthrough

> **Note:** We are currently working on a technical document that is way more detailed and comprehensive than the one below.

Parsing is the first thing that happens when a Loop program 
is ran. The Lexer converts the input program to tokens and 
the parser parses that to an abstract syntax tree (AST). The 
compiler runs through the AST and generates bytecode. 
That bytecode will either be interpreted by the virtual 
machine or it will be JIT-compiler to machine code and 
directly executed.

### Folder structure

The Loop compiler is fully contained in the `/src` folder

```
loop
|  
|--examples            contains example programs for Loop
|  |  ....
|
|--scripts             contains scripts which we might need for e.g CI/CD 
|  |  ....
|
|--src                 Loop compiler source code
   |  main.rs          Entry point of compiler
   |
   |--compiler
   |  | ....           Files and folders regarding compiler code
   |  |
   |  |--tests         Containing all tests from compiler
   |
   |--lexer
   |  | ....           Files and folders regarding lexer code
   |  |
   |  |--tests         Containing all tests from lexer
   |
   |--lib
   |  | ....           "lib" contains all first-party libraries that are used in Loop.
   |                   The individual folders could be used independently from Loop.
   |
   |--parser
   |  | ....           Files and folders regarding parser code
   |  |
   |  |--tests         Containing all tests from parser
   |
   |--vm
      | ....           Files and folders regarding compiler code
      |
      |--tests         Containing all tests from compiler
    
```

## Guidelines

If you want to contribute to one of the projects, it is recommenced to read two things: [development guidelines](https://gitlab.com/looplanguage/loop/-/wikis/Loop-Language-Development-Guidelines) and the [contributor guidelines](https://looplang.org/contributor_guidelines). 

## Code of Conduct

To read our code of conduct, go to the [website](https://looplang.org/conduct).

##

<p align="center">Go to the website for more information.</p>
