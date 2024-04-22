use core::panic;
use itertools::Itertools;
use std::cmp::Ordering;

advent_of_code::solution!(2);

#[derive(PartialEq, Copy, Clone, Debug)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Move::*;

        Some(match (self, other) {
            (Rock, Paper) | (Paper, Scissor) | (Scissor, Rock) => {
                Ordering::Less
            },

            (Rock, Scissor) | (Paper, Rock) | (Scissor, Paper) => {
                Ordering::Greater
            },

            (_, _) => Ordering::Equal,
        })
    }
}

impl Move {
    fn from(a: char) -> Option<Self> {
        match a {
            'A' | 'X' => Some(Move::Rock),
            'B' | 'Y' => Some(Move::Paper),
            'C' | 'Z' => Some(Move::Scissor),
            _ => None,
        }
    }

    fn to_num(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }

    fn win(self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
        }
    }

    fn lose(self) -> Self {
        match self {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
        }
    }

    fn fight2(self, other: char) -> u32 {
        match other {
            // x is lose, y is draw, z is win
            'X' => self.lose().to_num(),
            'Y' => 3 + self.to_num(),
            'Z' => 6 + self.win().to_num(),
            _ => panic!("Invalid"),
        }
    }
}

#[derive(Debug)]
struct Round(Move, Move);

// works only for 1 it will complicate things a lot if i introduce another enum to handle the part2 this way.
impl Round {
    fn new(x: (Move, Move)) -> Self {
        Round(x.0, x.1)
    }

    fn score(self) -> u32 {
        match self.0.partial_cmp(&self.1) {
            Some(Ordering::Equal) => 3 + self.1.to_num(),
            Some(Ordering::Less) => 6 + self.1.to_num(),
            Some(Ordering::Greater) => self.1.to_num(),
            None => {
                panic!("moves should be comparable")
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .chars()
        .filter_map(Move::from)
        .tuples()
        .map(Round::new)
        .map(Round::score)
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<char> = line
                .chars()
                .filter(|&c| !c.is_whitespace())
                .collect();

            let opp_mv = Move::from(moves[0]).unwrap();
            opp_mv.fight2(moves[1])
        })
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
