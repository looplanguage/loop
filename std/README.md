# Standard Library

In this directory you will find a standard library for Loop. It is implemented in Rust and Loop itself.

> **Note:** This is in heavy development and thus not finished

## Content

The std contains these functions to call:
- `const char* println(const char* ptr);`: Printing a string in the standard output, with a new line
- `const char* print(const char* ptr);`: Printing a string in the standard output
- `const char* input(const char* ptr);`: Reading from the standard input
- `const char* read_file(const char* filelocation);`: Reads a file (relative location), and returns the content as a string
- `const char* write_file(const char* filelocation, const char* content);`: Write to a file, creates the file if it does not exist. returns true if succesful

## Example

This is an example of how to use the library directly without the abstraction written in Loop arount it:

```
import "std" as std

file_loc := "file.txt"

std.println("Hello, this is std")
_ := std.write_file(file_loc, "I have written a file")
content := std.read_file(file_loc)
std.println(content)
```

## Compilation guide 

1. Make sure you have installed RustC
2. `rustc lib.rs --crate-type=cdylib`