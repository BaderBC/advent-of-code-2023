use std::cmp::Ordering;
use std::collections::HashMap;
use crate::{Part, PartResult};

pub fn main(part: Part) -> PartResult {
    let input = include_str!("../assets/day7-input");

    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut hands = vec![];

    for line in lines {
        let split = line.split(' ').collect::<Vec<&str>>();
        let label = split[0];
        let bid = split[1].parse::<u64>().unwrap();

        hands.push(Hand { label, bid });
    }

    hands.sort_by(|a, b| is_first_bigger(&part, a, b));

    let mut total_winnings = 0;

    for (i, hand) in hands.iter().enumerate() {
        total_winnings += (i as u64 + 1) * hand.bid
    }

    PartResult::FIRST(total_winnings as isize)
}

pub fn get_all_letters(part: &Part) -> Vec<char> {
    if let Part::FIRST = part {
        vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
    } else {
        vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J']
    }
}

fn is_first_bigger(part: &Part, hand1: &Hand, hand2: &Hand) -> Ordering {
    let mut hand1_importance: u64 = get_type_importance('J', hand1.label);
    let mut hand2_importance: u64 = get_type_importance('J', hand2.label);

    if let Part::SECOND = part {
        for letter in get_all_letters(part) {
            let importance = get_type_importance(letter, hand1.label);
            if importance > hand1_importance {
                hand1_importance = importance;
            }
        }
        for letter in get_all_letters(part) {
            let importance = get_type_importance(letter, hand2.label);
            if importance > hand2_importance {
                hand2_importance = importance;
            }
        }
    }

    if hand1_importance > hand2_importance {
        return Ordering::Greater;
    }
    if hand1_importance < hand2_importance {
        return Ordering::Less;
    }

    for (i, letter) in hand1.label.chars().enumerate() {
        let hand2_letter = hand2.label.chars().collect::<Vec<char>>()[i];

        if let Some(v) = is_first_more_valuable(part, letter, hand2_letter) {
            return if v {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
    }

    Ordering::Equal
}

fn is_first_more_valuable(part: &Part, char1: char, char2: char) -> Option<bool> {
    let all_letters = get_all_letters(part);

    let mut letters = all_letters.iter();
    let mut letters_clone = letters.clone();

    let pos1 = letters.position(|&e| e == char1);
    let pos2 = letters_clone.position(|&e| e == char2);
    if pos1 == pos2 {
        return None;
    }

    Some(pos1 > pos2)
}

fn get_type_importance(replace_j_into: char, s: &str) -> u64 {
    let s = s.replace('J', replace_j_into.to_string().as_str());
    let mut char_count: HashMap<char, u64> = HashMap::new();

    for character in s.chars() {
        let entry = char_count.entry(character).or_insert(0);
        *entry += 1;
    }

    let mut char_count = char_count.iter().collect::<Vec<(&char, &u64)>>();
    char_count.sort_by_key(|e| -(*e.1 as i64));

    if *char_count[0].1 == 5 {
        return 6;
    }

    if *char_count[0].1 == 4 {
        return 5;
    }

    if *char_count[0].1 == 3 && *char_count[1].1 == 2 {
        return 4;
    }

    if *char_count[0].1 == 3 {
        return 3;
    }

    if *char_count[0].1 == 2 && *char_count[1].1 == 2 {
        return 2;
    }

    if *char_count[0].1 == 2 {
        return 1;
    }

    0
}

struct Hand<'a> {
    label: &'a str,
    bid: u64,
}