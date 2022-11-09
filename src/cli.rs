use std::io::{self, Write};

use color_eyre::eyre;
use eyre::{Result, WrapErr};

/// # Errors
///
/// Will return `Err` if `io::stdin().read_line` failed.
pub fn ask_for_continue(desc: &str) -> Result<bool> {
    print!("{desc} (Y/n): ");
    io::stdout()
        .flush()
        .wrap_err("Unable to ask for continue")?;

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .wrap_err("Unable to ask for continue")?;

    Ok(line.trim() != "n")
}
