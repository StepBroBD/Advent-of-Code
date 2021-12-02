//! AoC 2021 Day 1.
//!
//! https://adventofcode.com/2021/day/3

#[allow(dead_code)]
pub fn part_01(_input: &[String]) -> i64 {
    0
}

#[allow(dead_code)]
pub fn part_02(_input: &[String]) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use util::io::input_parser::InputParser;

    use crate::day_03::solution::{part_01, part_02};

    #[test]
    fn test_aoc_2021_day_03_part_01() {
        let input = InputParser::<String>::data("../aoc2021/src/day_03/input");
        println!("AoC 2021 Day 3 Part 1: {}.", part_01(&input));
    }

    #[test]
    fn test_aoc_2021_day_03_part_02() {
        let input = InputParser::<String>::data("../aoc2021/src/day_03/input");
        println!("AoC 2021 Day 3 Part 2: {}.", part_02(&input));
    }
}
