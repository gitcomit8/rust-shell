use std::env;
use std::fs;

pub fn get_command_completions(input: &str) -> Vec<String> {
    let path_env = env::var("PATH").unwrap_or_default();
    let paths = env::split_paths(&path_env);
    let mut commands = Vec::new();

    for path in paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(name) = entry.file_name().to_str() {
                                if name.starts_with(input) {
                                    commands.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Add built-in commands
    let built_in_commands = vec![
        "cd", "exit", "history", // ... other built-in commands
    ];
    commands.extend(
        built_in_commands
            .into_iter()
            .filter(|cmd| cmd.starts_with(input))
            .map(|cmd| cmd.to_string()),
    );
    commands
}

pub fn get_file_path_completions(input: &str) -> Vec<String> {
    let current_dir = env::current_dir().unwrap();
    let mut completions = Vec::new();
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(input) {
                        completions.push(name.to_string());
                    }
                }
            }
        }
    }
    completions
}

// This is a placeholder for the completion handler function.
// You'll need to integrate this with your main loop and a library
// like `crossterm` to handle terminal input and output correctly.
pub fn handle_completion(input: &str) {
    let mut parts = input.trim().split_whitespace();
    let last_word = parts.next_back().unwrap_or(""); // Get the last word

    if input.contains(" ") {
        // Complete file paths
        let completions = get_file_path_completions(last_word);
        for completion in completions {
            println!("{}", completion);
        }
    } else {
        // Complete commands
        let completions = get_command_completions(last_word);
        for completion in completions {
            println!("{}", completion);
        }
    }
}
