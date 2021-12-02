//! AoC 2021 Day 1.
//!
//! https://adventofcode.com/2021/day/1

#[allow(dead_code)]
pub fn part_01(input: &[i64]) -> i64 {
    let mut result = 0;
    for i in 0..input.len() {
        if i != 0 && input[i] > input[i - 1] {
            result += 1;
        }
    }
    result
}

#[allow(dead_code)]
pub fn part_02(input: &[i64]) -> i64 {
    let mut result = 0;
    for i in 0..input.len() {
        if i > 2
            && input[i - 2] + input[i - 1] + input[i] > input[i - 3] + input[i - 2] + input[i - 1]
        {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use util::io::input_parser::InputParser;

    use crate::day_01::solution::{part_01, part_02};

    #[test]
    fn test_aoc_2021_day_01_part_01() {
        let input = InputParser::<i64>::data("../aoc2021/src/day_01/input");
        println!("AoC 2021 Day 1 Part 1: {}.", part_01(&input));
    }

    #[test]
    fn test_aoc_2021_day_01_part_02() {
        let input = InputParser::<i64>::data("../aoc2021/src/day_01/input");
        println!("AoC 2021 Day 1 Part 2: {}.", part_02(&input));
    }
}
