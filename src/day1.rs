use std::collections::HashMap;
use crate::{Part, PartResult};

pub fn main(part: Part) -> PartResult {
    let input = include_str!("../assets/day1-input");

    let word_digit_map = get_word_num_map();
    let mut results_sum: u32 = 0;

    for line in input.split('\n') {
        let reverted_line = line.chars().rev().collect::<String>();

        let first_num = find_first_num(line, false, &word_digit_map, &part);
        let second_num = find_first_num(&reverted_line, true, &word_digit_map, &part);

        results_sum += (first_num * 10 + second_num) as u32;
    }
    
    match part {
        Part::FIRST => PartResult::FIRST(results_sum as isize),
        Part::SECOND => PartResult::SECOND(results_sum as isize)
    }
}

fn find_first_num<'a>(line: &str, is_reversed: bool, word_digit_map: &HashMap<&'a str, u8>, part: &Part) -> u8 {
    let mut currently_checked = String::new();

    for char in line.chars() {
        if let Some(digit) = char.to_digit(10) {
            return digit as u8;
        }
        if let Part::SECOND = part {
            currently_checked += &char.to_string();
            if let Some(num) = find_in_map(currently_checked.clone(), is_reversed, word_digit_map) {
                return num
            }
        }
    }

    unreachable!()
}

fn find_in_map(mut text: String, is_reversed: bool, map: &HashMap<&str, u8>) -> Option<u8> {
    if is_reversed {
        text = text.chars().rev().collect::<String>();
    }

    for (key, value) in map {
        if text.contains(key) {
            return Some(*value);
        }
    }

    None
}

fn get_word_num_map<'a>() -> HashMap<&'a str, u8> {
    let mut map = HashMap::new();

    map.insert("zero", 0);
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    map
}