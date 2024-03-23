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
    #[clap(default_value_t = 1)]
    numbers: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum NumberError {
    MinGreaterThanMax,
    NumbersNotValid,
}

fn main() {
    let args = Args::parse();
}

fn check_for_errors(num: usize, min: u64, max: u64) -> Result<(), NumberError> {
    if min > max {
        return Err(NumberError::MinGreaterThanMax);
    }
    if num == 0 {
        return Err(NumberError::NumbersNotValid);
    }
    Ok(())
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

#[cfg(test)]
mod check_for_errors_should {
    use super::*;

    #[test]
    fn return_error_if_min_greater_than_max() {
        let min = 10;
        let max = 5;

        assert_eq!(
            check_for_errors(10, min, max),
            Err(NumberError::MinGreaterThanMax)
        );
    }

    #[test]
    fn return_error_if_numbers_is_zero() {
        let min = 0;
        let max = 100;

        assert_eq!(
            check_for_errors(0, min, max),
            Err(NumberError::NumbersNotValid)
        );
    }
}
