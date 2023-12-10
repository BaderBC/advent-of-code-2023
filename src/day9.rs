use crate::{Part, PartResult};

pub fn main(part: Part) -> PartResult {
    let input = include_str!("../assets/day9-input");
    let lines = input.split('\n');
    
    let lines = lines.map(|e| {
        e.split(' ').map(parse).collect::<Vec<i32>>()
    });
    let lines = lines.collect::<Vec<Vec<i32>>>();
    let mut sum = 0;

    for line in lines {
        match part {
            Part::FIRST => sum += get_next_val(&line),
            Part::SECOND => sum += get_prev_val(&line),
        };
    }
    
    match part {
        Part::FIRST => PartResult::FIRST(sum as isize),
        Part::SECOND => PartResult::SECOND(sum as isize),
    }
}

fn get_prev_val(seq: &Vec<i32>) -> i32 {
    let first_elem = *seq.first().unwrap();
    if seq.iter().all(|&e| e == 0) {
        return first_elem;
    }

    let mut next_seq = vec![];
    for i in 0..seq.len() - 1 {
        let dif = seq[i+1] - seq[i];
        next_seq.push(dif);
    }

    first_elem - get_prev_val(&next_seq)
}

fn get_next_val(seq: &Vec<i32>) -> i32 {
    let last_elem = *seq.last().unwrap();
    if seq.iter().all(|&e| e == 0) {
        return last_elem;
    }
    
    let mut next_seq = vec![];
    for i in 0..seq.len() - 1 {
        let dif = seq[i+1] - seq[i];
        next_seq.push(dif);
    }
    
    last_elem + get_next_val(&next_seq)
}

fn parse(s: &str) -> i32 {
    s.parse().unwrap()
}