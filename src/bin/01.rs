advent_of_code::solution!(1);
use itertools::EitherOrBoth;
use itertools::Itertools;

fn parse_and_sort(input: &str) -> (Vec<i32>, Vec<i32>) {
    fn parse_line(line: &str) -> (i32, i32) {
        line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap()
    }

    let (mut first_elements, mut second_elements): (Vec<i32>, Vec<i32>) =
        input.lines().map(parse_line).unzip();

    first_elements.sort_unstable();
    second_elements.sort_unstable();

    (first_elements, second_elements)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (first_elements, second_elements) = parse_and_sort(input);

    let total_sum: i32 = first_elements
        .iter()
        .zip(second_elements.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Some(total_sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (first_elements, second_elements) = parse_and_sort(input);

    // Group `second_elements` by value and count occurrences using `dedup_with_count`
    // `dedup_with_count` returns `(count, &x)` tuples
    let grouped_second = second_elements
        .iter()
        .dedup_with_count()
        .map(|(count, &x)| (x, count as i32));

    // Perform a merge join between `first_elements` and `grouped_second`
    let merged = first_elements
        .iter()
        .copied() // Converts `&i32` to `i32`
        .merge_join_by(grouped_second, |a, b| a.cmp(&b.0));

    // Sum the products where elements match
    let total_sum: i32 = merged
        .filter_map(|output| match output {
            EitherOrBoth::Both(a, b) => Some(a as i32 * b.1), // Multiply `a` by `count`
            _ => None, // Ignore elements not present in both iterators
        })
        .sum();

    Some(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
