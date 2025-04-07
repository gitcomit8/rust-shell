use std::io::{Write, stdin, stdout};

mod command;
mod env_vars; // Assuming you'll create this module
mod io;
mod logic;
mod ui;
mod util; // Assuming you'll create this module

use command::execute::execute_command; // Bring execute_command into scope
use command::parse::parse_input; // Bring parse_input into scope
use io::completion::handle_completion; // Hypothetical completion function
use io::history::{load_history, save_history}; // Bring history functions into scope
use ui::colors::Colorizer;

fn main() {
    let mut history = load_history();
    let mut colorizer = Colorizer::new();

    loop {
        print!("{}", colorizer.colorize("> ", "green")); // Use color for prompt
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin::read_line(&mut input).unwrap();

        let input = input.trim();

        // Handle history and completions (Conceptual - adapt as needed)
        if input.starts_with("!") {
            // Handle history recall (e.g., !10)
            // ... logic to fetch from history and execute
            continue;
        }

        if input.ends_with("\t") {
            handle_completion(input); // Handle tab completion
            continue;
        }

        let commands = parse_input(input);

        let mut previous_command = None;

        for command_info in commands {
            let result = execute_command(command_info, &mut previous_command);

            match result {
                Ok(_) => {} // Command executed successfully
                Err(e) => {
                    eprintln!("{}", colorizer.colorize(&format!("Error: {}", e), "red"));
                }
            }
        }

        //Need to implement background process logic here
        history.push(input.to_string());
        save_history(&history);
    }
}
