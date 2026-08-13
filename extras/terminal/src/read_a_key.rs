// Read keyboard

use std::io;
use crossterm::{terminal, event};

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    let read = event::read()?;

    if let event::Event::Key(key_event) = read {
        match key_event.code {
            event::KeyCode::Char(c) => println!("You pressed {c}"),
            _ => println!("You pressed something"),
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}