use std::{
    error::Error,
    fs::File,
    io::{BufReader, prelude::*},
};

use crate::{
    alphabet::{ALPHABET_LEN, LetterIndex, get_letter_index},
    bit_set::LetterSet,
};

pub struct Dawg {
    nodes: Vec<DawgNode>,
}

pub struct DawgNode {
    pub is_end_of_word: bool,
    pub children: LetterSet,
    children_indices: Vec<usize>,
    children_by_suffix_length: Vec<LetterSet>,
}

impl Dawg {
    pub fn read(path: &str) -> Result<Dawg, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut dawg = Dawg::new();
        for line in reader.lines() {
            let word = line?;
            dawg.add(word.as_str());
        }

        Ok(dawg)
    }

    fn new() -> Dawg {
        let root = DawgNode::new();
        Dawg { nodes: vec![root] }
    }

    pub fn node(&self, index: usize) -> &DawgNode {
        &self.nodes[index]
    }

    pub fn add(&mut self, word: &str) {
        self.add_impl(word, 0);
    }

    pub fn nodes(&self) -> usize {
        self.nodes.len()
    }

    fn add_impl(&mut self, suffix: &str, node_index: usize) {
        if suffix.is_empty() {
            self.nodes[node_index].is_end_of_word = true;
            return;
        }

        let letter: char = suffix.chars().next().unwrap();
        let letter_index = get_letter_index(letter);
        self.nodes[node_index].put_child_with_suffix_length(letter_index, suffix.len());

        let nodes_len = self.nodes.len();
        let node = self.node(node_index);

        if node.has_child(letter_index) {
            self.add_impl(&suffix[1..], node.child(letter_index));
        } else {
            let node = &mut self.nodes[node_index];
            let new_node = DawgNode::new();
            node.put_child(letter_index, nodes_len, suffix.len());
            self.nodes.push(new_node);
            self.add_impl(&suffix[1..], nodes_len);
        }
    }
}

impl DawgNode {
    fn new() -> DawgNode {
        DawgNode {
            is_end_of_word: false,
            children_indices: vec![0; ALPHABET_LEN],
            children: LetterSet::new(),
            // TODO this should take max word length into account
            children_by_suffix_length: vec![LetterSet::new(); 16],
        }
    }

    pub fn child(&self, letter_index: LetterIndex) -> usize {
        self.children_indices[letter_index as usize]
    }

    pub fn has_child(&self, letter_index: LetterIndex) -> bool {
        self.children_indices[letter_index as usize] != 0
    }

    pub fn has_child_with_suffix_length(
        &self,
        letter_index: LetterIndex,
        suffix_length: usize,
    ) -> bool {
        self.children_by_suffix_length[suffix_length].contains(letter_index as usize)
    }

    pub fn children(&self, suffix_length: usize) -> LetterSet {
        self.children_by_suffix_length[suffix_length]
    }

    fn put_child(&mut self, letter_index: LetterIndex, child: usize, suffix_length: usize) {
        self.children_indices[letter_index as usize] = child;
        self.children.add(letter_index as usize);
        self.children_by_suffix_length[suffix_length].add(letter_index as usize);
    }

    fn put_child_with_suffix_length(&mut self, letter_index: LetterIndex, suffix_length: usize) {
        self.children_by_suffix_length[suffix_length].add(letter_index as usize);
    }
}
