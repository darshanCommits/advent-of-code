use itertools::Itertools;
use std::num::ParseIntError;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Pair(u32, u32);

impl Pair {
    fn build(x: (&str, &str)) -> Result<Pair, ParseIntError> {
        let first = x.0.to_string().parse::<u32>()?;
        let second = x.1.to_string().parse::<u32>()?;

        Ok(Self(first, second))
    }

    fn is_overlapping(&self, other: &Self) -> bool {
        let p1 = other.0 >= self.0 && other.1 <= self.1;
        let p2 = self.0 >= other.0 && self.1 <= other.1;

        p1 || p2
    }

    fn is_overlapping_at_all(&self, other: &Self) -> bool {
        let range1 = self.0..self.1;
        let range2 = other.0..other.1;

        range1.start <= range2.end && range2.start <= range1.end
    }

    fn split_class(line: &str) -> Option<(Pair, Pair)> {
        line.split(',')
            .map(|x| x.split_once('-'))
            .filter_map(|y| y.and_then(|pair| Pair::build(pair).ok()))
            .tuples::<(Pair, Pair)>()
            .next()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .filter_map(Pair::split_class)
        .filter(|(x, y)| Pair::is_overlapping(x, y))
        .count() as u32;

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .filter_map(Pair::split_class)
        .filter(|(x, y)| Pair::is_overlapping_at_all(x, y))
        .count() as u32;

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "reason"]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
