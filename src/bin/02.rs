use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let input = read_to_string("input/02.txt").unwrap();

    let ranges = parse_input(&input);
    let invalid_sum = part_1(ranges);
    println!("Part 1: {}", invalid_sum);

    let ranges = parse_input(&input);
    let invalid_sum = part_2(ranges);
    println!("Part 2: {}", invalid_sum);
}

fn parse_input(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input.trim().split(",").map(|range| {
        let (start, end) = range.split_once("-").unwrap();
        start.parse().unwrap()..=end.parse().unwrap()
    })
}

fn part_1(ranges: impl Iterator<Item = RangeInclusive<u64>>) -> u64 {
    ranges.flat_map(|range| range.filter(is_duplicated)).sum()
}

fn part_2(ranges: impl Iterator<Item = RangeInclusive<u64>>) -> u64 {
    ranges.flat_map(|range| range.filter(is_repeated)).sum()
}

fn is_duplicated(val: &u64) -> bool {
    let val = val.to_string();

    if !val.len().is_multiple_of(2) {
        return false;
    };

    let (first, second) = val.split_at(val.len() / 2);
    first == second
}

fn is_repeated(val: &u64) -> bool {
    let val = val.to_string();

    let divisors = get_divisors(&val.len());
    divisors.iter().any(|d| {
        let binding = val.chars().collect::<Vec<_>>();
        let mut chunks = binding.chunks(*d);
        let first = chunks.next().unwrap();
        chunks.all(|c| c == first)
    })
}

fn get_divisors(val: &usize) -> Vec<usize> {
    (1..=val.isqrt())
        .filter(|&i| val.is_multiple_of(i))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    static TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn example_part_1() {
        let ranges = parse_input(TEST_INPUT);
        let invalid_sum = part_1(ranges);

        assert_eq!(invalid_sum, 1227775554);
    }

    #[test]
    fn example_part_2() {
        let ranges = parse_input(TEST_INPUT);
        let invalid_sum = part_2(ranges);

        assert_eq!(invalid_sum, 4174379265);
    }
}
