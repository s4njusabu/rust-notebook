// Enable and disable raw mode

use crossterm::terminal;
use std::io;

fn main() -> io::Result<()> {

    // Enables raw mode
    terminal::enable_raw_mode()?;

    // Disables raw mode
    terminal::disable_raw_mode()?;
    
    Ok(())
}