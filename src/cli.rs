use std::io::{self, Write};

use color_eyre::eyre::Result;

/// # Errors
///
/// Will return `Err` if `io::stdin().read_line` failed.
pub fn ask_for_continue(desc: &str) -> Result<bool> {
    print!("{desc} (Y/n): ");
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    Ok(line.trim() != "n")
}
