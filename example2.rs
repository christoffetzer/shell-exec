#!/usr/bin/env rust-script

//! Define external dependencies as we do in a Cargo.toml file.
//! ```cargo
//! [dependencies]
//! sh-exec = "*"
//! ```
//! 


/* 
## Example: convert a bash script to Rust using sh-exec
## This example demonstrates how to convert a simple bash script into Rust using the `sh-exec` library. 
## The original bash script prints a greeting, asks for the user's name, and shows the current date.
## The Rust version uses the `sh-exec` library to execute shell commands and handle errors.
## Original Bash Script:

#!/bin/bash

# Print a greeting
echo "Hello, World!"

# Ask for the user's name
read -p "What is your name? " name

# Print a personalized message
echo "Hello, $name!"

# Show the current date
echo "Today's date is: $(date)"
 */

use sh_exec::*;

fn main() {
    // Print a greeting
    println!("Hello, world!");

    // Ask for the user's name
    let name = read_prompt("What is your name? ");

    // Print a personalized message
    println!("Hello, {name}!");

    // Show the current date
    println!("Today's date is: {date}", date = e!("date"));

}
