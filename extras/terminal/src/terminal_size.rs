// Find the size of the terminal

use crossterm::terminal;

fn main() {
    println!("{:?}", terminal::size());
}
