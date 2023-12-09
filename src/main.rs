mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;
use std::process::exit;

fn main() -> anyhow::Result<()> {
    let mut day_input = String::new();
    let mut part_input = String::new();

    print!("Choose day number: ");
    io::stdout().flush()?;

    io::stdin().read_line(&mut day_input).expect("Failed to read input");
    let day_nr = day_input.trim().parse::<u8>().expect("Failed to parse your input");

    print!("First or second part (1|2): ");
    io::stdout().flush()?;

    io::stdin().read_line(&mut part_input).expect("Failed to read input");
    let part = part_input.trim().parse::<u8>().expect("Failed to parse your input");
    let part = match part {
        1 => Part::FIRST,
        2 => Part::SECOND,
        _ => {
            println!("Input must be 1 or 2");
            exit(1);
        }
    };

    let result = match day_nr {
        1 => {
            day1::main(part)
        }
        2 => {
            day2::main(part)
        }
        3 => {
            day3::main(part)
        }
        4 => {
            day4::main(part)
        }
        5 => {
            day5::main(part)
        }
        6 => {
            day6::main(part)
        }
        7 => {
            day7::main(part)
        }
        8 => {
            day8::main(part)
        }
        _ => {
            println!("Failed to found day nr.  {}", day_nr);
            exit(1)
        }
    };

    println!("Day {} {}", day_nr, result);

    Ok(())
}

pub enum Part {
    FIRST,
    SECOND,
}

pub enum PartResult {
    FIRST(isize),
    SECOND(isize),
}

impl Display for PartResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FIRST(v) => write!(f, "first part result is: {}", v),
            Self::SECOND(v) => write!(f, "second part result is: {}", v),
        }
    }
}
