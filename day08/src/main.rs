fn part_one(input: &str) -> usize {
    let mut visible_trees: usize = 0;
    let forrest_array = input
        .lines()
        .map(|c| c.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_rows = forrest_array.len();
    let max_columns = forrest_array[0].len();

    for (i, row) in input.lines().enumerate() {
        for (j, tree) in row.chars().flat_map(|c| c.to_digit(10)).enumerate() {
            if i == 0 || j == 0 || i == max_rows - 1 || j == max_columns - 1 {
                visible_trees += 1;
                continue;
            }

            let max_left = forrest_array[i].iter().take(j).max();
            if Some(&tree) > max_left {
                visible_trees += 1;
                continue;
            }

            let max_right = forrest_array[i].iter().skip(j + 1).max();
            if Some(&tree) > max_right {
                visible_trees += 1;
                continue;
            }

            let max_top = forrest_array.iter().map(|arr| arr[j]).take(i).max();
            if Some(tree) > max_top {
                visible_trees += 1;
                continue;
            }

            let max_bottom = forrest_array.iter().map(|arr| arr[j]).skip(i + 1).max();
            if Some(tree) > max_bottom {
                visible_trees += 1;
                continue;
            }
        }
    }
    visible_trees
}

fn part_two(input: &str) -> usize {
    let forrest_array = input
        .lines()
        .map(|c| c.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_rows = forrest_array.len();
    let max_columns = forrest_array[0].len();
    input
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .flat_map(|c| c.to_digit(10))
                .enumerate()
                .map(|(j, tree)| {
                    if i == 0 || j == 0 || i == max_rows - 1 || j == max_columns - 1 {
                        return 0;
                    }

                    let mut left = forrest_array[i].iter().take(j).collect::<Vec<_>>();
                    left.reverse();

                    let mut top = forrest_array
                        .iter()
                        .map(|arr| arr[j])
                        .take(i)
                        .collect::<Vec<_>>();
                    top.reverse();

                    let mut got_true = false;
                    let bottom_count = forrest_array
                        .iter()
                        .map(|arr| arr[j])
                        .skip(i + 1)
                        .take_while(|val| {
                            if got_true {
                                got_true = false;
                                return false;
                            }
                            got_true = tree <= *val;
                            true
                        })
                        .count();

                    got_true = false;
                    let top_count = top
                        .iter()
                        .take_while(|val| {
                            if got_true {
                                got_true = false;
                                return false;
                            }
                            got_true = tree <= **val;
                            true
                        })
                        .count();

                    got_true = false;
                    let right_count = forrest_array[i]
                        .iter()
                        .skip(j + 1)
                        .take_while(|val| {
                            if got_true {
                                got_true = false;
                                return false;
                            }
                            got_true = tree <= **val;
                            true
                        })
                        .count();
                    got_true = false;
                    let left_count = left
                        .iter()
                        .take_while(|val| {
                            if got_true {
                                got_true = false;
                                return false;
                            }
                            got_true = tree <= ***val;
                            true
                        })
                        .count();

                    left_count * right_count * top_count * bottom_count
                })
                .max()
        })
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../input");
    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    static TEST_INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), 8);
    }
}
