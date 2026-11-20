use crate::{
    alphabet::{LetterIndex, get_letter},
    dawg::{Dawg, DawgNode},
};

mod alphabet;
mod dawg;
mod letter_set;

fn main() {
    let dawg = Dawg::read("lexicon.txt").unwrap();
    println!("{}", dawg.nodes());
    traverse(&dawg);
}

fn traverse(dawg: &Dawg) {
    traverse_impl(dawg, dawg.node(0), &mut Vec::with_capacity(16));
}

fn traverse_impl(dawg: &Dawg, node: &DawgNode, prefix: &mut Vec<LetterIndex>) {
    if node.is_end_of_word {
        for index in prefix.iter() {
            print!("{}", get_letter(*index));
        }
        println!();
    }

    node.children.for_each(|letter_index| {
        let letter_index = letter_index as LetterIndex;
        prefix.push(letter_index);
        let child_node_index = node.child(letter_index);
        let child_node = dawg.node(child_node_index);
        traverse_impl(dawg, child_node, prefix);
        prefix.pop();
    });
}
