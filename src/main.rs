mod rng_generator;

use rng_generator::{Execute, NumberError, Parser, RngGenerator};

fn main() {
    let rng_generator = RngGenerator::parse();

    let numbers = match rng_generator.execute() {
        Ok(numbers) => numbers,
        Err(error) => match error {
            NumberError::MinGreaterThanMax => panic!(
                "Min ({}) cannot be greater than Max ({})",
                rng_generator.min, rng_generator.max
            ),
            NumberError::NumbersNotValid => {
                panic!("Numbers ({}) cannot be 0", rng_generator.numbers)
            }
        },
    };

    print!("{}", itertools::join(numbers.into_iter(), ","));
}
