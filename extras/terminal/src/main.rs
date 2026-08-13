// Clear entire screen

use std::io;
use crossterm::{self, terminal::{self, Clear}};

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();    

    crossterm::execute!(stdout, Clear(terminal::ClearType::All))?;

    terminal::disable_raw_mode()?;


    Ok(())
}