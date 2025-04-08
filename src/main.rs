use std::io::{Write, stdin, stdout};

mod command;
mod env_vars;
mod io;
mod logic;
mod ui;
mod util;

use command::execute::execute_command;
use command::parse::parse_input;
use env_vars::expand_env_vars;
use io::completion::handle_completion;
use io::history::{get_next_history_item, get_previous_history_item, load_history, save_history};
use ui::colors::Colorizer;
use util::print_error;

fn main() {
    // Initialize history and colorizer
    let mut history = load_history().unwrap_or_else(|e| {
        print_error(&format!("Failed to load history: {}", e));
        Vec::new()
    });
    let mut colorizer = Colorizer::new();
    let mut current_history_index = history.len();

    loop {
        // Print the prompt
        print!("{}", colorizer.colorize("> ", "green"));
        stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut input = input.trim().to_string();

        // Expand environment variables
        let expanded_input = expand_env_vars(&input);

        // Handle history recall
        if input.starts_with("!") {
            if let Ok(n) = input[1..].parse::<usize>() {
                if n > 0 && n <= history.len() {
                    let recalled_command = history[n - 1].clone();
                    println!("{}", recalled_command);
                    input = recalled_command;
                } else {
                    print_error("Invalid history number");
                    continue;
                }
            } else {
                print_error("Invalid history recall syntax");
                continue;
            }
        }

        // Handle tab completion
        if input.ends_with("\t") {
            handle_completion(&input);
            continue;
        }

        // Handle up/down arrow history navigation (basic)
        if input == "\u{1b}[A" {
            if current_history_index > 0 {
                current_history_index -= 1;
                print!("\r> {}\x1b[K", history[current_history_index]);
                stdout().flush().unwrap();
                input = history[current_history_index];
            }
            continue;
        } else if input == "\u{1b}[B" {
            if current_history_index < history.len() {
                current_history_index += 1;
                if current_history_index == history.len() {
                    print!("\r> \x1b[K");
                } else {
                    print!("\r> {}\x1b[K", history[current_history_index]);
                }
                stdout().flush().unwrap();
                if current_history_index < history.len() {
                    input = history[current_history_index];
                } else {
                }
            }
            continue;
        } else {
            current_history_index = history.len();
        }

        // Parse and execute commands
        let commands = parse_input(&expanded_input);
        let mut previous_command = None;

        for command_info in commands {
            let result = execute_command(command_info, &mut previous_command);
            match result {
                Ok(_) => {}
                Err(e) => {
                    print_error(&format!("Error: {}", e));
                }
            }
        }

        // Save history
        if !input.trim().is_empty() {
            history.push(input.to_string());
            if let Err(e) = save_history(&history) {
                print_error(&format!("Failed to save history: {}", e));
            }
        }
    }
}
