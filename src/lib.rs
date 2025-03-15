use colored::*;
use shells::sh;
use std::env;
use std::error::Error;
use std::fmt;

/// Custom error type for shell command execution
#[derive(Debug)]
pub struct ShellError<'a> {
    command: String,
    exit_code: i32,
    stderr: String,
    stdout: String,
    error_id: &'a str,
}

impl ShellError<'_> {
    /// Creates a new ShellError instance
    pub fn new(
        command: String,
        exit_code: i32,
        stderr: String,
        stdout: String,
        error_id: &'static str,
    ) -> Self {
        ShellError {
            command,
            exit_code,
            stderr,
            stdout,
            error_id,
        }
    }
}

impl fmt::Display for ShellError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}\nExit code: {}\nError ID: {}\n",
            "Command failed".red(),
            self.command,
            self.exit_code,
            self.error_id.green(),
        )?;

        if !self.stdout.is_empty() {
            write!(f, "Standard output:\n{}\n", self.stdout.green())?;
        }

        if !self.stderr.is_empty() {
            write!(f, "Standard error:\n{}\n", self.stderr.magenta())?;
        }

        Ok(())
    }
}

impl Error for ShellError<'_> {}

/// Executes a shell command and returns a Result containing the command's output
pub fn execute_command(cmd: &str, error_id: &'static str) -> Result<String, ShellError<'static>> {
    let command = cmd.to_string();
    let (code, stdout, stderr) = sh!("{}", cmd);

    // Check exit code
    if code == 0 {
        Ok(stdout)
    } else {
        let error = ShellError::new(command, code, stderr, stdout, error_id);
        Err(error)
    }
}

pub fn get_env(env: &str, error_id: &'static str) -> Result<String, ShellError<'static>> {
    // Get VERSION from environment
    match env::var(env) {
        Err(e) => Err(ShellError {
            command: format!("shell-exec: get_env({env})"),
            exit_code: 0,
            stderr: format!("Environment variable '{env}' is not defined: {e:#?}."),
            stdout: "".to_string(),
            error_id,
        }),
        Ok(value) => Ok(value),
    }
}

pub fn main_run(run: fn() -> Result<(), Box<dyn Error>>) {
    if let Err(e) = run() {
        eprintln!("Version: {}", env!("CARGO_PKG_VERSION"));
        eprintln!("Name: {}", env!("CARGO_PKG_NAME"));
        eprintln!("Authors: {}", env!("CARGO_PKG_AUTHORS"));

        // Optional fields
        eprintln!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
        eprintln!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
        eprintln!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
        eprintln!("{e}")
    }
}

/// trap_panics_and_errors traps panics that might be issued when calling a given function
/// It will print a nice error message in case a panic is trapped.
/// This macro also traps errors, prints the error and exists the program with error code 1
///
/// NOTE
///   the Err type returned by the given function must return an Err that implements the Display trait.
#[macro_export]
macro_rules! trap_panics_and_errors {
    ($error_id:literal , $main:expr) => {
        use std::process;
        use std::error::Error;
        use colored::*;
        match std::panic::catch_unwind(|| {
            match $main() {
                Err(e) => {
                    eprintln!("{}: {}", "trap_panics_and_errors".red(), $error_id.green());
                    eprintln!("  Version: {}", env!("CARGO_PKG_VERSION"));
                    eprintln!("  Name: {}", env!("CARGO_PKG_NAME"));
                    eprintln!("  Authors: {}", env!("CARGO_PKG_AUTHORS"));

                    // Optional fields
                    eprintln!("  Description: {}", env!("CARGO_PKG_DESCRIPTION"));
                    eprintln!("  Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
                    eprintln!("  Repository: {}", env!("CARGO_PKG_REPOSITORY"));
                    eprintln!("  Error: {e}");
                    // Exit with error (non-zero)
                    process::exit(1)
                }
                Ok(result) => result,
            }
        }) {
            Ok(result) => result,
            Err(e) => {
                eprintln!(
                    "Error id: {}, 31963-28837-7387. Error {}: {e:#?}!", $error_id,
                    "Application panicked".red()
                );
                std::process::exit(101);
            }
        }
    };
}

#[macro_export]
macro_rules! exec {
    ($error_id:literal , $verbose:expr , $($cmd:tt )* ) => {{
        let formatted_str = &format!($( $cmd )*);
        if $verbose { eprintln!("{}", format!("exec!({},{})", $error_id, formatted_str ).magenta()) }
        execute_command(formatted_str, $error_id)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_command() {
        let output = execute_command("echo Hello World", "8923-2323-2323").unwrap();
        assert_eq!(output.trim(), "Hello World");
    }

    #[test]
    fn test_successful_fmt() {
        let output = exec!("8923-2323-2323", false, "echo Hello World").unwrap();
        assert_eq!(output.trim(), "Hello World");
    }

    #[test]
    fn test_successful_fmt2() {
        let output = exec!("21236-28986-4446", true, "echo {}", "Hello World",).unwrap();
        assert_eq!(output.trim(), "Hello World");
    }

    #[test]
    fn test_failing_command() {
        let result = execute_command("nonexistent_command", "8923-2323-3289");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.exit_code, 127);
        assert!(!error.stderr.is_empty());
    }
}
