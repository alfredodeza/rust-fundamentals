# Create a File Reader Application

In this lab, you will enhance a file reader application in Rust.  Use the example code Opens in a new tab
  for error handling as a starting point for reading a file and printing its contents. You are tasked with extending the application to allow users to specify the file to be read as a command-line argument. The end result will be a GitHub repository containing the complete code for the enhanced file reader application.


**Learning Objectives:**

- Understand how to read and process files in Rust using the standard library.
- Practice error handling techniques, such as matching on different error cases.
- Learn how to organize code and structure a Rust application.

**Steps:**

1. 1. Create a new repository in your account for your Rust project. Use the [Rust template repository](https://github.com/alfredodeza/rust-template) to quickly generate one for your own account. Use this link to [create it in one step](https://github.com/alfredodeza/rust-template/generate).
1. Use the [example code](https://github.com/alfredodeza/rust-fundamentals/blob/main/examples/16-error-handling/error-handling/src/main.rs) as a starting point
1. Add the ability to pass in any file path to avoid hard-coding the path in the program. Use the [Rust docs](https://doc.rust-lang.org/rust-by-example/std_misc/arg.html)
  or this sample code to get an idea on how to get the first argument in the console:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);
}
```

**Concepts Covered:**

- Introduction to Rust: Students will be introduced to the provided source code and its basic structure, serving as a starting point for the lab.
- Variable assignment and immutability: Students will explore how variables are assigned and how immutability is enforced in Rust.
- Basics of control flow: Students will understand how control flow structures like match expressions and loops are used to handle errors and process file contents.
- Function Basics: Students will examine the main function and its purpose, as well as how to structure and organize code in Rust programs.
- Error handling: Students will learn how to handle different error cases when opening and reading files, using the match expression to provide helpful error messages.
- File processing: Students will modify the code to read the specified file line by line and print its contents to the console.

By completing this lab, you will gain practical experience in Rust by extending an existing file reader application. You will develop skills in file I/O, error handling, and some basic code organization, utilizing the concepts introduced in the lessons for this week.
