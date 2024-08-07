# Mini_grep Project

## Demo
[Demo Video](https://github.com/user-attachments/assets/7e677031-a8f8-4093-8a9c-c3ff6d1a2631)


## Project Description 


This project is a Rust application that replicates the functionality of the classic command line search tool, <b>grep</b>. Leveraging Rust's speed, safety, single binary output, and cross-platform support, this tool searches a specified file for a specified string and prints the matching lines. The application handles command line arguments, file I/O, and environment variables, and includes robust error handling using the <b >eprintln!</b> macro. It is designed to be well-organized, effectively store data in appropriate data structures, handle errors gracefully, and be thoroughly tested. This project is based on a chapter from the [The Rust Programming Language book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

## Features

#### Command Line Argument Parsing:

<li>Accepts a file path and a search string as command line arguments.
<li>Provides error messages if the arguments are missing or incorrect.

#### File I/O:

<li>Reads the contents of the specified file.
<li>Handles file-related errors gracefully, such as file not found or permission denied.

#### Search Functionality:

<li>Searches for the specified string within the file.
<li>Supports case-insensitive search.
<li>Returns lines that contain the search string.

#### Output:

<li>Prints the matching lines to the standard output.
<li>Uses the eprintln! macro to print error messages to the standard error.

#### Environment Variables:

<li>Supports configuration through environment variables, such as enabling case-insensitive search.

#### Error Handling:

<li>Provides clear and informative error messages for various failure scenarios.
<li>Uses Rust's robust error handling mechanisms to ensure the application does not crash unexpectedly.


## Author

**[KitsuneKode](https://github.com/KitsuneKode)**

