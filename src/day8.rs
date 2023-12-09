use std::collections::HashMap;
use crate::{Part, PartResult};

type InstructionsT<'a> = HashMap<&'a str, (&'a str, &'a str)>;

pub fn main(part: Part) -> PartResult {
    let input = include_str!("../assets/day8-input");

    let mut split_input = input.split("\n\n");

    let sequence = split_input.next().unwrap();
    let sequence = sequence.chars().map(|e| {
        if e == 'L' {
            Direction::LEFT
        } else {
            Direction::RIGHT
        }
    })
        .collect::<Vec<Direction>>();

    let instructions_str = split_input.next().unwrap();
    let mut instructions: InstructionsT = HashMap::new();

    for instruction in instructions_str.split('\n') {
        let mut split = instruction.split('=').map(|e| e.trim());
        let node = split.next().unwrap();
        let corresponding = split.next().unwrap();
        let corresponding = &corresponding[1..corresponding.len() - 1];
        let corresponding = corresponding.split(", ").collect::<Vec<&str>>();

        instructions.insert(node, (corresponding[0], corresponding[1]));
    }

    drop(split_input);

    let res = get_steps_count(&part, &sequence, &instructions);

    match part {
        Part::FIRST => PartResult::FIRST(res),
        Part::SECOND => PartResult::SECOND(res),
    }
}

fn get_steps_count(part: &Part, sequence: &Vec<Direction>, instructions: &InstructionsT) -> isize {
    let mut steps_count = 0;
    let mut seq_index = 0;
    let mut nodes = vec!["AAA"];
    
    if let Part::SECOND = part {
        nodes = vec![];
        for (node, _) in instructions {
            if node.ends_with('A') {
                nodes.push(node);
            }
        }
    }

    loop {
        for (i, node) in nodes.clone().iter().enumerate() {
            nodes[i] = get_corresponding_node(sequence, seq_index, instructions, node);
        }
        seq_index += 1;
        if sequence.len() <= seq_index {
            seq_index = 0;
        }

        steps_count += 1;
        if let Part::FIRST = part {
            if nodes[0] == "ZZZ" {
                break;
            } else {
                continue;
            }
        }

        if nodes.clone().iter()
            .all(|e| e.chars().collect::<Vec<char>>()[2] == 'Z') {
            println!("nodes: {:?}", nodes);
            break;
        }
    }

    steps_count
}

fn get_corresponding_node<'a>(sequence: &Vec<Direction>, seq_index: usize, instructions: &'a InstructionsT, current_node: &str) -> &'a str {
    let direction = &sequence[seq_index];
    let possibilities = instructions.get(current_node).unwrap();

    match direction {
        Direction::LEFT => possibilities.0,
        Direction::RIGHT => possibilities.1,
    }
}

enum Direction {
    LEFT,
    RIGHT,
}
