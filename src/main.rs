use sh_exec::*;
use anyhow::*; // show how to use together with anyhow

fn main() -> Result<()> {
        use std::time;

        env_logger::init();

    
        // example: ls of /tmp
        let path="/etc";
        println!("- ls of {path} is {}", exec!("17068-22053-696", true, "ls {path}").with_context(|| format!("Very unexpected - ls failed on {path}"))?);

        // example: with position argument "/"
        println!("ls of {path} is {}", s!("15911-12192-19189",  "ls {}", "/").with_context(|| "Very unexpected - ls failed on /".to_string())?);

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
        match s!("28328-2323-3278", "nonexistent_command").with_context(|| "Failed to execute command 'nonexistent_command'".to_string()) {
            std::result::Result::Ok(output) => println!("Unexpected success: {}", output),
            Err(e) => println!("Expected error: {}", e),
        }

        // macro a! provides timeouts and it will return with a Timeout error 
        // if the command does not finish in time
        let ten_secs = time::Duration::from_secs(10);

        println!("sleep = {:?}", a!("14526-30888026-777", ten_secs, "sleep 2; echo Hello World"));

        // Print a greeting
        println!("Hello, world!");

        // Ask for the user's name
        let name = read_prompt("What is your name? ");

        // Print a personalized message
        println!("Hello, {name}!");

        // Show the current date
        println!("Today's date is: {date}", date = e!("date"));

        Ok(())
    }
