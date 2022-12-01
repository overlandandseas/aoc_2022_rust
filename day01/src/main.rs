fn elves_array(input: &[&str]) -> Vec<u32> {
    let mut arr = vec![];
    let mut current_calories = 0;
    input.iter().for_each(|item| match item.parse::<u32>() {
        Ok(result) => {
            current_calories = current_calories + result;
        }
        Err(_) => {
            arr.push(current_calories);
            current_calories = 0;
        }
    });
    arr.push(current_calories);
    arr
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
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];
        assert_eq!(part_one(&test_input), 24000);
    }
    #[test]
    fn test_part_two() {
        let test_input = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];
        assert_eq!(part_two(&test_input), 45000);
    }
}
