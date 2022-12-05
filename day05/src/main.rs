use std::collections::HashMap;

fn part_one(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks_map = get_stacks(stacks);
    instructions.lines().for_each(|instruction| {
        let inst = instruction
            .split(" ")
            .flat_map(str::parse::<usize>)
            .collect::<Vec<usize>>();
        let mut temp_arr: Vec<char> = vec![];
        for _ in 0..inst[0] {
            temp_arr.push(stacks_map.get_mut(&inst[1]).unwrap().pop().unwrap());
        }
        for val in temp_arr {
            stacks_map.get_mut(&inst[2]).unwrap().push(val.to_owned());
        }
    });
    let mut string_king = String::new();
    for key in 1..stacks_map.len() + 1 {
        string_king.push(stacks_map.get_mut(&key).unwrap().pop().unwrap());
    }
    string_king
}
fn part_two(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks_map = get_stacks(stacks);
    instructions.lines().for_each(|instruction| {
        let inst = instruction
            .split(" ")
            .flat_map(str::parse::<usize>)
            .collect::<Vec<usize>>();
        let mut temp_arr: Vec<char> = vec![];
        for _ in 0..inst[0] {
            temp_arr.push(stacks_map.get_mut(&inst[1]).unwrap().pop().unwrap());
        }
        for val in temp_arr.iter().rev() {
            stacks_map.get_mut(&inst[2]).unwrap().push(val.to_owned());
        }
    });
    let mut string_king = String::new();
    for key in 1..stacks_map.len() + 1 {
        string_king.push(stacks_map.get_mut(&key).unwrap().pop().unwrap());
    }
    string_king
}

fn get_stacks(stacks: &str) -> HashMap<usize, Vec<char>> {
    let mut stacks_map: HashMap<usize, Vec<char>> = HashMap::new();
    stacks.lines().rev().skip(1).for_each(|row| {
        let each_crate = get_crates(row);
        for (index, crat) in each_crate.iter().enumerate() {
            if !crat.is_whitespace() {
                if stacks_map.contains_key(&(index + 1)) {
                    stacks_map
                        .get_mut(&(index + 1))
                        .unwrap()
                        .push(crat.to_owned())
                } else {
                    stacks_map.insert(index + 1, vec![crat.to_owned()]);
                }
            }
        }
    });
    stacks_map
}

fn get_crates(row: &str) -> Vec<char> {
    let binding = row.chars().collect::<Vec<char>>();
    binding
        .chunks(4)
        .map(|c_arr| c_arr.iter().collect::<String>())
        .map(|crat| {
            if crat.starts_with("[") {
                crat.chars().nth(1).unwrap()
            } else {
                ' '
            }
        })
        .collect::<Vec<_>>()
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
