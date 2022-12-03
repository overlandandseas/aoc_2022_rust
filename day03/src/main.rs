use std::collections::HashSet;

use itertools::Itertools;

static ALPHA_TO_DIGIT: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn part_one(input: &str) -> usize {
    input
        .lines()
        .flat_map(|rucksack| {
            let compartments = rucksack.split_at(rucksack.len() / 2);
            compartments
                .0
                .chars()
                .find(|c| compartments.1.contains(c.clone()))
                .map(|common_item| ALPHA_TO_DIGIT.find(common_item))
                .unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|group| {
            group
                .fold(String::new(), |acc, elf| {
                    if acc.is_empty() {
                        return elf.to_owned();
                    }
                    let acc_set: HashSet<char> = HashSet::from_iter(acc.chars());
                    let elf_set: HashSet<char> = HashSet::from_iter(elf.chars());

                    acc_set.intersection(&elf_set).join("")
                })
                .pop()
                .map(|badge| ALPHA_TO_DIGIT.find(badge))
                .unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input");
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), 157);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), 70);
    }
}
