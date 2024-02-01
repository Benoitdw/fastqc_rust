use clap::Parser;
use fastq::{parse_path, Record};

#[derive(Parser)]
struct Cli {
    fastq_path : String,
}

fn main() {   
    let args = Cli::parse();  
    
    let result = parse_path(Some(args.fastq_path), |parser| {
        parser.each(|record| {
            let record_as_str : String = record.seq().iter().map(|&byte| char::from(byte)).collect();
            println!("{:?}",record_as_str);
            true
        })
    });
    match result{
        Ok(_result) => println!("Fastq correctly handled."),
        Err(error) => println!("There was an error handling fastq. {:?}", error)
    };
}
