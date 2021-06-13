use super::huffman_node::HuffmanNode;

use bit_vec::BitVec;

pub struct HuffmanDecoder<T: PartialEq + Eq> {
    root: HuffmanNode<T>,
}

impl<T: PartialEq + Eq + Clone> HuffmanDecoder<T> {
    pub fn new(tree: HuffmanNode<T>) -> HuffmanDecoder<T> {
        HuffmanDecoder { root: tree }
    }

    pub fn decode_unbounded(&self, buffer: &BitVec) -> Vec<T> {
        let mut pos = 0;
        let mut result = vec![];
        while pos < buffer.len() {
            result.push(HuffmanDecoder::decode_single_symbol(
                buffer, &self.root, &mut pos,
            ))
        }
        result
    }

    fn decode_single_symbol(buffer: &BitVec, root: &HuffmanNode<T>, pos: &mut usize) -> T {
        match root {
            HuffmanNode::Branch(branch) => {
                let bit_value = buffer[*pos];
                let next_node = if bit_value {
                    &branch.links.1
                } else {
                    &branch.links.0
                };
                *pos += 1;
                HuffmanDecoder::decode_single_symbol(buffer, next_node, pos)
            }
            HuffmanNode::Leaf(leaf) => leaf.symbol.clone(),
        }
    }

    pub fn get_tree(&self) -> &HuffmanNode<T> {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::super::huffman_generator::*;
    use std::fmt::Debug;
    use std::hash::Hash;
    fn encode_decode<T: Eq + Clone + Hash + Ord + Debug>(
        gen_codes: &mut dyn Iterator<Item = &T>,
        encode: &mut dyn Iterator<Item = &T>,
    ) -> Vec<T> {
        let mut gen = HuffmanGenerator::new();
        gen.add_occurences_from_iterator(gen_codes);
        let (encoder, decoder) = gen.into_encoder_decoder_pair().unwrap();
        let result = encoder.encode(encode).unwrap();
        decoder.decode_unbounded(&result)
    }

    #[test]
    fn two_symbols_decoding_is_correct1() {
        let literal = ["B", "A"];
        let decoded = encode_decode(&mut literal.iter(), &mut literal.iter());
        assert_eq!(decoded, literal);
    }

    #[test]
    fn two_symbols_decoding_is_correct2() {
        let literal = ["B", "A", "B", "B", "B", "B", "A"];
        let decoded = encode_decode(&mut literal.iter(), &mut literal.iter());
        assert_eq!(decoded, literal);
    }

    #[test]
    fn two_symbols_decoding_is_correct3() {
        let literal = ["B", "A", "B", "B", "B", "B", "A", "B"];
        let decoded = encode_decode(&mut literal.iter(), &mut literal.iter());
        assert_eq!(decoded, literal);
    }

    #[test]
    fn three_symbols_decoding_is_correct3() {
        let literal = ["B", "A", "B", "B", "B", "B", "C", "B", "C", "C", "C"];
        let decoded = encode_decode(&mut literal.iter(), &mut literal.iter());
        assert_eq!(decoded, literal);
    }

    #[test]
    fn english_symbols_decoding_is_correct() {
        let literal = String::from("Hello there! General Kenobi!!?");
        let literal: Vec<char> = literal.chars().collect();
        let decoded = encode_decode(&mut literal.iter(), &mut literal.iter());
        assert_eq!(decoded, literal);
    }
}
