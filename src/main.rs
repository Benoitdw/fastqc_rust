use clap::{Arg, Command};

fn main() {
    let matches = Command::new("fastqc_rust")
        .version("0.1.0")
        .author("Benoît de Witte <benoitdewitte28@gmail.com>")
        .about("Some fastq qc check. (/!\\ Project mainly to learn Rust.)")
        .arg(Arg::new("fastq_path").help("The input fastq to process."))
        .get_matches();
    fastqc::parse_fastq(matches.get_one::<String>("fastq_path").unwrap())
}
