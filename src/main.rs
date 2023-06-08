use std::hash::Hash;

use huff_and_puff::types::*;
use huff_and_puff::funcs::*;

// We want something that can generate a Huffman coding tree given some text
//   Something that can encode some text given a tree
//   Something that can decode a HCode given a tree

/// What do I want the api to look like?
/// 
/// I want a program that can encode and decode Huffman messages
///     fn encode_message(message: &str, table: &Table) -> HCode
///     fn decode_message(hcode: &HCode, tree: &Tree) -> String
/// This can be a struct - the Table and Tree are the data.
/// 
/// I want a program that can make an optimal Huffman encoding scheme for some text
/// 
/// Let's make it generic. The symbols are generic - only data structure I need is a map from symbol to probability. 
/// 
/// 
/// 

struct Huffman<T> {
    tree: HuffmanTree<T>,
    table: HuffmanTable<T>,
}

impl<T> Huffman<T>
where
    T: Copy + Eq + Hash
{
    fn from_iter(s: impl IntoIterator<Item = T>) -> Self {
        let symbol_probs = symbol_probabilities(s);
        let tree = generate_tree(symbol_probs);
        let table = generate_table(&tree);
        Huffman { tree, table }
    }
}

fn main() {
    let symbols = "aaaabbbccd";
    let huffman = Huffman::from_iter(symbols.chars());
    println!("{:#?}", huffman.tree);
    println!("{:#?}", huffman.table);
}