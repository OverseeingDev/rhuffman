use std::{collections::HashMap, hash::Hash};

use bit_vec::BitVec;

use super::huffman_element::HuffmanNode;
pub struct HuffmanEncoder<T: Eq + Hash> {
    symbols: HashMap<T, HuffmanCode>
}

#[derive(Clone)]
struct HuffmanCode {
    code: BitVec
}

impl HuffmanCode {
    fn append_bit(&mut self, bit: bool){
        self.code.push(bit);
    }

    pub fn append_code_to_bitvec(&self, bitvec: &mut BitVec){
        bitvec.append(&mut self.code.clone());
    }
}

#[test]
fn appendbit_modifies_code_correctly() {
    let mut code = HuffmanCode {code: BitVec::new()};

    code.append_bit(true);

    assert!(code.code.eq_vec(&[true]));
}

#[test]
fn multiple_invocations_of_appendbit_result_in_correct_code() {
    let mut code = HuffmanCode {code: BitVec::new()};
    code.append_bit(false);
    code.append_bit(true);
    
    assert!(code.code.eq_vec(&[false, true]));

    let mut code = HuffmanCode {code: BitVec::new()};
    code.append_bit(false);
    code.append_bit(false);
    assert!(code.code.eq_vec(&[false, false]));

    let mut code = HuffmanCode {code: BitVec::new()};
    code.append_bit(true);
    code.append_bit(true);
    assert!(code.code.eq_vec(&[true, true]));

    let mut code = HuffmanCode {code: BitVec::new()};
    code.append_bit(true);
    code.append_bit(false);
    code.append_bit(true);
    code.append_bit(true);
    assert!(code.code.eq_vec(&[true, false, true, true]));
}

#[test]
fn first_encoded_is_identical() {
    let mut code = HuffmanCode {code: BitVec::new()};
    code.append_bit(true);
    code.append_bit(true);
    assert!(code.code.eq_vec(&[true, true]));

    let mut bitvec = BitVec::new();
    code.append_code_to_bitvec(&mut bitvec);
    assert!(bitvec.eq_vec(&[true, true]));
}

#[test]
fn encoding_behaves_predictably() {
    let mut code1 = HuffmanCode {code: BitVec::new()};
    code1.append_bit(true);
    code1.append_bit(false);

    let mut code2 = HuffmanCode {code: BitVec::new()};
    code2.append_bit(false);
    code2.append_bit(true);

    let mut bitvec = BitVec::new();
    code1.append_code_to_bitvec(&mut bitvec);
    code2.append_code_to_bitvec(&mut bitvec);
    assert!(bitvec.eq_vec(&[true, false, false, true]));
}