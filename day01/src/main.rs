fn elves_array(input: &[&str]) -> Vec<u32> {
    input.iter().fold(vec![], |mut acc, item| -> Vec<u32> {
        if let Ok(result) = item.parse::<u32>() {
            let end = acc.pop().unwrap_or(0);
            acc.push(end + result);
        } else {
            acc.push(0);
        }
        acc
    })
}
fn part_one(input: &[&str]) -> u32 {
    let arr = elves_array(input);
    arr.iter().max().unwrap().clone()
}

fn part_two(input: &[&str]) -> u32 {
    let mut arr = elves_array(input);
    arr.sort();
    arr[arr.len() - 3..].iter().sum()
}

fn main() {
    let input_slice = include_str!("../input").lines().collect::<Vec<&str>>();

    println!("Part 1: {}", part_one(&input_slice));
    println!("Part 2: {}", part_two(&input_slice));
}

#[cfg(test)]
mod tests {

    static TEST_INPUT: [&str; 14] = [
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ];

    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), 24000);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), 45000);
    }
}
