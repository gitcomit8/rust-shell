use crate::command::execute::execute_command;
use crate::command::parse::CommandInfo;

pub fn handle_and_logic(commands: Vec<CommandInfo>) {
    let mut previous_result = Ok(()); // Initialize with success

    for command_info in commands {
        if previous_result.is_ok() {
            previous_result = execute_command(command_info, &mut None); // Assuming no pipe here
        } else {
            break; // Stop executing if a command failed
        }
    }
}
