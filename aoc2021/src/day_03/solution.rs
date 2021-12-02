use std::vec::Vec;

#[allow(dead_code)]
pub fn part_01(_input: &Vec<String>) -> i64 {
    0
}

#[allow(dead_code)]
pub fn part_02(_input: &Vec<String>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use util::io::input_parser::InputParser;

    use crate::day_03::solution::{part_01, part_02};

    #[test]
    fn test_aoc_2021_day_03_part_01() {
        let data = InputParser::<String>::data("../aoc2021/src/day_03/input");
        println!("AoC 2021 Day 3 Part 1: {}.", part_01(&data));
    }

    #[test]
    fn test_aoc_2021_day_03_part_02() {
        let data = InputParser::<String>::data("../aoc2021/src/day_03/input");
        println!("AoC 2021 Day 3 Part 2: {}.", part_02(&data));
    }
}
