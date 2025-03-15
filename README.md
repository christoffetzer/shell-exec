# shell-exec

This is a crate that simplifies the execution of CLI programs from Rust programs using a macro `exec!(error_id, cmd)`. The `error_id` is just a unique string literal that is printed in the case of an error. I typically generate this string literal as follows:

```bash
echo "\"$RANDOM-$RANDOM-$RANDOM\", "
```

and pasting the output in the as the first argument of `exec!`. This simplifies finding the right code snipped in case you need to track down the code that issued a certain error message.

On success, `exec!` returns the `stdout` of the executed command. If the execution fails, the macro retuns an `Err` of type `ShellError`.  One can handle the errors using the question mark operator:

```rust
    exec!("10874-26631-30577", "ls")`
```

The `cmd` argument is a `format` sting, i.e., one can use positional and named arguments as well as variable names:

```rust
    let path="/tmp";
    exec!("17068-22053-696", "ls {path}")`
```

As for macro `format!`,  macro `exec!` supports positional arguments:

```rust
    // example: with position argument "/"
    println!("ls of {path} is {}", exec!("15911-12192-19189", "ls {}", "/")?);
```

`exec!` also supports named arguments:

```rust
    // example: with named argument p="/tmp"
    println!("ls of {path} is {}", exec!("15911-12192-19189", "ls {p}", p="/tmp")?);
```


Hence, prints on `stderr` an error message that includes:

- the command line that failed,
- the error ID,
- the stdout,
- the stderr,

- the cargo information related to this 

## Example

Here is a simple program that uses this crate:

```rust
#!/usr/bin/env rust-script
//! ```cargo
//! [package]
//! name = "example"
//! edition = "2024"
//!
//! [dependencies]
//! clap = { version = "4", features = ["derive"] }
//! shell-exec = { version = "*", path = "/home/ubuntu/subtree-sconectl/check_cpufeatures/shell-exec" }
//! colored = "*"
//! ```

use shell_exec::*;

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
```

