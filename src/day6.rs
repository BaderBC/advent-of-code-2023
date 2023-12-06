use crate::{Part, PartResult};

pub fn main(part: Part) -> PartResult {
    let input = include_str!("../assets/day6-input");
    let lines: Vec<&str> = input.split('\n').collect();

    let mut times: Vec<u64> = lines[0].split_whitespace().skip(1).map(parse).collect();
    let mut distances: Vec<u64> = lines[1].split_whitespace().skip(1).map(parse).collect();
    
    if let Part::SECOND = part {
        times = vec![times.iter().map(|e| e.to_string())
            .collect::<Vec<String>>().join("")
            .parse::<u64>().unwrap()];
        distances = vec![distances.iter().map(|e| e.to_string())
            .collect::<Vec<String>>().join("")
            .parse::<u64>().unwrap()];
    }

    let mut result = 1;
    for i in 0..times.len() {
        let all_distances = get_all_distances(times[i]);
        result *= all_distances.iter()
            .filter(|&&e| e > distances[i])
            .count();
    }

    PartResult::FIRST(result as isize)
}

fn get_all_distances(time: u64) -> Vec<u64> {
    let mut results = vec![];

    for pressing_time in 1..time {
        results.push((time - pressing_time) * pressing_time);
    }

    results
}

fn parse(s: &str) -> u64 {
    s.parse().unwrap()
}