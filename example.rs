#!/usr/bin/env rust-script
//! ```cargo
//! [package]
//! name = "example"
//! edition = "2024"
//!
//! [dependencies]
//! clap = { version = "4", features = ["derive"] }
//! sh-exec = "*"
//! colored = "*"
//! ```

use sh_exec::*;

fn main() {
    trap_panics_and_errors!("18428-30925-25863", || {

        // example: ls of /tmp
        let path="/etc";
        exec!("17068-22053-696", true, "ls -d {path}")?;

        // example: with position argument "/"
        println!("ls -d of / is {}", exec!("15911-12192-19189", false,  "ls -d {}", "/")?);

        // example: with named argument p="/tmp"
        println!("ls of /etc/hosts is {}", exec!("15911-12192-19189", true, "ls {p}", p="/etc/hosts")?);

        // Test successful command
        let output = exec!("28328-2323-44343", true, "bash -c 'echo Hello World'")?;
        println!("Output: {}", output);

        // Test failing command
        match exec!("28328-2323-3278", true, "nonexistent_command") {
            Ok(output) => println!("Unexpected success: {}", output),
            Err(e) => println!("Expected error: {}", e),
        }
        // expecting to fail:
        exec!( "28328-2323-333", true,  "nonexistent_command arg1 arg2")?;
 
        Ok::<(), Box<dyn Error>>(())
    });
}
