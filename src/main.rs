use std::fmt::Debug;
use std::{cmp::Reverse, fmt::Display};
use std::collections::HashMap;
use std::hash::Hash;
use display_tree::{AsTree, CharSet, DisplayTree, StyleBuilder};

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
use symbols::SymbolDistribution;

#[derive(Debug)]
enum HuffmanTree<T>
{
    Node { 
        p: f64, 
        left_tree: Box<Self>, 
        right_tree: Box<Self> },
    Leaf { 
        p: f64, 
        symbol: T 
    }
}

impl<T> HuffmanTree<T> 
{
    fn prob(&self) -> f64 {
        match self {
            HuffmanTree::Node { p, left_tree, right_tree } => *p,
            HuffmanTree::Leaf { p, symbol }                => *p,
        }
    }

    fn new_node(p: f64, left_tree: Self, right_tree: Self) -> Self {
        let left_tree = Box::new(left_tree);
        let right_tree = Box::new(right_tree);
        HuffmanTree::Node { p, left_tree, right_tree }
    }
}

type HuffmanTable<T> = HashMap<T, Vec<Bit>>;

#[derive(Clone, Debug)]
enum Bit { L, R }

struct Huffman<T> {
    tree: HuffmanTree<T>,
    table: HuffmanTable<T>,
}

impl<T> Huffman<T>
where
    T: Copy + Eq + Hash
{
    fn from_symbol_dist(symbol_dist: SymbolDistribution<T>) -> Self {
        // Sort the symbols from most to least probable
        // Map into HuffmanTree::Leafs
        // Merge the trailing trees recursively until one tree remains
        // Generate HuffmanTable from the tree
        let mut symbols: Vec<_> = symbol_dist.iter().collect();
        symbols.sort_by(|a, b| b.1.total_cmp(a.1));
        
        let mut trees: Vec<_> = symbols
            .iter()
            .map(|(&s, &p)| HuffmanTree::Leaf { p , symbol: s })
            .collect();

        while trees.len() > 1 {
            let tree_a = trees.pop().unwrap();
            let tree_b = trees.pop().unwrap();
            let combined_prob = tree_a.prob() + tree_b.prob();
            let merged_tree = HuffmanTree::new_node(combined_prob, tree_a, tree_b);
            let index = trees.binary_search_by(|probe| merged_tree.prob().total_cmp(&probe.prob()));
            let index = match index {
                Ok(i) => i,
                Err(i) => i,
            };
            trees.insert(index, merged_tree);
        }

        let tree = trees.pop().unwrap();

        let mut table = HashMap::new();
        let mut code = Vec::new();
        Huffman::generate_table(&tree, &mut table, &mut code);

        Huffman { tree, table }
    }

    fn generate_table(tree: &HuffmanTree<T>, table: &mut HuffmanTable<T>, code: &mut Vec<Bit>) {
        use self::*;
        match tree {
            HuffmanTree::Node { p, left_tree, right_tree } => {
                code.push(Bit::L);
                Huffman::generate_table(&left_tree, table, code);
                code.pop();

                code.push(Bit::R);
                Huffman::generate_table(&right_tree, table, code);
                code.pop();
            },
            HuffmanTree::Leaf { p, symbol } => {
                table.insert(symbol.clone(), code.clone());
            },
        }
    }
}

pub mod symbols {
    use std::collections::HashMap;
    use std::hash::Hash;

    pub type SymbolDistribution<T> = HashMap<T,f64>;

    pub fn from_symbol_iter<T>(symbols: impl IntoIterator<Item = T>) -> SymbolDistribution<T>
    where 
      T: Eq + Copy + Hash
    {
        let mut count = 0;
        let mut occurances = HashMap::new();
        for symbol in symbols.into_iter() {
            *occurances.entry(symbol).or_insert(0.) += 1.;
            count += 1;
        }
        for (_, v) in occurances.iter_mut() {
            *v = *v / count as f64;
        }
        return occurances; 
    }
    
    /// Cursed version that only allocates one HashMap and mem::translates the u64 counts into f64 probabilities
    /// because why not
    /// Should be safe because f64 and u64 take up the same space in memory
    /// I would do this because incrementing with ints is always accurate. I could also just use floats in the occurance count 
    /// but that's lame
    #[allow(dead_code)]
    fn from_symbols_iter_unsafe<T>(symbols: impl IntoIterator<Item = T>) -> SymbolDistribution<T>
    where 
      T: Eq + Copy + Hash
    {
        let mut count = 0;
        let mut occurances: HashMap<T, u64> = HashMap::new();
        for symbol in symbols.into_iter() {
            *occurances.entry(symbol).or_insert(0) += 1;
            count += 1;
        }
        for v in occurances.values_mut() {
            let occurs_float = *v as f64 / count as f64;
            *v = unsafe { std::mem::transmute(occurs_float) };
        }
        unsafe { std::mem::transmute(occurances) }
    }
    
    pub fn map_hashmap(map: HashMap<usize,u64>) -> HashMap<usize, f64> {
        unsafe { std::mem::transmute(map) }
    }
}






fn main() {
    let symbols = "aaaabbbccd";
    let symbol_dist = symbols::from_symbol_iter(symbols.chars());
    let tree = Huffman::from_symbol_dist(symbol_dist);

    // println!("{:?}", AsTree::new(&tree.tree));
    println!("{:#?}", tree.tree);
    println!("{:#?}", tree.table);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_symbols() {
        let symbols = "abcdefgabcdaba";
        let symbol_freqs = from_symbol_iter(symbols.chars());
        println!("{:#?}", symbol_freqs);
    }
}
