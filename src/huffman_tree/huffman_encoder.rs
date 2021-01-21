use std::{collections::HashMap, hash::Hash};

use bit_vec::BitVec;

use super::huffman_element::HuffmanNode;
use crate::huffman_tree::huffman_generator::HuffmanGenerator;

/// The huffman encoder struct contains a Huffman encoding scheme that can then be used to encode various sequences
/// of the symbols. Usually, that huffman encoding scheme is generated on a per-sample basis so as to optimize the
/// compression for the particular sequence being compressed.
/// ## Examples
/// ```
/// # use crate::huffman::huffman_tree::huffman_encoder::HuffmanEncoder;
/// let literal = [
/// "B", "A", "A", "A", "A", "C"
/// ];
/// let encoder = HuffmanEncoder::from_symbols_iterator(&mut literal.iter()).unwrap();
/// let result = encoder.encode(&mut literal.iter());
/// assert_eq!( result.unwrap().to_bytes(), vec![0b11000010]);
/// ```
pub struct HuffmanEncoder<T: Eq + Hash + Clone + Ord> {
    symbols: HashMap<T, HuffmanCode>,
}

impl<T: Eq + Hash + Clone + Ord> HuffmanEncoder<T> {
    /// Generates a HuffmanEncoder tailor-made to encode the contents streamed by this iterator.
    /// It is expected that you then encode the exact same content for encoding afterwards by using 'HuffmanEncoder::encode'.
    /// This usually means being able to restart the iterator or to create an identical one thereafter.
    pub fn from_symbols_iterator(
        iterator: &mut dyn Iterator<Item = &T>,
    ) -> Result<HuffmanEncoder<T>, &'static str> {
        let mut huffman_generator = HuffmanGenerator::new();
        for symbol in iterator {
            huffman_generator.add_occurences(symbol, 1);
        }
        match huffman_generator.into_huffman_tree() {
            Some(tree) => Ok(HuffmanEncoder::from_tree(&tree)),
            None => Err("One or fewer symbols were provided"),
        }
    }

    pub fn from_tree(tree: &HuffmanNode<T>) -> HuffmanEncoder<T> {
        let mut map = HashMap::new();

        HuffmanEncoder::visit_tree(&tree, HuffmanCode::new(), &mut map);
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

#[derive(Clone)]
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

        #[test]
        fn four_symbols_encoding_is_correct() {
            let mut gen = HuffmanGenerator::new();
            gen.add_occurences(&"A", 9);
            gen.add_occurences(&"B", 5);
            gen.add_occurences(&"C", 2);
            gen.add_occurences(&"D", 2);

            let encoder = HuffmanEncoder::from_tree(&gen.into_huffman_tree().unwrap());
            let literal = [
                "A", "B", "B", "A", "C", "D", "A", "A", "B", "A", "A", "B", "A", "A", "B", "C",
                "D", "A",
            ];
            let mut stream = literal.iter();
            let result = encoder.encode(&mut stream);
            assert_eq!(
                result.unwrap().to_bytes(),
                vec![0b01010011, 0b11100010, 0b00100010, 0b11111000]
            );
        }

        #[test]
        fn encoder_from_iterator() {
            let literal = [
                "A", "B", "B", "A", "C", "D", "A", "A", "B", "A", "A", "B", "A", "A", "B", "C",
                "D", "A",
            ];
            let encoder = HuffmanEncoder::from_symbols_iterator(&mut literal.iter()).unwrap();
            let result = encoder.encode(&mut literal.iter());
            assert_eq!(
                result.unwrap().to_bytes(),
                vec![0b01010011, 0b11100010, 0b00100010, 0b11111000]
            );
        }
    }
}
