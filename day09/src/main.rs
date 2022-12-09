use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
enum Direction {
    Right(usize),
    Up(usize),
    Left(usize),
    Down(usize),
}

impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((direction, distance_str)) => match distance_str.parse::<usize>() {
                Ok(n) => match direction {
                    "U" => Ok(Self::Up(n)),
                    "R" => Ok(Self::Right(n)),
                    "L" => Ok(Self::Left(n)),
                    "D" => Ok(Self::Down(n)),
                    _ => panic!("Invalid Direction {}", direction),
                },
                Err(e) => Err(e),
            },
            None => panic!("Invalid string compat"),
        }
    }
}

fn part_one(input: &str) -> usize {
    let mut head: (isize, isize) = (0, 0);
    let mut tail: (isize, isize) = (0, 0);
    for direction in input.lines().flat_map(str::parse::<Direction>) {
        head = move_direction(head, direction);
    }
    0
}

fn move_direction(node: (isize, isize), direction: Direction) -> (isize, isize) {
    match direction {
        Direction::Right(n) => (node.0 + n as isize, node.1),
        Direction::Up(n) => (node.0, node.1 + n as isize),
        Direction::Left(n) => (node.0 - n as isize, node.1),
        Direction::Down(n) => (node.0, node.1 - n as isize),
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::*;
    static TEST_INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), 13);
    }
}
