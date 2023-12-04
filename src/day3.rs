use std::collections::HashMap;
use crate::{Part, PartResult};

pub fn main(part: Part) -> PartResult {
    let plain_input = include_str!("../assets/day3-input");
    let lines: Vec<&str> = plain_input.split('\n').collect();

    let mut stars_locations: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
    let mut sum_result: u64 = 0;

    for (current_line_i, line) in lines.iter().enumerate() {
        let mut i = 0;
        loop {
            if i >= line.len() {
                break;
            }

            if let Some(c) = line.chars().nth(i) {
                if !c.is_ascii_digit() {
                    i += 1;
                    continue;
                }
            }
            let end_idx = get_number_end_idx(line, i);
            let near_special_char = is_near_special_char(&lines, current_line_i, i, end_idx);
            if near_special_char.0 {
                let number = &line[i..=end_idx];
                let number = number.parse::<u64>().unwrap();
                sum_result += number;

                if let Some(location) = near_special_char.1 {
                    let entry = stars_locations.entry(location).or_insert(vec![]);
                    entry.push(number);
                }

                i = end_idx + 1;
            } else {
                i += 1;
            }
        }
    }

    if let Part::FIRST = part {
        return PartResult::FIRST(sum_result as isize);
    }

    let mut gear_ratios_sum = 0;
    for (_, values) in stars_locations {
        let mut multiplied = 1;

        if values.len() > 1 {
            for val in values {
                multiplied *= val;
            }
        } else {
            continue
        }

        gear_ratios_sum += multiplied;
    }

    PartResult::SECOND(gear_ratios_sum as isize)
}

// 2nd arg is is '*' location
fn is_near_special_char(lines: &Vec<&str>, line_idx: usize, mut start_idx: usize, mut end_idx: usize)
                        -> (bool, Option<(usize, usize)>) {
    let start_line_idx = if line_idx == 0 {
        0
    } else {
        line_idx - 1
    };
    let end_line_idx = if line_idx + 1 == lines.len() {
        line_idx
    } else {
        line_idx + 1
    };

    for current_line_i in start_line_idx..=end_line_idx {
        if (start_idx as isize) - 1 == -1 {
            start_idx = 1;
        }
        if end_idx + 1 > lines[current_line_i].len() {
            end_idx -= 1;
        }

        let line = lines[current_line_i];
        for i in start_idx - 1..=end_idx + 1 {
            if let Some(c) = line.chars().nth(i) {
                let special_char = is_special_char(c);
                if special_char.0 && special_char.1 {
                    let start_location = Some((current_line_i, i));
                    return (true, start_location);
                }
                if special_char.0 {
                    return (true, None);
                }
            }
        }
    }

    (false, None)
}

fn get_number_end_idx(line: &str, start_index: usize) -> usize {
    for i in start_index..line.len() {
        if !line.chars().nth(i).unwrap().is_ascii_digit() {
            return i - 1;
        }
    }

    line.len() - 1
}

// 2nd arg is whether char is '*'
fn is_special_char(character: char) -> (bool, bool) {
    if character.is_ascii_digit() {
        return (false, false);
    }
    if character == '.' {
        return (false, false);
    }
    if character == '*' {
        return (true, true);
    }
    (true, false)
}