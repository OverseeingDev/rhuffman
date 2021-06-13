mod compressed;

use bit_vec::BitVec;
use rhuffman::huffman_tree::huffman_decoder::HuffmanDecoder;
use rhuffman::huffman_tree::huffman_generator::HuffmanGenerator;
use std::io::prelude::*;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

use compressed::Compressed;

#[derive(Debug, StructOpt)]
#[structopt(about = "Size shall be brought to entropy", author)]
struct Opt {
    /// Compress input file into output file
    #[structopt(short = "c", long = "compress", required_unless = "decompress")]
    compress: bool,
    /// Decompress input file into output file
    #[structopt(short = "d", long = "decompress", required_unless = "compress")]
    decompress: bool,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    //println!("{:?}", opt);

    let mut file = File::open(opt.input).expect("Unable to open the file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Unable to read the file");

    let mut out = File::create(opt.output).expect("could not create output file");

    if opt.compress {
        // Basic byte-wise Huffman compression
        let mut gen = HuffmanGenerator::new();
        gen.add_occurences_from_iterator(&mut contents.iter());
        let (encoder, _decoder) = gen.into_encoder_decoder_pair().unwrap();
        let compressed = encoder.encode(&mut contents.iter()).unwrap();

        let data = Compressed {
            tree: _decoder.get_tree().clone(),
            data: compressed.to_bytes(),
            data_len: compressed.len(),
        };

        let compressed = rmp_serde::to_vec(&data).unwrap();
        out.write_all(&compressed).unwrap();
    } else if opt.decompress {
        // Basic byte-wise Huffman compression

        let data: Compressed<u8> = rmp_serde::from_slice(contents.as_slice()).unwrap();
        let mut bitvec = BitVec::from_bytes(data.data.as_slice());

        // Restore bit length from bytes
        unsafe {
            bitvec.set_len(data.data_len);
        }
        let decoder = HuffmanDecoder::new(data.tree);
        out.write_all(&decoder.decode_unbounded(&bitvec)).unwrap();
    } else {
        panic!("Neither compress or decompress was set. This is a bug in rhuff-compress")
    }
}
