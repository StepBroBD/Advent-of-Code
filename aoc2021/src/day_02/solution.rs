use std::str::FromStr;
use std::vec::Vec;

#[allow(dead_code)]
pub fn part_01(input: &Vec<String>) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for i in 0..input.len() {
        let mut instructions: Vec<&str> = input[i].split_whitespace().collect();
        if instructions[0] == "forward" {
            x += i64::from_str(instructions[1]).unwrap();
        }
        if instructions[0] == "up" {
            y -= i64::from_str(instructions[1]).unwrap();
        }
        if instructions[0] == "down" {
            y += i64::from_str(instructions[1]).unwrap();
        }
    }
    x * y
}

#[allow(dead_code)]
pub fn part_02(input: &Vec<String>) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut aim: i64 = 0;
    for i in 0..input.len() {
        let mut instructions: Vec<&str> = input[i].split_whitespace().collect();
        if instructions[0] == "forward" {
            x += i64::from_str(instructions[1]).unwrap();
            y += aim * i64::from_str(instructions[1]).unwrap();
        }
        if instructions[0] == "up" {
            aim -= i64::from_str(instructions[1]).unwrap();
        }
        if instructions[0] == "down" {
            aim += i64::from_str(instructions[1]).unwrap();
        }
    }
    x * y
}

#[cfg(test)]
mod tests {
    use colored::*;

    use util::io::input_parser::InputParser;

    use crate::day_02::solution::{part_01, part_02};

    #[test]
    fn test_aoc_2021_day_02_part_01() {
        let data = InputParser::<String>::data("../aoc2021/src/day_02/input");
        println!("{}", "##################################################".green());
        println!("AoC 2021 Day 2 Part 1: {}.", part_01(&data));
    }

    #[test]
    fn test_aoc_2021_day_02_part_02() {
        let data = InputParser::<String>::data("../aoc2021/src/day_02/input");
        println!("AoC 2021 Day 2 Part 2: {}.", part_02(&data));
        println!("{}", "##################################################".green());
    }
}
