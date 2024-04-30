advent_of_code::solution!(5);

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace0, space1},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    // only me @29-04-24 and god knows what this is
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (_, amount, _, from, _, to, _)) = tuple((
            tag("move"),
            preceded(space1, map_res(digit1, str::parse)),
            preceded(space1, tag("from")),
            preceded(space1, map_res(digit1, str::parse::<usize>)),
            preceded(space1, tag("to")),
            preceded(space1, map_res(digit1, str::parse::<usize>)),
            opt(multispace0),
        ))(s)?;

        Ok((
            s,
            Self {
                amount,
                from: from - 1,
                to: to - 1,
            },
        ))
    }

    fn parse_all(s: &str) -> Vec<Instruction> {
        s.lines()
            .map(Self::parse)
            .filter_map(Result::ok)
            .map(|x| x.1)
            .collect()
    }
}

type Crate<'a> = Vec<&'a str>;
type CrateList<'a> = Vec<Crate<'a>>;

fn parse_line(s: &str) -> IResult<&str, Crate> {
    let (s, c) = separated_list1(
        // seperator of each crate
        tag(" "),
        alt((
            // parses empty crate to empty string
            map(tag("   "), |_| ""),
            // parses crate to char(&str)
            delimited(complete::char('['), alpha1, complete::char(']')),
        )),
    )(s)?;

    Ok((s, c))
}

fn parse_crates(s: &str) -> CrateList {
    let matrix: CrateList = s
        .lines()
        .map(parse_line)
        .filter_map(Result::ok)
        .map(|x| x.1)
        .collect();

    let max_len = matrix[0].len();

    let transposed = (0..max_len)
        .map(|i| {
            matrix
                .iter()
                .map(|arr| arr[i])
                .filter(|&x| !x.is_empty())
                .rev()
                .collect()
        })
        .collect();

    transposed
}

pub fn part_one(input: &str) -> Option<String> {
    let res = input.split_once("\n\n").unwrap();

    let mut parsed_crates = parse_crates(res.0);
    let parsed_inst = Instruction::parse_all(res.1);

    // for loops are atleast better than for each loops.
    for Instruction { amount, from, to } in parsed_inst {
        let len = parsed_crates[from].len();

        let drained = &parsed_crates[from]
            .drain((len - amount)..)
            .rev()
            .collect::<Vec<_>>();

        for &x in drained {
            parsed_crates[to].push(x);
        }
    }

    let res = parsed_crates
        .iter()
        .map(|x| x.last().unwrap())
        .join("");

    Some(res)
}

pub fn part_two(input: &str) -> Option<String> {
    let res = input.split_once("\n\n").unwrap();

    let mut parsed_crates = parse_crates(res.0);
    let parsed_inst = Instruction::parse_all(res.1);

    // for loops are atleast better than for each loops.
    for Instruction { amount, from, to } in parsed_inst {
        let len = parsed_crates[from].len();

        let drained = &parsed_crates[from]
            .drain((len - amount)..)
            .collect::<Vec<_>>();

        for &x in drained {
            parsed_crates[to].push(x);
        }
    }

    let res = parsed_crates
        .iter()
        .map(|x| x.last().unwrap())
        .join("");

    Some(res)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("cmz".to_uppercase()));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("mcd".to_uppercase()));
    }
}
