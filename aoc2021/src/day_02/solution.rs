//! AoC 2021 Day 1.
//!
//! https://adventofcode.com/2021/day/2

use std::str::FromStr;

#[allow(dead_code)]
pub fn part_01(input: &[String]) -> i64 {
    let mut x = 0;
    let mut y = 0;
    for line in input {
        let command: Vec<&str> = line.split_whitespace().collect();
        if command[0] == "forward" {
            x += i64::from_str(command[1]).unwrap();
        }
        if command[0] == "up" {
            y -= i64::from_str(command[1]).unwrap();
        }
        if command[0] == "down" {
            y += i64::from_str(command[1]).unwrap();
        }
    }
    x * y
}

#[allow(dead_code)]
pub fn part_02(input: &[String]) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in input {
        let command: Vec<&str> = line.split_whitespace().collect();
        if command[0] == "forward" {
            x += i64::from_str(command[1]).unwrap();
            y += aim * i64::from_str(command[1]).unwrap();
        }
        if command[0] == "up" {
            aim -= i64::from_str(command[1]).unwrap();
        }
        if command[0] == "down" {
            aim += i64::from_str(command[1]).unwrap();
        }
    }
    x * y
}

#[cfg(test)]
mod tests {
    use util::io::input_parser::InputParser;

    use crate::day_02::solution::{part_01, part_02};

    #[test]
    fn test_aoc_2021_day_02_part_01() {
        let input = InputParser::<String>::data("../aoc2021/src/day_02/input");
        println!("AoC 2021 Day 2 Part 1: {}.", part_01(&input));
    }

    #[test]
    fn test_aoc_2021_day_02_part_02() {
        let input = InputParser::<String>::data("../aoc2021/src/day_02/input");
        println!("AoC 2021 Day 2 Part 2: {}.", part_02(&input));
    }
}
