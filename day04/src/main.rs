fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(into_sections)
        .filter(|chunk| {
            (chunk[0].0 <= chunk[1].0 && chunk[0].1 >= chunk[1].1)
                || (chunk[1].0 <= chunk[0].0 && chunk[1].1 >= chunk[0].1)
        })
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(into_sections)
        .filter(|chunk| chunk[0].1 >= chunk[1].0 && chunk[0].0 <= chunk[1].1)
        .count()
}

fn into_sections(line: &str) -> Vec<(u32, u32)> {
    line.split(",")
        .map(|sections| {
            let split = sections
                .split("-")
                .flat_map(str::parse)
                .collect::<Vec<u32>>();

            (split[0], split[1])
        })
        .collect::<Vec<_>>()
}
fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), 2);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), 4);
    }
}
