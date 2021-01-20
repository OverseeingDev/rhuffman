use std::{collections::HashMap, fmt::Debug, hash::Hash};

use bit_vec::BitVec;

use super::huffman_element::HuffmanNode;
pub struct HuffmanEncoder<T: Eq> {
    symbols: HashMap<T, HuffmanCode>,
}

impl<T: Eq + Hash + Clone + Debug> HuffmanEncoder<T> {
    pub fn from_tree(tree: &HuffmanNode<T>) -> HuffmanEncoder<T> {
        let mut map = HashMap::new();

        HuffmanEncoder::visit_tree(&tree, HuffmanCode::new(), &mut map);
        println!("Map: {:?}", map);
        HuffmanEncoder { symbols: map }
    }

    fn visit_tree(
        tree: &HuffmanNode<T>,
        mut current_prefix: HuffmanCode,
        symbols: &mut HashMap<T, HuffmanCode>,
    ) {
        match tree {
            HuffmanNode::Leaf(leaf) => {
                symbols.insert(leaf.symbol.clone(), current_prefix);
            }
            HuffmanNode::Branch(branch) => {
                let mut left_prefix = current_prefix.clone();
                left_prefix.append_bit(false);
                current_prefix.append_bit(true);
                HuffmanEncoder::visit_tree(&branch.links.0, left_prefix, symbols);
                HuffmanEncoder::visit_tree(&branch.links.1, current_prefix, symbols);
            }
        }
    }

    pub fn encode(&self, iter: &mut dyn Iterator<Item = &T>) -> Result<BitVec, T> {
        let mut bitvec = BitVec::new();
        for symbol in iter {
            if let Some(code) = self.symbols.get(&symbol) {
                code.append_code_to_bitvec(&mut bitvec);
            } else {
                return Err(symbol.clone());
            }
        }
        Ok(bitvec)
    }
}

#[derive(Clone, Debug)]
struct HuffmanCode {
    code: BitVec,
}

impl HuffmanCode {
    pub fn new() -> HuffmanCode {
        HuffmanCode {
            code: BitVec::new(),
        }
    }

    fn append_bit(&mut self, bit: bool) {
        self.code.push(bit);
    }

    pub fn append_code_to_bitvec(&self, bitvec: &mut BitVec) {
        bitvec.append(&mut self.code.clone());
    }
}

#[cfg(test)]
mod tests {
    mod huffman_code {
        use super::super::*;

        #[test]
        fn appendbit_modifies_code_correctly() {
            let mut code = HuffmanCode {
                code: BitVec::new(),
            };

            code.append_bit(true);

            assert!(code.code.eq_vec(&[true]));
        }

        #[test]
        fn multiple_invocations_of_appendbit_result_in_correct_code() {
            let mut code = HuffmanCode {
                code: BitVec::new(),
            };
            code.append_bit(false);
            code.append_bit(true);

            assert!(code.code.eq_vec(&[false, true]));

            let mut code = HuffmanCode {
                code: BitVec::new(),
            };
            code.append_bit(false);
            code.append_bit(false);
            assert!(code.code.eq_vec(&[false, false]));

            let mut code = HuffmanCode {
                code: BitVec::new(),
            };
            code.append_bit(true);
            code.append_bit(true);
            assert!(code.code.eq_vec(&[true, true]));

            let mut code = HuffmanCode {
                code: BitVec::new(),
            };
            code.append_bit(true);
            code.append_bit(false);
            code.append_bit(true);
            code.append_bit(true);
            assert!(code.code.eq_vec(&[true, false, true, true]));
        }

        #[test]
        fn first_encoded_is_identical() {
            let mut code = HuffmanCode {
                code: BitVec::new(),
            };
            code.append_bit(true);
            code.append_bit(true);
            assert!(code.code.eq_vec(&[true, true]));

            let mut bitvec = BitVec::new();
            code.append_code_to_bitvec(&mut bitvec);
            assert!(bitvec.eq_vec(&[true, true]));
        }

        #[test]
        fn encoding_behaves_predictably() {
            let mut code1 = HuffmanCode {
                code: BitVec::new(),
            };
            code1.append_bit(true);
            code1.append_bit(false);

            let mut code2 = HuffmanCode {
                code: BitVec::new(),
            };
            code2.append_bit(false);
            code2.append_bit(true);

            let mut bitvec = BitVec::new();
            code1.append_code_to_bitvec(&mut bitvec);
            code2.append_code_to_bitvec(&mut bitvec);
            assert!(bitvec.eq_vec(&[true, false, false, true]));
        }
    }

    mod huffman_encoder {
        use crate::huffman_tree::huffman_generator::HuffmanGenerator;

        use super::super::*;

        #[test]
        fn using_unknown_symbol_returns_offending_symbol() {
            let mut gen = HuffmanGenerator::new();
            gen.add_occurences(&String::from("A"), 2);
            gen.add_occurences(&String::from("B"), 2);

            let encoder = HuffmanEncoder::from_tree(&gen.into_huffman_tree().unwrap());
            let literal = ["C".to_string()];
            let mut stream = literal.iter();
            let result = encoder.encode(&mut stream);
            assert_eq!(Err(String::from("C")), result);
        }

        #[test]
        fn two_symbols_encoding_is_correct() {
            let mut gen = HuffmanGenerator::new();
            gen.add_occurences(&"A", 2);
            gen.add_occurences(&"B", 2);

            let encoder = HuffmanEncoder::from_tree(&gen.into_huffman_tree().unwrap());
            let literal = ["B", "A"];
            let mut stream = literal.iter();
            let result = encoder.encode(&mut stream);
            assert!(result.unwrap().eq_vec(&[false, true]));
        }

        #[test]
        fn three_symbols_encoding_is_correct() {
            let mut gen = HuffmanGenerator::new();
            gen.add_occurences(&"A", 10);
            gen.add_occurences(&"B", 2);
            gen.add_occurences(&"C", 2);

            let encoder = HuffmanEncoder::from_tree(&gen.into_huffman_tree().unwrap());
            let literal = [
                "A", "A", "B", "A", "A", "C", "C", "A", "A", "A", "A", "B", "A",
            ];
            let mut stream = literal.iter();
            let result = encoder.encode(&mut stream);
            assert_eq!(result.unwrap().to_bytes(), vec![0b110010, 0b10000011, 0b0]);
        }
    }
}
