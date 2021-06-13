use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use HuffmanNode::*;
/// This node is used while building the Huffman tree, it
/// keeps track of its weight but should not be used for children nodes as weight is useless for children nodes

#[derive(PartialEq, Eq, Debug)]
pub struct Weighted<T: Eq>(HuffmanNode<T>, u64);

impl<T> Weighted<T>
where
    T: PartialEq + Eq,
{
    pub fn get_weight(&self) -> u64 {
        self.1
    }

    pub fn new_leaf(symbol: T, weight: u64) -> Weighted<T> {
        Weighted(HuffmanNode::Leaf(HuffmanLeaf { symbol }), weight)
    }

    pub fn new_branch(greater: Weighted<T>, lower: Weighted<T>) -> Weighted<T> {
        let sum_of_weights = greater.get_weight() + lower.get_weight();
        Weighted(
            HuffmanNode::Branch(HuffmanBranch {
                links: (Box::new(greater.into()), Box::new(lower.into())),
            }),
            sum_of_weights,
        )
    }
}

impl<T> PartialOrd for Weighted<T>
where
    T: PartialEq + Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Need to implement some arbitrary ord so that equally weighted branches are chosen deterministically
impl<T> Ord for Weighted<T>
where
    T: PartialEq + Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.get_weight().cmp(&other.get_weight());
        if let Ordering::Equal = ordering {
            match (self, other) {
                (Weighted(Leaf(_), ..), Weighted(Branch(_), ..)) => Ordering::Greater,
                (Weighted(Branch(_), ..), Weighted(Leaf(_), ..)) => Ordering::Less,
                (Weighted(Leaf(me), ..), Weighted(Leaf(other), ..)) => me.symbol.cmp(&other.symbol),
                (Weighted(Branch(me), ..), Weighted(Branch(other), ..)) => {
                    me.links.0.cmp(&other.links.0)
                }
            }
        } else {
            ordering
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HuffmanNode<T>
where
    T: PartialEq + Eq,
{
    Leaf(HuffmanLeaf<T>),
    Branch(HuffmanBranch<T>),
}

impl<T: PartialEq + Eq> HuffmanNode<T> {}

impl<T> From<Weighted<T>> for HuffmanNode<T>
where
    T: PartialEq + Eq,
{
    fn from(weighted: Weighted<T>) -> Self {
        match weighted {
            Weighted(Leaf(leaf), ..) => HuffmanNode::Leaf(HuffmanLeaf {
                symbol: leaf.symbol,
            }),
            Weighted(Branch(HuffmanBranch { links }), ..) => {
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

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HuffmanLeaf<T>
where
    T: PartialEq + Eq,
{
    pub symbol: T,
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HuffmanBranch<T>
where
    T: PartialEq + Eq,
{
    pub links: (Box<HuffmanNode<T>>, Box<HuffmanNode<T>>),
}
