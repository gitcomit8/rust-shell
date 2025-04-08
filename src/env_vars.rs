use std::collections::HashMap;
use std::env;

use thiserror::Error; // Assuming you're using thiserror for error handling

#[derive(Error, Debug)]
enum EnvVarError {
    #[error("Environment variable not found: {0}")]
    NotFound(String),
    #[error("Failed to set environment variable: {0}")]
    SetError(String),
}

type Result<T> = std::result::Result<T, EnvVarError>;

pub fn get_env_var(name: &str) -> Result<String> {
    env::var(name).map_err(|_| EnvVarError::NotFound(name.to_string()))
}

pub fn set_env_var(name: &str, value: &str) -> Result<()> {
    unsafe {
        env::set_var(name, value);
    }
    // In the standard library, `env::set_var` doesn't return a Result,
    // so we assume success.  However, in a more complex scenario,
    // you might need to check for errors in some way (platform-specific).
    Ok(())
}

pub fn unset_env_var(name: &str) -> Result<()> {
    unsafe {
        env::remove_var(name);
    }
    Ok(()) // Similar to set_env_var, no explicit error return
}

pub fn expand_env_vars(input: &str) -> String {
    let mut result = String::new();
    let mut var_start = None;

    for (i, c) in input.char_indices() {
        if c == '$' && var_start.is_none() {
            var_start = Some(i + 1);
        } else if c.is_whitespace() && var_start.is_some() {
            // Variable name ends at whitespace
            let var_name = &input[var_start.unwrap()..i];
            match get_env_var(var_name) {
                Ok(value) => result.push_str(&value),
                Err(_) => result.push_str(""), // Or you could choose to leave the $VAR as is
            }
            result.push(c);
            var_start = None;
        } else if var_start.is_some() {
            // Inside a variable name
        } else {
            result.push(c);
        }
    }

    if let Some(start) = var_start {
        // Handle the case where a variable is at the end of the input
        let var_name = &input[start..];
        if let Ok(value) = get_env_var(var_name) {
            result.push_str(&value);
        }
    }

    result
}

pub fn get_all_env_vars() -> HashMap<String, String> {
    env::vars().collect()
}
