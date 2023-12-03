mod day1;
mod day2;
mod day3;

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

    match day_nr {
        1 => {
            day1::main(part);
        }
        2 => {
            day2::main(part)
        }
        3 => {
            day3::main(part)
        }
        _ => {
            println!("Failed to found day nr.  {}", day_nr);
            exit(1)
        }
    }

    Ok(())
}

pub enum Part {
    FIRST,
    SECOND,
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FIRST => write!(f, "first"),
            Self::SECOND => write!(f, "second"),
        }
    }
}
