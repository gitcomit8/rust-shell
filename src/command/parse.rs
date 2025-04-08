#[derive(Debug)]
pub struct CommandInfo {
    pub command: String,
    pub args: Vec<String>,
    pub redirect_in: Option<String>,
    pub redirect_out: Option<String>,
    pub redirect_out_append: bool,
    pub background: bool,
}

pub fn parse_input(input: &str) -> Vec<CommandInfo> {
    let mut commands = Vec::new();
    let mut current_command = CommandInfo {
        command: String::new(),
        args: Vec::new(),
        redirect_in: None,
        redirect_out: None,
        redirect_out_append: false,
        background: false,
    };

    let mut parts = input.trim().split_whitespace().peekable();

    while let Some(part) = parts.next() {
        match part {
            "<" => {
                if let Some(filename) = parts.next() {
                    current_command.redirect_in = Some(filename.to_string());
                }
            }
            ">" => {
                if let Some(filename) = parts.next() {
                    current_command.redirect_out = Some(filename.to_string());
                    current_command.redirect_out_append = false;
                }
            }
            ">>" => {
                if let Some(filename) = parts.next() {
                    current_command.redirect_out = Some(filename.to_string());
                    current_command.redirect_out_append = true;
                }
            }
            "&" => {
                current_command.background = true;
            }
            "|" => {
                // Handle pipes (not fully implemented here)
                commands.push(current_command);
                current_command = CommandInfo {
                    command: String::new(),
                    args: Vec::new(),
                    redirect_in: None,
                    redirect_out: None,
                    redirect_out_append: false,
                    background: false,
                };
            }
            _ => {
                if current_command.command.is_empty() {
                    current_command.command = part.to_string();
                } else {
                    current_command.args.push(part.to_string());
                }
            }
        }
    }

    commands.push(current_command); // Push the last command
    commands
}
