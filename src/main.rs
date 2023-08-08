use clap::Parser;
use conway_sequence::generators::naive::NaiveGenerator;
use conway_sequence::utils::sequence::Sequence;
use std::fs::File;
use std::io::prelude::*;
/// Generate and save conway sequence
#[derive(Parser, Debug)]
struct Args {
    /// Seed to start the conway sequence
    #[arg(short, long, default_value_t = String::from("1"))]
    seed: String,

    /// Number of the term in the sequence at which to stop
    #[arg(short, long, default_value_t = 10)]
    iterations: usize,

    #[arg(short, long, default_value_t = String::from("smart"))]
    generator_type: String,
}

fn naive_generation(seed: String, iterations: usize, mut file: File) -> std::io::Result<File> {
    let generator = NaiveGenerator::new();
    let mut n_term: Vec<char> = seed.chars().collect();

    for _ in 0..iterations {
        let n1_term = generator.next_sequence(&n_term);
        file = generator.write_sequence_to_file(n_term, file).unwrap();
        n_term = n1_term;
    }

    generator.write_sequence_to_file(n_term, file)
}

fn smart_generation(seed: String, iterations: usize, mut file: File) -> std::io::Result<File> {
    let mut s = Sequence::new(seed);
    for i in 0..iterations {
        s = s.next();
        file.write_all(format!("{}\n", s).as_bytes())?;
        println!("{i}");
    }
    Ok(file)
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let seed = args.seed;
    let iterations = args.iterations;
    let generator_type = args.generator_type;
    let file = File::create(format!("seed_{seed}_iterations_{iterations}.txt"))?;

    if generator_type == "naive" {
        naive_generation(seed, iterations, file)?;
    } else if generator_type == "smart" {
        smart_generation(seed, iterations, file)?;
    } else {
        println!("Generator type not recognized, expected : naive or smart got {generator_type}")
    };

    Ok(())
}
