use std::collections::HashSet;

fn part_one(input: &str) -> usize {
    get_first_unique(input, 4)
}

fn part_two(input: &str) -> usize {
    get_first_unique(input, 14)
}

fn get_first_unique(input: &str, n: usize) -> usize {
    input
        .as_bytes()
        .windows(n)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == n)
        .unwrap()
        + n
}
fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
