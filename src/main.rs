use clap::Parser;
use fastqc;

#[derive(Parser)]
struct Config {
    fastq_path: String,
}

fn main() {
    let args = Config::parse();
    fastqc::parse_fastq(args.fastq_path)
}
