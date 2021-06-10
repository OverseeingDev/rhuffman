use std::{cmp::Ordering, usize};
use WeightedHuffmanNode::*;

/// This node is used while building the Huffman tree, it
/// keeps track of its weight but should not be used for children nodes as weight is useless for children nodes
#[derive(PartialEq, Eq, Debug)]
pub enum WeightedHuffmanNode<T>
where
    T: PartialEq + Eq,
{
    Leaf(WeightedHuffmanLeaf<T>),
    Branch(WeightedHuffmanBranch<T>),
}

impl<T> WeightedHuffmanNode<T>
where
    T: PartialEq + Eq,
{
    pub fn get_weight(&self) -> usize {
        match self {
            Leaf(leaf) => leaf.weight,
            Branch(branch) => branch.weight,
        }
    }

    pub fn into_leaf(symbol: T, weight: usize) -> WeightedHuffmanNode<T> {
        Leaf(WeightedHuffmanLeaf { symbol, weight })
    }

    pub fn into_branch(
        greater: WeightedHuffmanNode<T>,
        lower: WeightedHuffmanNode<T>,
    ) -> WeightedHuffmanNode<T> {
        Branch(WeightedHuffmanBranch::new(greater, lower))
    }
}

impl<T> PartialOrd for WeightedHuffmanNode<T>
where
    T: PartialEq + Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Need to implement some arbitrary ord so that equally weighted branches are chosen deterministically
impl<T> Ord for WeightedHuffmanNode<T>
where
    T: PartialEq + Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.get_weight().cmp(&other.get_weight());
        if let Ordering::Equal = ordering {
            match (self, other) {
                (Leaf(_), Branch(_)) => Ordering::Greater,
                (Branch(_), Leaf(_)) => Ordering::Less,
                (Leaf(me), Leaf(other)) => me.symbol.cmp(&other.symbol),
                (Branch(me), Branch(other)) => me.links.0.cmp(&other.links.0),
            }
        } else {
            ordering
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct WeightedHuffmanLeaf<T>
where
    T: PartialEq + Eq,
{
    pub weight: usize,
    pub symbol: T,
}
#[derive(PartialEq, Eq, Debug)]
pub struct WeightedHuffmanBranch<T>
where
    T: PartialEq + Eq,
{
    pub weight: usize,
    pub links: (Box<HuffmanNode<T>>, Box<HuffmanNode<T>>),
}

impl<T> WeightedHuffmanBranch<T>
where
    T: PartialEq + Eq,
{
    pub fn new(
        greater: WeightedHuffmanNode<T>,
        lower: WeightedHuffmanNode<T>,
    ) -> WeightedHuffmanBranch<T> {
        WeightedHuffmanBranch {
            weight: greater.get_weight() + lower.get_weight(),
            links: (Box::new(greater.into()), Box::new(lower.into())),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum HuffmanNode<T>
where
    T: PartialEq + Eq,
{
    Leaf(HuffmanLeaf<T>),
    Branch(HuffmanBranch<T>),
}

impl<T> From<WeightedHuffmanNode<T>> for HuffmanNode<T>
where
    T: PartialEq + Eq,
{
    fn from(weighted: WeightedHuffmanNode<T>) -> Self {
        match weighted {
            Leaf(leaf) => HuffmanNode::Leaf(HuffmanLeaf {
                symbol: leaf.symbol,
            }),
            Branch(WeightedHuffmanBranch { links, .. }) => {
                HuffmanNode::Branch(HuffmanBranch { links })
            }
        }
    }
}
impl<T> PartialOrd for HuffmanNode<T>
where
    T: PartialEq + Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Some arbitrary ord so that equally weighted branches are chosen deterministically
impl<T> Ord for HuffmanNode<T>
where
    T: PartialEq + Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HuffmanNode::Leaf(_), HuffmanNode::Branch(_)) => Ordering::Greater,
            (HuffmanNode::Branch(_), HuffmanNode::Leaf(_)) => Ordering::Less,
            (HuffmanNode::Leaf(me), HuffmanNode::Leaf(other)) => me.symbol.cmp(&other.symbol),
            (HuffmanNode::Branch(me), HuffmanNode::Branch(other)) => me.links.0.cmp(&other.links.0),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct HuffmanLeaf<T>
where
    T: PartialEq + Eq,
{
    pub symbol: T,
}

#[derive(PartialEq, Eq, Debug)]
pub struct HuffmanBranch<T>
where
    T: PartialEq + Eq,
{
    pub links: (Box<HuffmanNode<T>>, Box<HuffmanNode<T>>),
}
