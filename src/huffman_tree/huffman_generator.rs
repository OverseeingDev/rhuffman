use std::{
    collections::{BTreeSet, HashMap},
    fmt::Debug,
    hash::Hash,
};

use super::huffman_element::HuffmanNode;

pub struct HuffmanGenerator<T>
where
    T: Eq + Hash,
{
    symbols: HashMap<T, usize>,
}

impl<T> HuffmanGenerator<T>
where
    T: Eq + Hash + Clone + Debug + Ord,
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

        let mut symbols = BTreeSet::new();
        for (symbol, count) in self.symbols.into_iter() {
            println!("inserted {:?}", symbol);
            symbols.insert(HuffmanNode::into_leaf(symbol, count));
            println!("length after insertion {:?}", symbols.len());
        }

        while symbols.len() > 1 {
            let lower = symbols.pop_first().unwrap();
            let greater = symbols.pop_first().unwrap();

            symbols.insert(HuffmanNode::into_branch(greater, lower));
        }

        Some(symbols.pop_first().unwrap())
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
