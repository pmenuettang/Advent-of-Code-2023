use std::error::Error;

use advent_of_code_2023::day1;

fn main() {
    match run()     {
        Ok(_) => println!("Done."),
        Err(e) => println!("{e}"),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("Day 1 part 1 : total from input is {}.", day1::day1_step1()?);
    println!("Day 1 part 2 : total from input is {}.", day1::day1_step2()?);

    Ok(())
}
