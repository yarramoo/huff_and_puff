use std::fmt::Display;
use std::hash::Hash;
use std::fs::File;
use std::io::{read_to_string, BufReader};

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
    T: Copy + Eq + Hash + Display
{
    fn from_iter(s: impl IntoIterator<Item = T>) -> Self {
        let symbol_probs = symbol_probabilities(s);
        let tree = generate_tree(symbol_probs);
        let table = generate_table(&tree);
        Huffman { tree, table }
    }

    fn encode(&self, symbols: impl IntoIterator<Item = T>) -> Result<HuffmanCode, &str> {
        let mut encoding = Vec::new();

        for s in symbols.into_iter() {
            let s_code = self.table.get(&s).expect(format!("Symbol: '{s}' not in table").as_str());
            encoding.extend_from_slice(&s_code);
        }

        Ok(encoding)
    }

    fn decode(&self, code: HuffmanCode) -> Result<Vec<T>, &str> {
        let mut walker = &self.tree;
        let mut message = Vec::new();

        for c in code.into_iter() {
            match c {
                Bit::L => walker = &walker.left().unwrap(),
                Bit::R => walker = &walker.right().unwrap(),
            }
            if let HuffmanTree::Leaf { p: _, symbol } = walker {
                message.push(symbol.clone());
                walker = &self.tree;
            }
        }

        Ok(message)
    }
}

fn main() -> std::io::Result<()> {
    let text_path = "alice_in_wonderland.txt";
    let f = File::open(text_path)?;
    let f = BufReader::new(f);
    let s = read_to_string(f)?;

    let huffman = Huffman::from_iter(s.split_ascii_whitespace());
    // println!("{:?}", huffman.tree);
    // println!("{:?}", huffman.table);
    let code = huffman.encode("I once was a white rabbit".split_ascii_whitespace()).unwrap();
    println!("{:?}", &code);
    println!("{:?}", huffman.decode(code));

    Ok(())
}