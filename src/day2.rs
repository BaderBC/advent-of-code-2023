use crate::{Part, PartResult};

#[derive(Debug)]
struct RevealedCube {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl RevealedCube {
    fn is_game_possible(&self) -> bool {
        let game_possible = !compare_option_to_num(self.red, 12);
        let game_possible = game_possible && !compare_option_to_num(self.green, 13);
        let game_possible = game_possible && !compare_option_to_num(self.blue, 14);

        game_possible
    }

    fn from_str(reveal: &str) -> Self {
        let mut result = Self {
            red: None,
            green: None,
            blue: None,
        };

        let reveal = reveal.replace(' ', "");
        let values = reveal.split(',').collect::<Vec<&str>>();
        
        for value in values {
            if value.contains("red") {
                let num = &value[0..value.len() - 3].parse::<u32>().unwrap();
                result.red = Some(*num);
            }
            if value.contains("green") {
                let num = &value[0..value.len() - 5].parse::<u32>().unwrap();
                result.green = Some(*num);
            }
            if value.contains("blue") {
                let num = &value[0..value.len() - 4].parse::<u32>().unwrap();
                result.blue = Some(*num);
            }
        }

        result
    }
}

fn compare_option_to_num(option: Option<u32>, num: u32) -> bool {
    if let Some(v) = option {
        v > num
    } else {
        false
    }
}

pub fn main(part: Part) -> PartResult {
    _ = part;

    let input = include_str!("../assets/day2-input");
    
    let mut game_ids_sum: u32 = 0;
    let mut power_sum: u32 = 0;

    for line in input.split('\n') {
        let split = line.split(':').collect::<Vec<&str>>();
        let game_id = get_game_id(split[0]);

        let mut is_game_possible = true;
        let mut cubes = vec![];
        
        for reveal in split[1].split(';') {
            let revealed_cube = RevealedCube::from_str(reveal);
            is_game_possible = is_game_possible && revealed_cube.is_game_possible();
            
            cubes.push(revealed_cube);
        }

        if is_game_possible {
            game_ids_sum += game_id;
        }
        
        power_sum += get_power(cubes);
    }
    
    match part {
        Part::FIRST => PartResult::FIRST(game_ids_sum as isize),
        Part::SECOND => PartResult::SECOND(power_sum as isize),
    }
}

fn get_game_id(game_id_string: &str) -> u32 {
    let split = game_id_string.split(' ').collect::<Vec<&str>>();
    split[1].parse::<u32>().expect("Wrong number")
}

fn get_power(revealed_cubes: Vec<RevealedCube>) -> u32 {
    let mut highest_red = 0;
    let mut highest_green = 0;
    let mut highest_blue = 0;
    
    for cube in revealed_cubes {
        if compare_option_to_num(cube.red, highest_red) {
            highest_red = cube.red.unwrap_or(highest_red);
        }
        if compare_option_to_num(cube.green, highest_green) {
            highest_green = cube.green.unwrap_or(highest_green);
        }
        if compare_option_to_num(cube.blue, highest_blue) {
            highest_blue = cube.blue.unwrap_or(highest_blue);
        }
    }
    
    highest_red * highest_green * highest_blue
}
