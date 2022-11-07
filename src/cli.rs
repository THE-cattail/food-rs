use std::io::{self, Write};

use anyhow::Result;

/// # Errors
///
/// Will return `Err` if `io::stdin().read_line` failed.
pub fn ask_for_continue(desc: &str) -> Result<bool> {
    print!("Continue {desc}? (Y/n): ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    line = line.trim().to_owned();
    Ok(line != "n")
}
