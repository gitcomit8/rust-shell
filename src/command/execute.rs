use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};

use crate::command::builtin::execute_builtin_command;
use crate::command::parse::CommandInfo;
use crate::io::redirection;

pub fn execute_command(
    command_info: CommandInfo,
    previous_command: &mut Option<std::process::Child>,
) -> Result<(), std::io::Error> {
    let mut command = Command::new(&command_info.command);

    command.args(&command_info.args);

    if let Some(input_file) = &command_info.redirect_in {
        let file = std::fs::File::open(input_file)?;
        command.stdin(Stdio::from(file));
    }

    if let Some(output_file) = &command_info.redirect_out {
        let file = std::fs::File::create(output_file)?;
        if command_info.redirect_out_append {
            command.stdout(Stdio::from(file)); //TODO: Implement append
        } else {
            command.stdout(Stdio::from(file));
        }
    }

    if command_info.background {
        command.spawn()?;
        return Ok(());
    }

    let output = command.status()?;

    if output.success() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Command failed"))
    }
}
