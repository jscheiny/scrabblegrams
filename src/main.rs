use crate::{dawg::Dawg, puzzle::Puzzle};

mod alphabet;
mod dawg;
mod letter_set;
mod puzzle;
mod puzzle_square;

fn main() {
    let dawg = Dawg::read("lexicon.txt").unwrap();
    println!("{}", dawg.nodes());
    Puzzle::read("puzzle.txt").unwrap();
}
