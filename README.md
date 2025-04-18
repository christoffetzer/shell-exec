# shell-exec

This Rust crate simplifies the execution of CLI programs by a Rust programs. It exports two macros: `s!` and `exec!`. In general, you want to use macro `s!` to execute shell commands. We export  `exec!` for backward compatibility with some existing code.

## Macro `exec!`

Macro `exec!` takes three agruments: `exec!(error_id, verbose, cmd)`. Argument `error_id` is just a unique string literal that is printed in case of an error. I typically generate this string literal as follows:

```bash
echo "\"$RANDOM-$RANDOM-$RANDOM\", "
```

and paste the output as the first argument of macro `exec!` (or, even better the first argument of `s!`). This simplifies finding the code that issued a certain error message.

Argument `verbose` must be of type `bool`. If it is `true`, the command that will be executed is first printed to `stderr`.

On success, `exec!` returns the `stdout` of the executed command. If the execution fails, the macro returns an `Err` of type `ShellError`.  One can handle the errors using the question mark operator:

```rust
    exec!("10874-26631-30577", false, "ls")?`
```

The `cmd` argument is a `format` sting, i.e., one can use positional and named arguments as well as variable names:

```rust
    let path="/tmp";
    exec!("17068-22053-696", "ls {path}")`
```

As for macro `format!`,  macro `exec!` supports positional arguments:

```rust
    // example: with position argument "/"
    println!("ls of {path} is {}", exec!("15911-12192-19189", false, "ls {}", "/")?);
```

`exec!` also supports named arguments:

```rust
    // example: with named argument p="/tmp"
    println!("ls of {path} is {}", exec!("15911-12192-19189", false, "ls {p}", p="/tmp")?);
```

## Macro `s!`

Macro `s!` is similar to macro `exec!`: `s!` uses crate `log` to issue log output instead of `eprintln!`. 
Hence, it does not have a flag `verbose`.  Moreover, it  

- logs the executed command with all arguments at `info` level, 
- logs the output, i.e., the `stdout` of the command, at `debug` level, and 
- logs errors, i.e., the `stderr` of the command,  at `error` level.

On success of the executed command, `sh!` returns the `stdout` of the executed command wrapped in `Ok(stdout)`.

Example:

```rust
        // s! the command is executed and the output is returned
        // s! uses the logger to print the command if the log level is set to info
        // s! uses the logger to print the output of the command if the log level is set to debug
        s!("14526-30026-17058", "echo Hello World")?;
```


On error, `s!`, logs an error that includes:

- the command line that failed,
- the error ID,
- the stdout,
- the stderr,

- the cargo information related to this 

## Example

Here is a simple program that uses this crate. Note that you need to define dependency `sh-exec` to import this crate and additionally dependencies `colored`, and `log` in your Cargo.toml:

```toml
[dependencies]
sh-exec = "*"
colored = "*"
log =  "*"
```

and in your Rust program, you import the macros as follows:

```Rust
use sh_exec::*;
```

You can use this crate also from within Rust scripts:

```rust
#!/usr/bin/env rust-script
//! ```cargo
//! [package]
//! name = "example"
//! edition = "2024"
//!
//! [dependencies]
//! sh-exec = "*"
//! colored = "*"
//! log = "*"
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
        println!("ls of /etc/hosts is {}", s!("15911-12192-19189", "ls {p}", p="/etc/hosts")?);

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

        // We need to help Rust regarding the error type
        Ok::<(), Box<dyn Error>>(())
    });
}
```

Executing the code as a `rust-script` (see file `example.rs`), we get the following output:

```bash
$ ./example.rs
exec!(17068-22053-696,ls -d /etc)
ls -d of / is /

ls of /etc/hosts is /etc/hosts

exec!(28328-2323-44343,bash -c 'echo Hello World')
Output: Hello World

exec!(28328-2323-3278,nonexistent_command)
Expected error: Command failed: 'nonexistent_command'
sh_exec Exit code: 127
sh_exec Error ID:  28328-2323-3278
Standard error:
sh: 1: nonexistent_command: not found


exec!(28328-2323-333,nonexistent_command arg1 arg2)
```
