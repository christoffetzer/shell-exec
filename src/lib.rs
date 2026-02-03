use colored::*;
use shells::sh;
use std::env;
use std::fmt;

/// Custom error type for shell command execution with beautiful formatting
#[derive(Debug)]
pub enum ShellExecError {
    ExecutionFailed {
        command: String,
        exit_code: i32,
        stderr: Option<String>,
        stdout: Option<String>,
        error_id: String,
    },
    EnvVarNotFound {
        var_name: String,
        error_id: String,
        source: env::VarError,
    },
    Timeout {
        command: String,
        duration_ms: u64,
        error_id: String,
    },
    JoinFailed {
        command: String,
        error_id: String,
    },
}

// Implement Display manually to override thiserror's default formatting
impl fmt::Display for ShellExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_detailed())
    }
}

// Implement std::error::Error manually since we're not using thiserror's Error derive
impl std::error::Error for ShellExecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ShellExecError::EnvVarNotFound { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl ShellExecError {
    /// Format the error with colors and detailed information
    pub fn format_detailed(&self) -> String {
        match self {
            ShellExecError::ExecutionFailed {
                command,
                exit_code,
                stderr,
                stdout,
                error_id,
            } => {
                let mut output = String::new();
                output.push_str(&format!("{}\n", "Command Execution Failed".red().bold()));
                output.push_str(&format!("  {}: {}\n", "Command".cyan(), command));
                output.push_str(&format!("  {}: {}\n", "Exit Code".cyan(), exit_code));
                output.push_str(&format!("  {}: {}\n", "Error ID".cyan(), error_id.green()));

                if let Some(stdout_val) = stdout {
                    if !stdout_val.is_empty() {
                        output.push_str(&format!("\n  {}:\n", "Standard Output".yellow()));
                        for line in stdout_val.lines() {
                            output.push_str(&format!("    {}\n", line));
                        }
                    }
                }

                if let Some(stderr_val) = stderr {
                    if !stderr_val.is_empty() {
                        output.push_str(&format!("\n  {}:\n", "Standard Error".red()));
                        for line in stderr_val.lines() {
                            output.push_str(&format!("    {}\n", line));
                        }
                    }
                }

                output
            }
            ShellExecError::EnvVarNotFound {
                var_name,
                error_id,
                source,
            } => {
                format!(
                    "{}\n  {}: {}\n  {}: {}\n  {}: {:?}\n",
                    "Environment Variable Not Found".red().bold(),
                    "Variable".cyan(),
                    var_name,
                    "Error ID".cyan(),
                    error_id.green(),
                    "Reason".cyan(),
                    source
                )
            }
            ShellExecError::Timeout {
                command,
                duration_ms,
                error_id,
            } => {
                format!(
                    "{}\n  {}: {}\n  {}: {}ms\n  {}: {}\n",
                    "Command Timed Out".red().bold(),
                    "Command".cyan(),
                    command,
                    "Timeout".cyan(),
                    duration_ms,
                    "Error ID".cyan(),
                    error_id.green()
                )
            }
            ShellExecError::JoinFailed { command, error_id } => {
                format!(
                    "{}\n  {}: {}\n  {}: {}\n",
                    "Thread Join Failed".red().bold(),
                    "Command".cyan(),
                    command,
                    "Error ID".cyan(),
                    error_id.green()
                )
            }
        }
    }
}

/// Result type alias for shell operations
pub type ShellExecResult<T> = anyhow::Result<T>;

/// Output from a shell command execution
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl CommandOutput {
    /// Returns stdout if it's not empty, None otherwise
    pub fn stdout(&self) -> Option<String> {
        if self.stdout.is_empty() {
            None
        } else {
            Some(self.stdout.clone())
        }
    }

    /// Returns stderr if it's not empty, None otherwise
    pub fn stderr(&self) -> Option<String> {
        if self.stderr.is_empty() {
            None
        } else {
            Some(self.stderr.clone())
        }
    }

    /// Returns true if the command succeeded (exit code 0)
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
}

/// Executes a shell command and returns a Result containing the command's output
///
/// # Arguments
/// * `cmd` - The shell command to execute
/// * `error_id` - A unique identifier for debugging purposes
///
/// # Returns
/// * `Ok(String)` - The stdout of the command if successful
/// * `Err(ShellError)` - An error with diagnostic information if the command failed
pub fn execute_command(cmd: &str, error_id: &str) -> ShellExecResult<String> {
    let output = execute_command_raw(cmd, error_id)?;
    Ok(output.stdout)
}

/// Executes a shell command and returns the full output structure
///
/// # Arguments
/// * `cmd` - The shell command to execute
/// * `error_id` - A unique identifier for debugging purposes
///
/// # Returns
/// * `Ok(CommandOutput)` - The full output including stdout, stderr, and exit code
/// * `Err(ShellError)` - An error with diagnostic information if the command failed
pub fn execute_command_raw(cmd: &str, error_id: &str) -> Result<CommandOutput, ShellExecError> {
    let (code, stdout, stderr) = sh!("{}", cmd);

    let output = CommandOutput {
        stdout,
        stderr,
        exit_code: code,
    };

    if code == 0 {
        Ok(output)
    } else {
        Err(ShellExecError::ExecutionFailed {
            command: cmd.to_string(),
            exit_code: code,
            stderr: output.stderr(),
            stdout: output.stdout(),
            error_id: error_id.to_string(),
        }
        .into())
    }
}

/// Read the content of a given environment variable
///
/// # Arguments
/// * `var_name` - The name of the environment variable
/// * `error_id` - A unique identifier for debugging purposes
///
/// # Returns
/// * `Ok(String)` - The value of the environment variable
/// * `Err(ShellError)` - An error if the variable is not set
pub fn get_env(var_name: &str, error_id: &str) -> ShellExecResult<String> {
    env::var(var_name).map_err(|e| {
        ShellExecError::EnvVarNotFound {
            var_name: var_name.to_string(),
            error_id: error_id.to_string(),
            source: e,
        }
        .into()
    })
}

/// Read an environment variable with a default value if not set
///
/// # Arguments
/// * `var_name` - The name of the environment variable
/// * `default` - The default value to return if the variable is not set
pub fn get_env_or(var_name: &str, default: &str) -> String {
    env::var(var_name).unwrap_or_else(|_| default.to_string())
}

/// Main runner that handles errors with nice formatting
///
/// This function runs your main logic and prints diagnostic information
/// if an error occurs, including cargo package metadata.
pub fn run_with_diagnostics<F>(f: F)
where
    F: FnOnce() -> anyhow::Result<()>,
{
    if let Err(report) = f() {
        eprintln!("\n{}", "=".repeat(80).red());
        eprintln!("{}", "Application Error".red().bold());
        eprintln!("{}", "=".repeat(80).red());
        eprintln!();

        // Try to get ShellError for better formatting
        if let Some(shell_err) = report.downcast_ref::<ShellExecError>() {
            eprintln!("{}", shell_err.format_detailed());
        } else {
            // Fall back to anyhow's formatting
            eprintln!("{:?}", report);
        }

        eprintln!();
        eprintln!("{}", "Package Information:".cyan().bold());
        eprintln!("  Name:        {}", env!("CARGO_PKG_NAME"));
        eprintln!("  Version:     {}", env!("CARGO_PKG_VERSION"));
        eprintln!("  Authors:     {}", env!("CARGO_PKG_AUTHORS"));
        eprintln!("  Description: {}", env!("CARGO_PKG_DESCRIPTION"));
        eprintln!("  Homepage:    {}", env!("CARGO_PKG_HOMEPAGE"));
        eprintln!("  Repository:  {}", env!("CARGO_PKG_REPOSITORY"));
        eprintln!();
        std::process::exit(1);
    }
}

/// Macro to trap panics and errors with nice error messages
///
/// # Example
/// ```ignore
/// trap_panics_and_errors!("main-entry-point", || {
///     // Your main logic here
///     Ok(())
/// });
/// ```
#[macro_export]
macro_rules! trap_panics_and_errors {
    ($error_id:expr, $main:expr) => {{
        use colored::Colorize;
        use std::panic;

        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            $crate::run_with_diagnostics(|| {
                $main().map_err(|e: Box<dyn std::error::Error>| {
                    anyhow::anyhow!("[{}] {}", $error_id, e)
                })
            });
        }));

        if let Err(panic_info) = result {
            eprintln!("\n{}", "=".repeat(80).red().bold());
            eprintln!("{}", "PANIC OCCURRED".red().bold());
            eprintln!("{}", "=".repeat(80).red().bold());
            eprintln!("Error ID: {}", $error_id.to_string().green());
            eprintln!("Panic Info: {:?}", panic_info);
            eprintln!();
            eprintln!("{}", "Package Information:".cyan().bold());
            eprintln!("  Name:    {}", env!("CARGO_PKG_NAME"));
            eprintln!("  Version: {}", env!("CARGO_PKG_VERSION"));
            eprintln!();
            std::process::exit(101);
        }
    }};
}

/// Execute a shell command with optional verbose output
///
/// # Example
/// ```ignore
/// let output = exec!("test-001", true, "echo {}", "Hello World")?;
/// ```
#[macro_export]
macro_rules! exec {
    ($error_id:expr, $verbose:expr, $($cmd:tt)*) => {{
        use colored::Colorize;
        let formatted_str = format!($($cmd)*);
        if $verbose {
            eprintln!(
                "{}",
                format!("[{}] {}", $error_id, formatted_str).magenta()
            );
        }
        $crate::execute_command(&formatted_str, $error_id)
    }};
}

/// Execute a shell command with logging at INFO level
///
/// # Example
/// ```ignore
/// let output = s!("test-002", "ls -la")?;
/// ```
#[macro_export]
macro_rules! s {
    ($error_id:expr, $($cmd:tt)*) => {{
        use colored::Colorize;
        use log::{debug, info, error};
        let formatted_str = format!($($cmd)*);
        info!("{}", format!("[{}] Executing: {}", $error_id, formatted_str).magenta());

        let result = $crate::execute_command(&formatted_str, $error_id);

        match &result {
            Ok(output) => debug!("Output: {}", output),
            Err(e) => {
                if let Some(shell_err) = e.downcast_ref::<$crate::ShellExecError>() {
                    error!("{}", shell_err.format_detailed());
                } else {
                    error!("Error: {:?}", e);
                }
            }
        }

        result
    }};
}

/// Execute a shell command and panic on error
///
/// # Example
/// ```ignore
/// let output = e!("echo Hello");
/// ```
#[macro_export]
macro_rules! e {
    ($($cmd:tt)*) => {{
        let formatted_str = format!($($cmd)*);
        $crate::execute_command(&formatted_str, "no-error-id")
            .expect(&format!("Command failed: {}", formatted_str))
    }};
}

/// Execute a shell command with a timeout
///
/// # Example
/// ```ignore
/// use std::time::Duration;
/// let output = a!("test-003", Duration::from_secs(5), "sleep 2 && echo done")?;
/// ```
#[macro_export]
macro_rules! a {
    ($error_id:expr, $duration:expr, $($cmd:tt)*) => {{
        use std::{thread, time};
        use colored::Colorize;
        use log::{debug, info, error};

        let formatted_str = format!($($cmd)*);
        info!("{}", format!("[{}] Executing with timeout: {}", $error_id, formatted_str).magenta());

        let error_id_clone = $error_id.to_string();
        let cmd_clone = formatted_str.clone();

        let handle = thread::spawn(move || {
            $crate::execute_command(&cmd_clone, &error_id_clone)
        });

        let check_interval = time::Duration::from_millis(10);
        let start = time::Instant::now();

        let result = loop {
            if handle.is_finished() {
                break match handle.join() {
                    Ok(result) => {
                        match &result {
                            Ok(output) => debug!("Output: {}", output),
                            Err(e) => {
                                if let Some(shell_err) = e.downcast_ref::<$crate::ShellExecError>() {
                                    error!("{}", shell_err.format_detailed());
                                } else {
                                    error!("Error: {:?}", e);
                                }
                            }
                        }
                        result
                    }
                    Err(_) => {
                        Err($crate::ShellExecError::JoinFailed {
                            command: formatted_str.clone(),
                            error_id: $error_id.to_string(),
                        }.into())
                    }
                };
            }

            thread::sleep(check_interval);

            if start.elapsed() >= $duration {
                // Thread will be abandoned but command may continue running
                let duration_ms = $duration.as_millis() as u64;
                break Err($crate::ShellExecError::Timeout {
                    command: formatted_str.clone(),
                    duration_ms,
                    error_id: $error_id.to_string(),
                }.into());
            }
        };

        result
    }};
}

/// Read input from stdin with a prompt
///
/// # Arguments
/// * `prompt` - The prompt to display to the user
///
/// # Returns
/// The user's input as a String (trimmed of whitespace)
pub fn read_prompt(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");

    buffer.trim().to_string()
}

/// Read input from stdin with a prompt, returning a Result
///
/// # Arguments
/// * `prompt` - The prompt to display to the user
///
/// # Returns
/// * `Ok(String)` - The user's input (trimmed)
/// * `Err` - If reading from stdin fails
pub fn read_prompt_result(prompt: &str) -> anyhow::Result<String> {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout()
        .flush()
        .map_err(|e| anyhow::anyhow!("Failed to flush stdout: {}", e))?;

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {}", e))?;

    Ok(buffer.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_command() {
        let output = execute_command("echo Hello World", "test-001").unwrap();
        assert_eq!(output.trim(), "Hello World");
    }

    #[test]
    fn test_successful_command_raw() {
        let output = execute_command_raw("echo Hello World", "test-002").unwrap();
        assert_eq!(output.stdout.trim(), "Hello World");
        assert!(output.success());
        assert!(output.stdout().is_some());
    }

    #[test]
    fn test_exec_macro() {
        let output = exec!("test-003", false, "echo {}", "Hello World").unwrap();
        assert_eq!(output.trim(), "Hello World");
    }

    #[test]
    fn test_e_macro() {
        let output = e!("echo test");
        assert_eq!(output.trim(), "test");
    }

    #[test]
    fn test_failing_command() {
        let result = execute_command("nonexistent_command_xyz", "test-004");
        assert!(result.is_err());

        if let Err(e) = result {
            let error_string = format!("{:?}", e);
            assert!(error_string.contains("nonexistent_command_xyz"));
        }
    }

    #[test]
    fn test_command_output_options() {
        let output = execute_command_raw("echo test", "test-005").unwrap();
        assert!(output.stdout().is_some());
        assert_eq!(output.stdout().unwrap().trim(), "test");

        // stderr should be empty for echo
        assert!(output.stderr().is_none() || output.stderr().unwrap().is_empty());
    }

    #[test]
    fn test_get_env_or() {
        let value = get_env_or("NONEXISTENT_VAR_XYZ", "default_value");
        assert_eq!(value, "default_value");

        // Set an env var and test
        unsafe { std::env::set_var("TEST_VAR_XYZ", "test_value") };
        let value = get_env_or("TEST_VAR_XYZ", "default");
        assert_eq!(value, "test_value");
    }

    #[test]
    fn test_timeout_macro() {
        use std::time::Duration;

        // This should succeed (quick command)
        let result = a!("test-006", Duration::from_secs(5), "echo fast");
        assert!(result.is_ok());
    }

    #[test]
    fn test_formatted_error() {
        // Test that errors format nicely
        let result = execute_command("nonexistent_xyz_123", "format-test");
        assert!(result.is_err());

        if let Err(e) = result {
            if let Some(shell_err) = e.downcast_ref::<ShellExecError>() {
                let formatted = shell_err.format_detailed();
                assert!(formatted.contains("Command Execution Failed"));
                assert!(formatted.contains("nonexistent_xyz_123"));
                assert!(formatted.contains("format-test"));
            }
        }
    }
}
