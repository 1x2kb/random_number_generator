pub use clap::Parser;
use rand::Rng;

pub trait GenerateNumbers {
    fn generate_numbers(num: usize, min: u64, max: u64) -> Vec<u64>;
}

pub trait CheckForErrors {
    fn check_for_errors(num: usize, min: u64, max: u64) -> Result<(), NumberError>;
}

pub trait Execute {
    fn execute(&self) -> Result<Vec<u64>, NumberError>;
}

#[derive(Debug, Parser, PartialEq, Eq)]
#[clap(author, version, about, long_about = None)]
pub struct RngGenerator {
    #[clap(short, long, default_value_t = 0)]
    pub min: u64,
    #[clap(short('x'), long, default_value_t = 100)]
    pub max: u64,
    #[clap(default_value_t = 1)]
    pub numbers: usize,
}

impl GenerateNumbers for RngGenerator {
    fn generate_numbers(num: usize, min: u64, max: u64) -> Vec<u64> {
        let mut rng = rand::thread_rng();
        (0..num).map(|_| rng.gen_range(min..=max)).collect()
    }
}

impl CheckForErrors for RngGenerator {
    fn check_for_errors(num: usize, min: u64, max: u64) -> Result<(), NumberError> {
        if min > max {
            return Err(NumberError::MinGreaterThanMax);
        }
        if num == 0 {
            return Err(NumberError::NumbersNotValid);
        }

        Ok(())
    }
}

impl Execute for RngGenerator
where
    RngGenerator: GenerateNumbers + CheckForErrors,
{
    fn execute(&self) -> Result<Vec<u64>, NumberError> {
        let (numbers, min, max) = (self.numbers, self.min, self.max);
        RngGenerator::check_for_errors(numbers, min, max)?;
        Ok(RngGenerator::generate_numbers(numbers, min, max))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberError {
    MinGreaterThanMax,
    NumbersNotValid,
}

#[cfg(test)]
mod generate_numbers_should {
    use crate::rng_generator::GenerateNumbers;

    use super::*;

    #[test]
    fn generate_exactly_n_numbers() {
        let numbers = 10;

        let (min, max) = (0, 100);
        let generated = RngGenerator::generate_numbers(numbers, min, max);
        assert_eq!(generated.len(), numbers);
    }

    #[test]
    fn generate_between_min_and_max() {
        let numbers = 10;

        let (min, max) = (10, 15);
        let generated = RngGenerator::generate_numbers(numbers, min, max);
        assert!(generated.into_iter().all(|num| num >= min && num <= max),);
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
            RngGenerator::check_for_errors(10, min, max),
            Err(NumberError::MinGreaterThanMax)
        );
    }

    #[test]
    fn return_error_if_numbers_is_zero() {
        let min = 0;
        let max = 100;

        assert_eq!(
            RngGenerator::check_for_errors(0, min, max),
            Err(NumberError::NumbersNotValid)
        );
    }
}

#[cfg(test)]
mod execute_should {
    use super::*;

    #[test]
    fn return_generated_numbers() {
        let min = 0;
        let max = 100;

        let numbers = 100;
        let rng_generator = RngGenerator { min, max, numbers };
        let generated = rng_generator.execute().unwrap();

        assert_eq!(generated.len(), numbers);
        assert!(generated.into_iter().all(|num| num >= min && num <= max));
    }

    #[test]
    fn return_error_if_min_greater_than_max() {
        let min = 10;
        let max = 5;

        let numbers = 100;

        let rng_generator = RngGenerator { min, max, numbers };

        assert_eq!(rng_generator.execute(), Err(NumberError::MinGreaterThanMax));
    }

    #[test]
    fn return_error_if_numbers_is_zero() {
        let min = 0;
        let max = 100;

        let numbers = 0;

        let rng_generator = RngGenerator { min, max, numbers };

        assert_eq!(rng_generator.execute(), Err(NumberError::NumbersNotValid));
    }
}

#[cfg(test)]
mod rng_generator_should {
    use super::*;

    #[test]
    fn respect_defaults() {
        let expected_rng = RngGenerator {
            min: 0,
            max: 100,
            numbers: 1,
        };
        let rng: RngGenerator = RngGenerator::parse_from(["rns"]);

        assert_eq!(rng, expected_rng);

        let generated = rng.execute().unwrap();
        assert_eq!(generated.len(), rng.numbers);
        assert!(generated
            .into_iter()
            .all(|num| num >= rng.min && num <= rng.max));
    }

    #[test]
    fn parse_custom_inputs() {
        let expected_rng = RngGenerator {
            min: 1,
            max: 4,
            numbers: 5,
        };
        let rng = RngGenerator::parse_from(["rns", "-m=1", "-x=4", "5"]);

        assert_eq!(rng, expected_rng);

        let generated = rng.execute().unwrap();
        assert_eq!(generated.len(), rng.numbers);
        assert!(generated
            .into_iter()
            .all(|num| num >= rng.min && num <= rng.max));
    }

    #[test]
    fn error_min_greater_than_max() {
        let rng = RngGenerator::parse_from(["rns", "-m=4", "-x=1", "5"]);

        assert_eq!(rng.execute(), Err(NumberError::MinGreaterThanMax));
    }

    #[test]
    fn error_numbers_not_valid() {
        let rng = RngGenerator::parse_from(["rns", "-m=1", "-x=1", "0"]);

        assert_eq!(rng.execute(), Err(NumberError::NumbersNotValid));
    }
}
