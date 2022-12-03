use std::collections::HashSet;

use itertools::Itertools;

static ALPHA_TO_DIGIT: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|rucksack| {
            let compartments = rucksack.split_at(rucksack.len() / 2);
            let common_item = compartments
                .0
                .chars()
                .nth(compartments.0.find(|c| compartments.1.contains(c)).unwrap())
                .unwrap();
            ALPHA_TO_DIGIT.find(common_item).unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let mut badge_letter = group.fold(String::new(), |acc, elf| {
                if acc.len() == 0 {
                    return elf.to_owned();
                }
                let acc_set: HashSet<char> = HashSet::from_iter(acc.chars());
                let elf_set: HashSet<char> = HashSet::from_iter(elf.chars());

                acc_set.intersection(&elf_set).join("")
            });
            ALPHA_TO_DIGIT.find(badge_letter.pop().unwrap()).unwrap()
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
