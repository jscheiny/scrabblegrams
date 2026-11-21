use std::{
    error::Error,
    fs::File,
    io::{BufReader, prelude::*},
};

use itertools::Itertools;

use crate::puzzle_square::PuzzleSquare;

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
                let square = PuzzleSquare::from(symbol, letter).unwrap();
                row.push(square);
            }
            grid.push(row);
        }

        for row in &grid {
            for square in row {
                print!("{square}");
            }
            println!();
        }

        Ok(Puzzle { grid })
    }
}
