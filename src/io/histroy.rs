use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn load_history() -> Vec<String> {
    let home_dir = env::var("HOME").unwrap();
    let history_file = Path::new(&home_dir).join(".mash_history");

    if let Ok(file) = File::open(history_file) {
        BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .collect()
    } else {
        Vec::new()
    }
}

pub fn save_history(history: &Vec<String>) {
    let home_dir = env::var("HOME").unwrap();
    let history_file = Path::new(&home_dir).join(".mash");

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .write(true)
        .open(history_file)
    {
        for line in history {
            writeln!(file, "{}", line).unwrap();
        }
    }
}

pub fn get_previous_history_item(history: &Vec<String>, current_index: usize) -> Option<String> {
    if current_index > 0 {
        history.get(current_index - 1).cloned()
    } else {
        None
    }
}

pub fn get_next_history_item(history: &Vec<String>, current_index: usize) -> Option<String> {
    if current_index < history.len() - 1 {
        history.get(current_index + 1).cloned()
    } else {
        None
    }
}
