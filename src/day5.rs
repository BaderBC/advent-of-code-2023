use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::{Part, PartResult};

// TODO: fix part 2
pub fn main(part: Part) -> PartResult {
    let input = include_str!("../assets/day5-input");
    let maps: Vec<&str> = input.split("\n\n").collect();

    let mut seeds: Vec<ResultRange> = vec![];

    let mut _seeds_to_check: Vec<u64> = maps[0].split(' ').skip(1).map(parse).collect();
    if let Part::SECOND = part {
        for i in (0..(_seeds_to_check.len() - 1)).step_by(2) {
            seeds.push(ResultRange {
                from: _seeds_to_check[i],
                range: _seeds_to_check[i + 1],
                to: _seeds_to_check[i + 1] + _seeds_to_check[i] - 1,
            });
        }
    } else {
        seeds = _seeds_to_check.iter()
            .map(|e| ResultRange {
                from: *e,
                range: 1,
                to: *e,
            })
            .collect();
    }

    let mut conversion_maps: HashMap<&str, Vec<(MapRange, MapRange)>> = HashMap::new();

    for map in maps {
        let lines: Vec<&str> = map.split('\n').collect();
        let map_name = lines[0].split(' ').next().unwrap();
        let value_lines = &lines[1..];
        let entry = conversion_maps.entry(map_name).or_default();

        for line in value_lines {
            let numbers: Vec<u64> = line.split(' ').map(parse).collect();
            let param1 = MapRange {
                from: numbers[0],
                _to: numbers[0] + numbers[2] - 1,
                range: numbers[2],
            };
            let param2 = MapRange {
                from: numbers[1],
                _to: numbers[1] + numbers[2] - 1,
                range: numbers[2],
            };
            entry.push((param2, param1));
        }
    }

    let res = get_result_ranges(conversion_maps.get("seed-to-soil").unwrap(), seeds);
    let res = get_result_ranges(conversion_maps.get("soil-to-fertilizer").unwrap(), res);
    let res = get_result_ranges(conversion_maps.get("fertilizer-to-water").unwrap(), res);
    let res = get_result_ranges(conversion_maps.get("water-to-light").unwrap(), res);
    let res = get_result_ranges(conversion_maps.get("light-to-temperature").unwrap(), res);
    let res = get_result_ranges(conversion_maps.get("temperature-to-humidity").unwrap(), res);
    let locations = get_result_ranges(conversion_maps.get("humidity-to-location").unwrap(), res);

    let result = locations.iter().map(|e| {
        println!("from: {} to: {}", e.from, e.to);
        if e.to > e.from {
            e.from
        } else {
            e.to
        }
    }).min().unwrap();
    match part {
        Part::FIRST => PartResult::FIRST(result as isize),
        Part::SECOND => PartResult::SECOND(result as isize),
    }
}

fn get_result_ranges(map: &Vec<(MapRange, MapRange)>, inputs: Vec<ResultRange>) -> Vec<ResultRange> {
    let mut results = vec![];

    for input in inputs {
        let res = get_corresponding_value(map, input);
        results.push(res);
    }

    results
}

// 2nd arg is input limit
fn get_corresponding_value(map: &Vec<(MapRange, MapRange)>, input: ResultRange) -> ResultRange {
    for (param1, param2) in map {
        if !param1.is_between(input.from) {
            continue;
        }

        let lower_range = [param2.range, input.range];
        let lower_range = lower_range.iter().min().unwrap();

        let res = ResultRange {
            from: param2.from + input.from - param1.from,
            to: param2.from + input.to - param1.from,
            range: *lower_range,
        };
        return res;
    }

    ResultRange {
        from: input.from,
        to: input.from,
        range: 1,
    }
}

fn parse(s: &str) -> u64 {
    s.parse().unwrap()
}

struct MapRange {
    from: u64,
    _to: u64,
    range: u64,
}

struct ResultRange {
    from: u64,
    to: u64,
    range: u64,
}

impl Debug for ResultRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "from: {} | to: {}", self.from, self.to)
    }
}

impl MapRange {
    fn is_between(&self, num: u64) -> bool {
        num >= self.from && (self.from + self.range - 1) >= num
    }
}