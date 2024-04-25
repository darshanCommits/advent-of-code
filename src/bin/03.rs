advent_of_code::solution!(3);

fn to_num(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(c as u32 - ('a' as u32) + 1),
        'A'..='Z' => Some(c as u32 - ('A' as u32) + 27),
        _ => None,
    }
}

fn common_char(arr: Vec<&str>) -> char {
    arr.first()
        .unwrap()
        .chars()
        .find(|&c| {
            arr.iter()
                .skip(1)
                .all(|str| str.contains(c))
        })
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(first, second)| vec![first, second])
        .map(common_char)
        .filter_map(to_num)
        .sum::<u32>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let chunks = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|x| x.to_vec())
        .map(common_char)
        .filter_map(to_num)
        .sum::<u32>();

    Some(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(70));
    }
}
