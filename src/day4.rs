use crate::{Part, PartResult};

pub fn main(part: Part) -> PartResult {
    let input = include_str!("../assets/day4-input");
    let lines: Vec<&str> = input.split('\n').collect();

    if let Part::FIRST = part {
        let mut points_sum = 0;

        for line in lines {
            let matching_count = get_matching_count(line);

            if matching_count > 0 {
                points_sum += u64::pow(2, (matching_count - 1).try_into().unwrap());
            }
        }

        return PartResult::FIRST(points_sum as isize);
    }

    let mut matching_count_per_line: Vec<u64> = vec![];

    for line in lines {
        matching_count_per_line.push(get_matching_count(line));
    }

    let res = get_won_scratchcards_count(&matching_count_per_line, 0, matching_count_per_line.len() as u64);
    PartResult::SECOND(res as isize - 1)
}

fn get_won_scratchcards_count(matches_per_line: &Vec<u64>, current_idx: u64, scratchcards_to_check: u64) -> u64 {
    let mut res = 1;

    for i in current_idx..(current_idx + scratchcards_to_check) {
        res += get_won_scratchcards_count(
            matches_per_line,
            i + 1,
            matches_per_line[i as usize],
        );
    }

    res
}

fn get_matching_count(line: &str) -> u64 {
    let split: Vec<&str> = line.split(':').collect();
    let line: &str = split[1];
    let split: Vec<&str> = line.split('|').collect();

    let winning_numbers: &str = split[0].trim();
    let winning_numbers = winning_numbers.replace("  ", " ");
    let winning_numbers: Vec<u32> = winning_numbers.split(' ').map(parse).collect();

    let my_numbers: &str = split[1].trim();
    let my_numbers = my_numbers.replace("  ", " ");
    let my_numbers = my_numbers.split(' ').map(parse);

    let mut winning_num_count = 0;

    for num in my_numbers {
        if winning_numbers.contains(&num) {
            winning_num_count += 1;
        }
    }

    winning_num_count
}

fn parse(s: &str) -> u32 {
    s.parse().unwrap()
}