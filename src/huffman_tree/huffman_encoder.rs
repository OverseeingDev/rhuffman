use std::{collections::HashMap, hash::Hash};

use super::huffman_element::HuffmanNode;
pub struct HuffmanEncoder<T: Eq + Hash> {
    symbols: HashMap<T, HuffmanCode>
}

#[derive(Copy, Clone)]
struct HuffmanCode {
    code: usize,
    bit_length: usize
}

impl HuffmanCode {
    fn append_bit(&mut self, bit: bool){
        self.code = (self.code << 1) + if bit { 1 } else { 0 };
        self.bit_length += 1;
    }
}

#[test]
fn appendbit_modifies_code_correctly() {
    let mut code = HuffmanCode {code: 0, bit_length: 0};

    code.append_bit(true);

    assert_eq!(1, code.bit_length);
    assert_eq!(1, code.code);
}

#[test]
fn multiple_invocations_of_appendbit_result_in_correct_bitlength() {
    let mut code = HuffmanCode {code: 0, bit_length: 0};
    code.append_bit(false);
    code.append_bit(false);
    
    assert_eq!(2, code.bit_length);
}

#[test]
fn multiple_invocations_of_appendbit_result_in_correct_code() {
    let mut code = HuffmanCode {code: 0, bit_length: 0};
    code.append_bit(false);
    code.append_bit(true);
    
    assert_eq!(1, code.code);

    let mut code = HuffmanCode {code: 0, bit_length: 0};
    code.append_bit(false);
    code.append_bit(false);
    assert_eq!(0, code.code);

    let mut code = HuffmanCode {code: 0, bit_length: 0};
    code.append_bit(true);
    code.append_bit(true);
    assert_eq!(3, code.code);

    let mut code = HuffmanCode {code: 0, bit_length: 0};
    code.append_bit(true);
    code.append_bit(false);
    code.append_bit(false);
    code.append_bit(true);
    assert_eq!(9, code.code);
}