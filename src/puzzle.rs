use std::{
    error::Error,
    fs::File,
    io::{BufReader, prelude::*},
};

use itertools::Itertools;

#[derive(Debug)]
enum PuzzleSquare {
    Empty,
    Letter(PuzzleLetter),
}

#[derive(Debug)]
struct PuzzleLetter {
    letter: char,
    status: PuzzleLetterStatus,
}

#[derive(Debug)]
enum PuzzleLetterStatus {
    Right,
    Wrong,
    Move,
}

pub struct Puzzle {
    grid: Vec<Vec<PuzzleSquare>>,
}

impl Puzzle {
    pub fn read(path: &str) -> Result<Puzzle, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut grid = vec![];
        for line in reader.lines() {
            let mut row = vec![];
            for mut entry in &line?.chars().chunks(2) {
                let symbol = entry.next().unwrap();
                let letter = entry.next().unwrap();
                let square = get_puzzle_square(symbol, letter).unwrap();
                row.push(square);
            }
            grid.push(row);
        }

        Ok(Puzzle { grid })
    }
}

fn get_puzzle_square(symbol: char, letter: char) -> Option<PuzzleSquare> {
    match symbol {
        ' ' => Some(PuzzleSquare::Empty),
        _ => get_puzzle_letter_status(symbol)
            .and_then(|status| Some(PuzzleSquare::Letter(PuzzleLetter { letter, status }))),
    }
}

fn get_puzzle_letter_status(symbol: char) -> Option<PuzzleLetterStatus> {
    match symbol {
        '+' => Some(PuzzleLetterStatus::Right),
        '-' => Some(PuzzleLetterStatus::Wrong),
        '*' => Some(PuzzleLetterStatus::Move),
        _ => None,
    }
}
