use std::io::{self, Write};

// Utility function to handle errors and print them to stderr
pub fn print_error(message: &str) {
    eprintln!("Error: {}", message);
    io::stderr().flush().unwrap(); // Ensure the error is printed immediately
}

// Add more utility functions as needed (e.g., string manipulation, etc.)
