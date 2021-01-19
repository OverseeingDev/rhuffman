use std::{cmp::Ordering, usize};
use HuffmanNode::*;

#[derive(Debug, PartialEq, Eq)]
pub enum HuffmanNode<T>
where
    T: PartialEq + Eq,
{
    Leaf(HuffmanLeaf<T>),
    Branch(HuffmanBranch<T>),
}

impl<T> HuffmanNode<T>
where
    T: PartialEq + Eq,
{
    pub fn get_weight(&self) -> usize {
        match self {
            Leaf(leaf) => leaf.weight,
            Branch(branch) => branch.weight,
        }
    }

    pub fn into_leaf(symbol: T, weight: usize) -> HuffmanNode<T> {
        Leaf(HuffmanLeaf { symbol, weight })
    }

    pub fn into_branch(greater: HuffmanNode<T>, lower: HuffmanNode<T>) -> HuffmanNode<T> {
        Branch(HuffmanBranch::new(greater, lower))
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

// Need to implement some arbitrary ord so that different branches are not completely equal
impl<T> Ord for HuffmanNode<T>
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

#[derive(Debug, PartialEq, Eq)]
pub struct HuffmanLeaf<T>
where
    T: PartialEq + Eq,
{
    weight: usize,
    symbol: T,
}
#[derive(Debug, PartialEq, Eq)]
pub struct HuffmanBranch<T>
where
    T: PartialEq + Eq,
{
    weight: usize,
    links: (Box<HuffmanNode<T>>, Box<HuffmanNode<T>>),
}

impl<T> HuffmanBranch<T>
where
    T: PartialEq + Eq,
{
    pub fn new(greater: HuffmanNode<T>, lower: HuffmanNode<T>) -> HuffmanBranch<T> {
        HuffmanBranch {
            weight: greater.get_weight() + lower.get_weight(),
            links: (Box::new(greater), Box::new(lower)),
        }
    }
}
