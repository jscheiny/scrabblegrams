// TODO Allow separate index type
#[derive(Debug, Clone, Copy)]
pub struct LetterSet(u64);

impl LetterSet {
    pub fn new() -> LetterSet {
        LetterSet(0)
    }

    pub fn from(value: u64) -> LetterSet {
        LetterSet(value)
    }

    pub fn add(&mut self, value: usize) {
        self.0 |= Self::flag(value)
    }

    pub fn without(&self, value: usize) -> LetterSet {
        LetterSet(self.0 & !Self::flag(value))
    }

    pub fn contains(&self, value: usize) -> bool {
        self.0 & Self::flag(value) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn decrement(&self) -> LetterSet {
        LetterSet(self.0 >> 1)
    }

    pub fn intersection(&self, other: LetterSet) -> LetterSet {
        LetterSet(self.0 & other.0)
    }

    pub fn invert(&self, max_value: usize) -> LetterSet {
        let max_flag = Self::flag(max_value);
        let full_ones = max_flag - 1;
        LetterSet(full_ones & !self.0)
    }

    pub fn for_each<F: FnMut(usize)>(&self, mut f: F) {
        let mut value = self.0;
        let mut index = 0;
        while value != 0 {
            let trailing = value.trailing_zeros() as usize;
            index += trailing;
            f(index);
            value >>= trailing + 1;
            index += 1;
        }
    }

    fn flag(value: usize) -> u64 {
        1 << value
    }
}
