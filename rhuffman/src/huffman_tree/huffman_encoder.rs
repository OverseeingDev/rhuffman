use std::{collections::HashMap, hash::Hash};

use bit_vec::BitVec;

use super::huffman_generator::HuffmanGenerator;
use super::huffman_node::HuffmanNode;

/// The huffman encoder struct contains a Huffman encoding scheme that can then be used to encode various sequences
/// of the symbols. Usually, that huffman encoding scheme is generated on a per-sample basis so as to optimize the
/// compression for the particular sequence being compressed.
/// ## Examples
/// ```
/// # use crate::rhuffman::huffman_tree::huffman_encoder::HuffmanEncoder;
/// let literal = [
/// "B", "A", "A", "A", "A", "C"
/// ];
/// let encoder = HuffmanEncoder::from_symbols_iterator(&mut literal.iter()).unwrap();
/// let result = encoder.encode(&mut literal.iter());
/// assert_eq!( result.unwrap().to_bytes(), vec![0b11000010]);
/// ```
pub struct HuffmanEncoder<T: Eq + Hash + Clone + Ord> {
    symbols: HashMap<T, BitVec>,
}

impl<T: Eq + Hash + Clone + Ord> HuffmanEncoder<T> {
    /// Generates a HuffmanEncoder tailor-made to encode the contents streamed by this iterator.
    /// It is expected that you then encode the exact same content for encoding afterwards by using [encode()](HuffmanEncoder::encode)
    /// on the resulting Encoder.
    ///
    /// This usually means being able to restart the iterator or to create an identical one thereafter.
    pub fn from_symbols_iterator(
        iterator: &mut dyn Iterator<Item = &T>,
    ) -> Result<HuffmanEncoder<T>, &'static str> {
        let mut huffman_generator = HuffmanGenerator::new();
        huffman_generator.add_occurences_from_iterator(iterator);
        match huffman_generator.into_huffman_tree() {
            Some(tree) => Ok(HuffmanEncoder::from_tree(&tree)),
            None => Err("One or fewer symbols were provided"),
        }
    }

    /// Generates a [HuffmanEncoder](HuffmanEncoder) from the tree. You may obtain
    /// such a tree from a [HuffmanGenerator](super::huffman_generator::HuffmanGenerator)
    pub fn from_tree(tree: &HuffmanNode<T>) -> HuffmanEncoder<T> {
        let mut map = HashMap::new();

        HuffmanEncoder::visit_tree(&tree, BitVec::new(), &mut map);
        HuffmanEncoder { symbols: map }
    }

    fn visit_tree(
        tree: &HuffmanNode<T>,
        mut current_prefix: BitVec,
        symbols: &mut HashMap<T, BitVec>,
    ) {
        match tree {
            HuffmanNode::Leaf(leaf) => {
                symbols.insert(leaf.symbol.clone(), current_prefix);
            }
            HuffmanNode::Branch(branch) => {
                let mut left_prefix = current_prefix.clone();
                left_prefix.push(false);
                current_prefix.push(true);
                HuffmanEncoder::visit_tree(&branch.links.0, left_prefix, symbols);
                HuffmanEncoder::visit_tree(&branch.links.1, current_prefix, symbols);
            }
        }
    }

    /// Attempts to encode the given stream of symbols with the internal encoding.
    /// ## Errors
    /// If the stream produces a symbol that is not part of the encoding, encode returns Err containing a copy of the offending symbol.
    pub fn encode(&self, iter: &mut dyn Iterator<Item = &T>) -> Result<BitVec, T> {
        let mut bitvec = BitVec::new();
        for symbol in iter {
            if let Some(code) = self.symbols.get(&symbol) {
                bitvec.append(&mut code.clone());
            } else {
                return Err(symbol.clone());
            }
        }
        Ok(bitvec)
    }
}

#[cfg(test)]
mod tests {
    use crate::huffman_tree::huffman_generator::HuffmanGenerator;

    use super::*;

    #[test]
    fn using_unknown_symbol_returns_offending_symbol() {
        let mut gen = HuffmanGenerator::new();
        gen.add_occurences_to_symbol(&String::from("A"), 2);
        gen.add_occurences_to_symbol(&String::from("B"), 2);

        let encoder = HuffmanEncoder::from_tree(&gen.into_huffman_tree().unwrap());
        let literal = ["C".to_string()];
        let mut stream = literal.iter();
        let result = encoder.encode(&mut stream);
        assert_eq!(Err(String::from("C")), result);
    }

    #[test]
    fn two_symbols_encoding_is_correct() {
        let mut gen = HuffmanGenerator::new();
        gen.add_occurences_to_symbol(&"A", 2);
        gen.add_occurences_to_symbol(&"B", 2);

        let encoder = HuffmanEncoder::from_tree(&gen.into_huffman_tree().unwrap());
        let literal = ["B", "A"];
        let mut stream = literal.iter();
        let result = encoder.encode(&mut stream);
        assert!(result.unwrap().eq_vec(&[false, true]));
    }

    #[test]
    fn three_symbols_encoding_is_correct() {
        let mut gen = HuffmanGenerator::new();
        gen.add_occurences_to_symbol(&"A", 10);
        gen.add_occurences_to_symbol(&"B", 2);
        gen.add_occurences_to_symbol(&"C", 2);

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
        gen.add_occurences_to_symbol(&"A", 9);
        gen.add_occurences_to_symbol(&"B", 5);
        gen.add_occurences_to_symbol(&"C", 2);
        gen.add_occurences_to_symbol(&"D", 2);

        let encoder = HuffmanEncoder::from_tree(&gen.into_huffman_tree().unwrap());
        let literal = [
            "A", "B", "B", "A", "C", "D", "A", "A", "B", "A", "A", "B", "A", "A", "B", "C", "D",
            "A",
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
            "A", "B", "B", "A", "C", "D", "A", "A", "B", "A", "A", "B", "A", "A", "B", "C", "D",
            "A",
        ];
        let encoder = HuffmanEncoder::from_symbols_iterator(&mut literal.iter()).unwrap();
        let result = encoder.encode(&mut literal.iter());
        assert_eq!(
            result.unwrap().to_bytes(),
            vec![0b01010011, 0b11100010, 0b00100010, 0b11111000]
        );
    }
}
