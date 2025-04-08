use std::fmt;

pub struct Colorizer {
    // You might want to add configuration options here later,
    // like whether to actually use colors or not.
}

impl Colorizer {
    pub fn new() -> Self {
        Colorizer {}
    }

    pub fn colorize<T: fmt::Display>(&self, text: T, color: &str) -> String {
        match color {
            "red" => format!("\x1b[31m{}\x1b[0m", text),
            "green" => format!("\x1b[32m{}\x1b[0m", text),
            "yellow" => format!("\x1b[33m{}\x1b[0m", text),
            "blue" => format!("\x1b[34m{}\x1b[0m", text),
            "magenta" => format!("\x1b[35m{}\x1b[0m", text),
            "cyan" => format!("\x1b[36m{}\x1b[0m", text),
            "white" => format!("\x1b[37m{}\x1b[0m", text),
            _ => format!("{}", text), // No color
        }
    }
}
