use crate::command::parse::CommandInfo;
use std::env;
use std::io::{Error, ErrorKind};

pub fn execute_builtin_command(command_info: &CommandInfo) -> Result<(), std::io::Error> {
    match command_info.command.as_str() {
        "cd" => {
            let default_dir = String::from(".");
            let dir = command_info.args.get(0).unwrap_or(&default_dir);
            env::set_current_dir(dir).map_err(|e| Error::new(ErrorKind::Other, e))?;
            Ok(())
        }
        "exit" => {
            std::process::exit(0); // Terminate the process
        }
        "history" => {
            // Access and print history (requires access to history data structure)
            // This is a placeholder - you'll need to integrate with your history module
            let history = vec!["command1".to_string(), "command2".to_string()]; // Replace with actual history
            for entry in history {
                println!("{}", entry);
            }
            Ok(())
        }
        _ => Err(Error::new(ErrorKind::Other, "Not a builtin command")),
    }
}
