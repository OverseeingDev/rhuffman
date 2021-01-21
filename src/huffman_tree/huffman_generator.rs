use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use super::huffman_element::HuffmanNode;
use std::cmp::Reverse;

pub struct HuffmanGenerator<T>
where
    T: Eq + Hash + Clone + Ord,
{
    symbols: HashMap<T, usize>,
}

impl<T> HuffmanGenerator<T>
where
    T: Eq + Hash + Clone + Ord,
{
    pub fn new() -> HuffmanGenerator<T> {
        HuffmanGenerator {
            symbols: HashMap::new(),
        }
    }

    pub fn add_occurences(&mut self, symbol: &T, occurences: usize) {
        let entry = self.symbols.get_mut(symbol);
        match entry {
            Some(count) => *count += occurences,
            None => {
                self.symbols.insert(symbol.clone(), occurences);
            }
        };
    }

    pub fn into_huffman_tree(self) -> Option<HuffmanNode<T>> {
        if self.symbols.len() == 0 {
            return None;
        }

        let mut symbols = BinaryHeap::new();
        for (symbol, count) in self.symbols.into_iter() {
            symbols.push(Reverse(HuffmanNode::into_leaf(symbol, count)));
        }

        while symbols.len() > 1 {
            let lower = symbols.pop().unwrap().0;
            let greater = symbols.pop().unwrap().0;

            symbols.push(Reverse(HuffmanNode::into_branch(greater, lower)));
        }

        Some(symbols.pop().unwrap().0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_invocation_of_add_occurences_adds_single_occurence_correctly() {
        let mut generator = HuffmanGenerator::new();
        generator.add_occurences(&"A", 2);
        generator.add_occurences(&"B", 4);

        assert_eq!(2, *generator.symbols.get(&"A").unwrap());
        assert_eq!(4, *generator.symbols.get(&"B").unwrap());
    }

    #[test]
    fn multiple_invocations_of_add_occurences_do_add_occurences() {
        let mut generator = HuffmanGenerator::new();
        generator.add_occurences(&"A", 2);
        generator.add_occurences(&"A", 2);

        assert_eq!(4, *generator.symbols.get(&"A").unwrap());
    }

    #[test]
    fn empty_generator_generates_empty_huffman_tree() {
        let generator: HuffmanGenerator<&str> = HuffmanGenerator::new();
        let tree = generator.into_huffman_tree();
        assert_eq!(tree, None);
    }

    #[test]
    fn single_symbol_generates_leaf_tree() {
        let mut generator = HuffmanGenerator::new();
        generator.add_occurences(&"A", 2);

        let tree = generator.into_huffman_tree().unwrap();

        assert_eq!(tree, HuffmanNode::into_leaf("A", 2));
    }

    #[test]
    fn two_symbols_generate_branch_tree() {
        let mut generator = HuffmanGenerator::new();
        generator.add_occurences(&"A", 2);
        generator.add_occurences(&"B", 4);
        generator.add_occurences(&"C", 1);

        let tree_weight = generator.into_huffman_tree().unwrap().get_weight();
        assert_eq!(7, tree_weight)
    }
}
