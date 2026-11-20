use crate::{
    alphabet::{LetterIndex, get_letter},
    dawg::{Dawg, DawgNode},
    puzzle::Puzzle,
};

mod alphabet;
mod dawg;
mod letter_set;
mod puzzle;

fn main() {
    let dawg = Dawg::read("lexicon.txt").unwrap();
    println!("{}", dawg.nodes());
    Puzzle::read("puzzle.txt");
}
