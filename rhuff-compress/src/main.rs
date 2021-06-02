use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Size shall be brought to entropy", author)]
struct Opt {
    /// Compress input file into output file
    #[structopt(short = "c", long = "compress", conflicts_with("decompress"))]
    compress: bool,
    /// Decompress input file into output file
    #[structopt(short = "d", long = "decompress", conflicts_with("compress"))]
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
    println!("{:?}", opt);
}
