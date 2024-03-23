use std::io::{stdout, Write};

use clap::Parser;
use rand::Rng;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 0)]
    min: u64,
    #[clap(short('x'), long, default_value_t = 100)]
    max: u64,
    numbers: usize,
}

fn main() {
    let args = Args::parse();
}

fn generate_numbers(num: usize, min: u64, max: u64) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    (0..num)
        .into_iter()
        .map(|_| rng.gen_range(min..=max))
        .collect()
}

#[cfg(test)]
mod generate_numbers_should {
    use super::*;

    #[test]
    fn generate_exactly_n_numbers() {
        let numbers = 10;

        let (min, max) = (0, 100);
        let generated = generate_numbers(numbers, min, max);
        assert_eq!(generated.len(), numbers);
    }

    #[test]
    fn generate_between_min_and_max() {
        let numbers = 10;

        let (min, max) = (10, 15);
        let generated = generate_numbers(numbers, min, max);
        assert!(
            generated.into_iter().all(|num| num >= min && num <= max),
            "Number generated was not between {} and {}",
            min,
            max
        );
    }
}
