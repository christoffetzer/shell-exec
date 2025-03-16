use sh_exec::*;
fn main() {
    trap_panics_and_errors!("18428-30925-25863", || {
        env_logger::init();

        // example: ls of /tmp
        let path="/etc";
        println!("- ls of {path} is {}", exec!("17068-22053-696", true, "ls {path}")?);

        // example: with position argument "/"
        println!("ls of {path} is {}", exec!("15911-12192-19189", false,  "ls {}", "/")?);

        // example: with named argument p="/tmp"
        println!("ls of {path} is {}", exec!("15911-12192-19189", true, "ls {p}", p="/etc")?);

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
