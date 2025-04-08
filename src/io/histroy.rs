use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Error as IoError, Write};
use std::path::Path;

use thiserror::Error; // For cleaner error handling (add this to Cargo.toml)

#[derive(Error, Debug)]
enum HistoryError {
    #[error("Failed to get HOME directory")]
    HomeDirNotFound,
    #[error("IO Error: {0}")]
    Io(#[from] IoError),
}

type Result<T> = std::result::Result<T, HistoryError>;

const HISTORY_FILE: &str = ".mash_history"; // Consistent filename

pub fn load_history() -> Result<Vec<String>> {
    let home_dir = env::var("HOME").map_err(|_| HistoryError::HomeDirNotFound)?;
    let history_file = Path::new(&home_dir).join(HISTORY_FILE);

    if let Ok(file) = File::open(history_file) {
        let reader = BufReader::new(file);
        reader
            .lines()
            .collect::<std::result::Result<Vec<String>, _>>()
            .map_err(HistoryError::Io)
    } else {
        Ok(Vec::new())
    }
}

pub fn save_history(history: &Vec<String>) -> Result<()> {
    let home_dir = env::var("HOME").map_err(|_| HistoryError::HomeDirNotFound)?;
    let history_file = Path::new(&home_dir).join(HISTORY_FILE);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true) // Append instead of overwrite
        .open(history_file)
        .map_err(HistoryError::Io)?;

    let mut writer = BufWriter::new(&mut file); // Buffer writes for efficiency

    for line in history {
        writeln!(writer, "{}", line).map_err(HistoryError::Io)?;
    }

    writer.flush().map_err(HistoryError::Io)?; // Ensure everything is written

    Ok(())
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
