use std::hash::Hash;
use std::collections::HashMap;

use crate::types::*;

/// Make a Vector of symbols and their probabilities out of a collection of symbols by counting
/// how often they occur
pub fn symbol_probabilities<T>(symbols: impl IntoIterator<Item = T>) -> Vec<(T, f64)>
where 
    T: Eq + Copy + Hash
{
    let mut count = 0;
    let mut occurances = HashMap::new();
    // Collect a map of occurances
    for symbol in symbols.into_iter() {
        *occurances.entry(symbol).or_insert(0) += 1;
        count += 1;
    }
    // Collect into a Vec
    let symbol_distr: Vec<_> = occurances
        .into_iter()
        .map(|(k, v)| (k, v as f64 / count as f64))
        .collect();
    
    symbol_distr
}

/// Create a Huffman Tree out of a Vector of symbols and their probabilities
pub fn generate_tree<T>(mut symbol_vec: Vec<(T,f64)>) -> HuffmanTree<T> {
    // Sort the vector in descending order
    symbol_vec.sort_by(|a, b| b.1.total_cmp(&a.1));
    // Map all into Leafs
    let mut trees: Vec<_> = symbol_vec
        .into_iter()
        .map(|(s, p)| HuffmanTree::new_leaf(p, s))
        .collect();
    // Amalgamate into a single tree
    while trees.len() > 1 {
        // Create new merged tree out of two least probable
        let tree_a = trees.pop().unwrap();
        let tree_b = trees.pop().unwrap();
        let p = tree_a.prob() + tree_b.prob();
        let new_tree = HuffmanTree::new_node(p, tree_a, tree_b);
        // Insert new tree back into the sorted list of trees
        let index = trees.binary_search_by(|probe| new_tree.prob().total_cmp(&probe.prob()));
        let index = match index {
            Ok(i) => i,
            Err(i) => i,
        };
        trees.insert(index, new_tree);
    }

    trees.pop().unwrap()
}

/// Make a Huffman table for quick encoding using a Huffman tree
pub fn generate_table<T>(tree: &HuffmanTree<T>) -> HuffmanTable<T> 
where
    T: Eq + Clone + Hash
{
    let mut table = HuffmanTable::new();
    let mut code = HuffmanCode::new();
    generate_table_recursive(tree, &mut table, &mut code);
    table
}

fn generate_table_recursive<T>(
    tree: &HuffmanTree<T>, 
    table: &mut HuffmanTable<T>, 
    code: &mut HuffmanCode) 
where
    T: Eq + Clone + Hash
{
    match tree {
        HuffmanTree::Node { p: _, left, right } => {
            code.push(Bit::L);
            generate_table_recursive(&left, table, code);
            code.pop();

            code.push(Bit::R);
            generate_table_recursive(&right, table, code);
            code.pop();
        },
        HuffmanTree::Leaf { p: _, symbol } => {
            table.insert(symbol.clone(), code.clone());
        },
    }
}