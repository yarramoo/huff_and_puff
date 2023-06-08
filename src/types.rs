use std::collections::HashMap;

#[derive(Debug)]
pub enum HuffmanTree<T>
{
    Node { 
        p: f64, 
        left: Box<Self>, 
        right: Box<Self> 
    },
    Leaf { 
        p: f64, 
        symbol: T 
    }
}

impl<T> HuffmanTree<T> 
{
    pub fn prob(&self) -> f64 {
        match self {
            Self::Node { p, left: _, right: _ } => *p,
            Self::Leaf { p, symbol: _ }         => *p,
        }
    }

    pub fn new_node(p: f64, left: Self, right: Self) -> Self {
        let left = Box::new(left);
        let right = Box::new(right);
        Self::Node { p, left, right }
    }

    pub fn new_leaf(p: f64, symbol: T) -> Self {
        Self::Leaf { p , symbol }
    }

    pub fn left(&self) -> Option<&Self> {
        match self {
            HuffmanTree::Node { p:_, left, right:_ } => Some(&left),
            HuffmanTree::Leaf { p:_, symbol:_ } => None,
        }
    }

    pub fn right(&self) -> Option<&Self> {
        match self {
            HuffmanTree::Node { p:_, left:_, right } => Some(&right),
            HuffmanTree::Leaf { p:_, symbol:_ } => None,
        }
    }
}

pub type HuffmanTable<T> = HashMap<T, Vec<Bit>>;

#[derive(Clone, Copy, Debug)]
pub enum Bit { L, R }

pub type HuffmanCode = Vec<Bit>;