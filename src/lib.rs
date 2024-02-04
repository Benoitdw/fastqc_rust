use std::{result, vec};

use fastq::{parse_path, Record};

struct Resume {
    raw_quality_count: Vec<Vec<u8>>,
    base_average_quality: Vec<f32>,
}

impl Resume {
    fn new() -> Self {
        Self {
            raw_quality_count: Vec::new(),
            base_average_quality: Vec::new(),
        }
    }

    fn compute_stats(&mut self) {
        self.raw_quality_count
            .iter()
            .for_each(|base| self.base_average_quality.push(average(base)))
    }
}

pub fn parse_fastq(fastq_path: String) {
    let mut resume = Resume::new();
    let result = parse_path(Some(fastq_path), |parser| {
        parser.each(|record| {
            record.qual().iter().enumerate().for_each(|(id, &qual)| {
                match resume.raw_quality_count.get_mut(id) {
                    Some(vector) => vector.push(qual),
                    None => resume.raw_quality_count.push(vec![qual]),
                }
                resume.raw_quality_count[id].push(qual)
            });
            true
        })
    });
    match result {
        Ok(_result) => println!("Fastq correctly handled."),
        Err(error) => println!("There was an error handling fastq. {:?}", error),
    };
    resume.compute_stats();
    println!(
        "Average quality:  {:?}",
        resume
            .base_average_quality
            .iter()
            .map(|v| { *v as u32 })
            .sum::<u32>() as f32
            / resume.base_average_quality.len() as f32
    )
}

// I want this function to be able to take any type of int in vector
fn average(vector: &Vec<u8>) -> f32 {
    vector.iter().map(|v| *v as u32).sum::<u32>() as f32 / vector.len() as f32
}
