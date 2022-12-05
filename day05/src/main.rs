#![feature(map_many_mut)]
use std::collections::HashMap;

#[derive(Debug)]
struct Supplies {
    name: char,
}

impl Supplies {
    fn new(input: String) -> Option<Supplies> {
        if input.starts_with("[") {
            input.chars().nth(1).map(|c| Supplies { name: c })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Platform {
    stack: Vec<Supplies>,
}

impl Platform {
    fn pop(&mut self, num: usize, at_once: bool) -> Vec<Supplies> {
        let mut temp_stack = vec![];
        for _ in 0..num {
            if let Some(crat) = self.stack.pop() {
                temp_stack.push(crat)
            }
        }
        if at_once {
            temp_stack.reverse();
        }
        temp_stack
    }

    fn push(&mut self, mut stack: Vec<Supplies>) {
        self.stack.append(&mut stack);
    }

    fn get_top_shelf_name(&self) -> char {
        self.stack.last().unwrap().name
    }
}

#[derive(Debug)]
struct CargoShip {
    containers: HashMap<PlatformName, Platform>,
}

type PlatformName = usize;

impl CargoShip {
    fn transfer(&mut self, amount: usize, from: PlatformName, to: PlatformName, at_once: bool) {
        if let Some(from_platform) = self.containers.get_mut(&from) {
            let popped = from_platform.pop(amount, at_once);
            if let Some(to_platform) = self.containers.get_mut(&to) {
                to_platform.push(popped);
            }
        }
    }

    fn add_supplies(&mut self, location: PlatformName, supplies: Supplies) {
        if let Some(platform) = self.containers.get_mut(&location) {
            platform.push(vec![supplies]);
        } else {
            self.containers.insert(
                location,
                Platform {
                    stack: vec![supplies],
                },
            );
        }
    }

    fn get_top_shelf_str(&mut self) -> String {
        let mut keys = self.containers.keys().collect::<Vec<_>>();
        keys.sort();

        keys.iter()
            .flat_map(|key| {
                self.containers
                    .get(key)
                    .map(|platform| platform.get_top_shelf_name())
            })
            .collect::<String>()
    }
}

fn part_one(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut cargo_ship = load_ship(stacks);
    instructions.lines().for_each(|instruction| {
        let step = instruction
            .split(" ")
            .flat_map(str::parse::<usize>)
            .collect::<Vec<usize>>();
        cargo_ship.transfer(step[0], step[1], step[2], false);
    });

    cargo_ship.get_top_shelf_str()
}
fn part_two(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut cargo_ship = load_ship(stacks);
    instructions.lines().for_each(|instruction| {
        let step = instruction
            .split(" ")
            .flat_map(str::parse::<usize>)
            .collect::<Vec<usize>>();
        cargo_ship.transfer(step[0], step[1], step[2], true);
    });

    cargo_ship.get_top_shelf_str()
}

fn load_ship(input: &str) -> CargoShip {
    let mut cargo_ship = CargoShip {
        containers: HashMap::new(),
    };

    input.lines().rev().skip(1).for_each(|row| {
        row.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|char_arr| char_arr.iter().collect::<String>())
            .enumerate()
            .flat_map(|(index, input_str)| Supplies::new(input_str).map(|s| (index, s)))
            .for_each(|(index, supplies)| {
                cargo_ship.add_supplies(index + 1, supplies);
            });
    });
    cargo_ship
}

fn main() {
    let input = include_str!("../input");
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), "CMZ");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), "MCD");
    }
}
