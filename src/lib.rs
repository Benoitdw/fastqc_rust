use fastq::{parse_path, Record};

pub fn parse_fastq(fastq_path: String) {
    let result = parse_path(Some(fastq_path), |parser| {
        parser.each(|record| {
            let record_as_str: String = record.seq().iter().map(|&byte| char::from(byte)).collect();
            println!("{:?}", record_as_str);
            true
        })
    });
    match result {
        Ok(_result) => println!("Fastq correctly handled."),
        Err(error) => println!("There was an error handling fastq. {:?}", error),
    };
}