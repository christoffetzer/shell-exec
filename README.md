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


Macro `s!` is similar to macro `exec!` but it uses crate `log` to issue output (instead of `eprintln!`). 
Hence, it does not have a flag `verbose`.  Moreover, it logs the command at `info` level, 
logs the output at `debug` level, and errors at `error` level.

Example:

```rust
        // s! the command is executed and the output is returned
        // s! uses the logger to print the command if the log level is set to info
        // s! uses the logger to print the output of the command if the log level is set to debug
        s!("14526-30026-17058", "echo Hello World")?;
```


Hence, prints on `stderr` an error message that includes:

- the command line that failed,
- the error ID,
- the stdout,
- the stderr,

- the cargo information related to this 

## Example

Here is a simple program that uses this crate. Note that you need to define dependency `colored`.

```rust
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

        // s! the command is executed and the output is returned
        // s! uses the logger to print the command if the log level is set to info
        // s! uses the logger to print the output of the command if the log level is set to debug
        s!("14526-30026-17058", "echo Hello World")?;

        // Test failing command
        match exec!("28328-2323-3278", true, "nonexistent_command") {
            Ok(output) => println!("Unexpected success: {}", output),
            Err(e) => println!("Expected error: {}", e),
        }
        // expecting to fail:
        s!("14526-30026-17061", "exit 1")?;
 
        Ok::<(), Box<dyn Error>>(())
    });
}
```

Executing the code as a `rust-script` (see file `example.rs`), we get the following output:

```bash
$ ./example.rs
exec!(17068-22053-696,ls -d /etc)
ls -d of / is /

exec!(15911-12192-19189,ls /etc/hosts)
ls of /etc/hosts is /etc/hosts

exec!(28328-2323-44343,bash -c 'echo Hello World')
Output: Hello World

exec!(28328-2323-3278,nonexistent_command)
Expected error: Command failed: nonexistent_command
Exit code: 127
Error ID: 28328-2323-3278
Standard error:
sh: 1: nonexistent_command: not found


exec!(28328-2323-333,nonexistent_command arg1 arg2)
trap_panics_and_errors: 18428-30925-25863
  Version: 0.1.0
  Name: example
  Authors: Anonymous
  Description:
  Homepage:
  Repository:
  Error: Command failed: nonexistent_command arg1 arg2
Exit code: 127
Error ID: 28328-2323-333
Standard error:
sh: 1: nonexistent_command: not found
```
