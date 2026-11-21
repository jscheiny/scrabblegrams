use colored::{ColoredString, Colorize};
use std::fmt::Display;

use crate::alphabet::{LetterIndex, get_letter, get_letter_index};

#[derive(Debug)]
pub enum PuzzleSquare {
    Empty,
    Letter(PuzzleLetter),
}

impl PuzzleSquare {
    pub fn from(symbol: char, letter: char) -> Option<Self> {
        match symbol {
            ' ' => Some(Self::Empty),
            _ => Some(Self::Letter(PuzzleLetter {
                letter_index: get_letter_index(letter),
                status: PuzzleLetterStatus::from(symbol)?,
            })),
        }
    }
}

impl Display for PuzzleSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Letter(letter) => write!(f, "{letter}"),
        }
    }
}

#[derive(Debug)]
pub struct PuzzleLetter {
    pub letter_index: LetterIndex,
    pub status: PuzzleLetterStatus,
}

impl Display for PuzzleLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = get_letter(self.letter_index);
        write!(f, "{}", self.status.color(letter))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PuzzleLetterStatus {
    Right,
    Wrong,
    Move,
}

impl PuzzleLetterStatus {
    pub fn from(symbol: char) -> Option<Self> {
        match symbol {
            '+' => Some(PuzzleLetterStatus::Right),
            '-' => Some(PuzzleLetterStatus::Wrong),
            '*' => Some(PuzzleLetterStatus::Move),
            _ => None,
        }
    }

    fn color(self, letter: char) -> ColoredString {
        match self {
            Self::Right => letter.to_string().green(),
            Self::Wrong => letter.to_string().white(),
            Self::Move => letter.to_string().yellow(),
        }
    }
}
