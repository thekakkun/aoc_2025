use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/02.txt").unwrap();

    let banks = parse_input(&input);
    let joltage_sum = part_1(banks);
    println!("Part 1: {}", joltage_sum);

    // let ranges = parse_input(&input);
    // let invalid_sum = part_2(ranges);
    // println!("Part 2: {}", invalid_sum);
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<u8>> {
    input.trim().split(",").map(|bank| {
        bank.chars()
            .map(|j| j.to_string().parse().unwrap())
            .collect()
    })
}

fn part_1(banks: impl Iterator<Item = Vec<u8>>) -> u16 {
    todo!()
}

fn part_2(banks: impl Iterator<Item = Vec<u8>>) -> u16 {
    todo!()
}

fn max_joltage(bank: &Vec<u8>) -> u8 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::*;

    static TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn example_part_1() {
        let banks = parse_input(TEST_INPUT);
        let max_joltage = part_1(banks);

        assert_eq!(max_joltage, 357);
    }

    #[test]
    fn example_part_2() {
        let ranges = parse_input(TEST_INPUT);
        let invalid_sum = part_2(ranges);

        assert_eq!(invalid_sum, 4174379265);
    }
}
