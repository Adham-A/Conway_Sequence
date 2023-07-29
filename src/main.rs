use std::fs::File;
use std::io::prelude::*;

use clap::Parser;

/// Generate and save conway sequence
#[derive(Parser, Debug)]
struct Args {
    /// Seed to start the conway sequence
    #[arg(short, long, default_value_t = String::from("1"))]
    seed: String,

    /// Number of the term in the sequence at which to stop
    #[arg(short, long, default_value_t = 10)]
    iterations: u128,
}

fn next_sequence(input: &[char]) -> Vec<char> {
    let mut sequence = Vec::new();
    let mut count = 0;
    let mut digit = input[0];

    for current in input.iter() {
        if *current == digit {
            count += 1;
        } else {
            sequence.push(char::from_digit(count, 10).unwrap());
            sequence.push(digit);
            digit = *current;
            count = 1;
        }
    }
    sequence.push(char::from_digit(count, 10).unwrap());
    sequence.push(digit);

    sequence
}

fn write_sequence_to_file(input: Vec<char>, mut file: File) -> std::io::Result<File> {
    let row_str: String = input.iter().collect();
    file.write_all(row_str.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(file)
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let seed = args.seed;
    let iterations = args.iterations;

    let mut file = File::create(format!("seed_{seed}_iterations_{iterations}.txt"))?;

    let mut n_term: Vec<char> = seed.chars().collect();

    for i in 0..iterations {
        let n1_term = next_sequence(&n_term);
        file = write_sequence_to_file(n_term, file).unwrap();
        n_term = n1_term;
        println!("Iteration {i}");
    }

    write_sequence_to_file(n_term, file)?;
    Ok(())
}
