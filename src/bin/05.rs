advent_of_code::solution!(5);
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space1},
    combinator::{map_res, opt},
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Instruction {
    amount: u32,
    from: u32,
    to: u32,
}

impl Instruction {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (_, amount, _, from, _, to, _)) = tuple((
            tag("move"),
            preceded(space1, map_res(digit1, str::parse)),
            preceded(space1, tag("from")),
            preceded(space1, map_res(digit1, str::parse)),
            preceded(space1, tag("to")),
            preceded(space1, map_res(digit1, str::parse)),
            opt(multispace0),
        ))(s)?;

        Ok((s, Self { amount, from, to }))
    }

    fn parse_all(s: &str) -> IResult<&str, Vec<Self>> {
        many0(Self::parse)(s)
    }
}

#[derive(Debug)]
struct Crates;

impl Crates {
    fn parse() {}
    fn parse_all() {}
    fn shift() {}
}

pub fn part_one(input: &str) -> Option<String> {
    let res = input.split_once("\n\n").unwrap().1;

    dbg!(Instruction::parse_all(res));

    // dbg!(parse_inst("move 1 from 8 to 4"));
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
    #[ignore = "reason"]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
