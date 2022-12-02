#[derive(Clone, Copy)]
enum Outcome {
    DRAW = 3,
    WIN = 6,
    LOSE = 0,
}

#[derive(PartialEq, Clone, Copy)]
enum Move {
    ROCK = 0,
    PAPER = 1,
    SCISSORS = 2,
}

fn part_one(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| -> u32 {
            let game = line.split(" ").collect::<Vec<&str>>();

            let op_move = Move::from_str(game[0]);
            let player_move = Move::from_str(game[1]);
            get_outcome(op_move, player_move) as u32 + player_move as u32 + 1
        })
        .sum()
}

fn get_outcome(op_move: Move, player_move: Move) -> Outcome {
    if op_move == player_move {
        Outcome::DRAW
    } else if get_move(op_move, Outcome::WIN) == player_move {
        Outcome::WIN
    } else {
        Outcome::LOSE
    }
}

fn part_two(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| -> u32 {
            let game = line.split(" ").collect::<Vec<&str>>();
            let op_move = Move::from_str(game[0]);
            let outcome = Outcome::from_str(game[1]);
            let player_move = get_move(op_move, outcome);
            outcome as u32 + player_move as u32 + 1
        })
        .sum()
}

fn get_move(op_move: Move, outcome: Outcome) -> Move {
    let move_int = match outcome {
        Outcome::DRAW => op_move as u32,
        Outcome::WIN => (op_move as u32 + 1) % 3,
        Outcome::LOSE => (op_move as u32 + 2) % 3,
    };
    Move::from_u32(move_int)
}

fn main() {
    let input = include_str!("../input").lines().collect::<Vec<&str>>();

    println!("Part 1: {}", part_one(&input));
    println!("part 2: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: [&str; 3] = ["A Y", "B X", "C Z"];

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), 15);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), 12);
    }
}
impl Move {
    fn from_u32(move_int: u32) -> Move {
        match move_int {
            0 => Move::ROCK,
            1 => Move::PAPER,
            2 => Move::SCISSORS,
            _ => panic!("Move {:?} not found", move_int),
        }
    }

    fn from_str(move_str: &str) -> Move {
        match move_str {
            "A" | "X" => Move::ROCK,
            "B" | "Y" => Move::PAPER,
            "C" | "Z" => Move::SCISSORS,
            _ => panic!("Move {:?} not known", move_str),
        }
    }
}
impl Outcome {
    fn from_str(outcome_str: &str) -> Outcome {
        match outcome_str {
            "X" => Outcome::LOSE,
            "Y" => Outcome::DRAW,
            "Z" => Outcome::WIN,
            _ => panic!("Outcome {:?} not known", outcome_str),
        }
    }
}
