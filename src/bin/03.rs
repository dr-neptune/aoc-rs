advent_of_code::solution!(3);
use regex::Regex;

fn parse_multiplications(input: &str) -> Result<Vec<(u32, u32)>, regex::Error> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    Ok(re
        .captures_iter(input)
        .map(|cap| (
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap()
        ))
        .collect())
}

fn dot_product(pairs: &[(u32, u32)]) -> u32 {
    pairs.iter().map(|(a, b)| a * b).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_multiplications(input)
        .map(|pairs| dot_product(&pairs))
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(?s)do\(\).*?don't\(\)").ok()?;

    let result = re
        .find_iter(input)
        .filter_map(|m| parse_multiplications(m.as_str()).ok())
        .map(|pairs| dot_product(&pairs))
        .sum();

    Some(result)
}
