use rhuffman::huffman_tree::huffman_generator::HuffmanTree;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Compressed<T: Eq> {
    pub tree: HuffmanTree<T>,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    pub data_len: usize,
}
