pub const ALPHABET_LEN: usize = 26;
pub type LetterIndex = u8;

const BASE_LETTER: LetterIndex = b'A';
pub fn get_letter_index(letter: char) -> LetterIndex {
    (letter as LetterIndex) - BASE_LETTER
}

pub fn get_letter(index: LetterIndex) -> char {
    (BASE_LETTER + index) as char
}
