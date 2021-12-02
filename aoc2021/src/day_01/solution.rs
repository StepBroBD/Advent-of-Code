use std::vec::*;

#[allow(dead_code)]
pub fn part_01(input: &Vec<i64>) -> i64 {
    let mut result: i64 = 0;
    for (index, _value) in input.iter().enumerate() {
        if index != 0 && &input[index] > &input[index - 1] {
            result += 1;
        }
    }
    result
}

#[allow(dead_code)]
pub fn part_02(input: &Vec<i64>) -> i64 {
    let mut result: i64 = 0;
    for (index, _value) in input.iter().enumerate() {
        if index > 2 && &input[index - 2] + &input[index - 1] + &input[index] > &input[index - 3] + &input[index - 2] + &input[index - 1] {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use colored::*;

    use util::io::input_parser::InputParser;

    use crate::day_01::solution::{part_01, part_02};

    #[test]
    fn test_aoc_2021_day_01_part_01() {
        let data = InputParser::<i64>::data("../aoc2021/src/day_01/input");
        println!("{}", "##################################################".green());
        println!("AoC 2021 Day 1 Part 1: {}.", part_01(&data));
    }

    #[test]
    fn test_aoc_2021_day_01_part_02() {
        let data = InputParser::<i64>::data("../aoc2021/src/day_01/input");
        println!("AoC 2021 Day 1 Part 2: {}.", part_02(&data));
        println!("{}", "##################################################".green());
    }
}
