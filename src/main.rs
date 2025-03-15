use sh_exec::*;

fn main() {
    trap_panics_and_errors!("18428-30925-25863", || {

        // example: ls of /tmp
        let path="/tmp";
        println!("- ls of {path} is {}", exec!("17068-22053-696", true, "ls {path}")?);

        // example: with position argument "/"
        println!("ls of {path} is {}", exec!("15911-12192-19189", false,  "ls {}", "/")?);

        // example: with named argument p="/tmp"
        println!("ls of {path} is {}", exec!("15911-12192-19189", true, "ls {p}", p="/tmp")?);

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
