pub mod board;
pub mod system;
pub mod pathfind;

use crate::system::play_sequential;
use clearscreen::clear;

/// Main runner which initializes a game based on user input
fn main() {
    let _ = clear();
    let mut input: String = String::new();
    println!("Number of rings:");
    let _ = std::io::stdin().read_line(&mut input);
    let rings: usize = input
        .trim()
        .parse()
        .expect("Could not parse arguments");
    let mut input: String = String::new();
    println!("Number of towers:");
    let _ = std::io::stdin().read_line(&mut input);
    let towers: usize = input
        .trim()
        .parse()
        .expect("Could not parse arguments");
    println!("Mode (options: seq/auto) (default: seq):");
    let mut input: String = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    if input.trim() == "seq" { play_sequential(rings, towers); } 
}
