# End2end Tests

To keep everything working in Loop we make use of tests. Most of these are written in the source code, and can we run like this: `cargo test`. Some tests are not really possible, tedious or are just better to write end2end tests for. This directory contains all the end2end tests for Loop. 

Just for clarification, these are the three types of tests we use:
 - Unit-test, tests a small unit of code
 - Integration-test, tests the whole stack of the source-code (lexer to compiler/vm)
 - End2End-test, tests by simulating user behaviour (running *.loop files)

## Run Locally

It is meant to be ran in the pipeline, but you can be ran locally too. Here is how to do it:
1. Install [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), [Python3.x](https://www.python.org/downloads/) and [LLVM:12.0.0](https://releases.llvm.org/)
2. Clone the repository: `git clone https://gitlab.com/looplanguage/loop.git`
3. `cd loop`
4. Run: `python3 tests/end2end_test.py`

You can add `-v` after the end of the command to print the errors if they occur.

The results of the tests will be printed inside the terminal.

## Add Tests

When adding a tests you have to do two things: Add a loop file with your code and add the file + results in `end2end_test.py` (`testlib.py` can be ignored, it contains all the logic for testing.)

### Writing The Test

A test can be whatever is needed, but lets pick a return in a function as an easy example test. The code would look something like this:

> **Note:** Currently you can only compare strings to check whether a test is successful, this will be expanded in the future

```
var fun = fn(x) {
    return x
}
fun(5)
```

You save the file in the folder `/tests`. The filename should start with `test` and and with `.loop`, in between you give a short but descriptive name. In this case something like: `test_function_return.loop` would suit well.

> **Note:** Try not the create a sub-directory in the `/tests` directoryfireclar
> 


Now that you have written the code for the test it needs to be added.

### Adding to Test List

In the Python script called: `end2end_test.py` there is a imported function called `add_test()`. It expects two arguments: 1. The name of the test file (`test_function_return.loop` is our example), 2. The expected value ('5' in our case). In our case you endup with something like this:
```python
add_test("test_comments.loop", "3")
add_test("test_import_export.loop", "8")
add_test("test_function_return.loop", "5")
```

This is everything you have to do to write and add a test.

##

<p align="center">
       <a href="https://looplang.org/">Website</a> | 
       <a href="https://looplang.org/docs">Documentation</a> | 
       <a href="https://downloads.looplang.org">Downloads</a> |
       <a href="https://discord.gg/T3tqQBTyJA">Discord</a> | 
       <a href="https://looplang.atlassian.net/jira/dashboards/10003">Jira</a>
</p>