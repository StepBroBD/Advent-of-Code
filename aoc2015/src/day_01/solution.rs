//! AoC 2015 Day 1.
//!
//! https://adventofcode.com/2015/day/1

#[allow(dead_code)]
pub fn part_01(input: &[String]) -> i64 {
    let mut result = 0;
    let instructions: Vec<_> = input[0].chars().collect();
    for instruction in instructions {
        match instruction {
            '(' => result += 1,
            ')' => result -= 1,
            _ => panic!("Instruction not recognized."),
        }
    }
    result
}

#[allow(dead_code)]
pub fn part_02(input: &[String]) -> i64 {
    let mut floor_count = 0;
    let instructions: Vec<_> = input[0].chars().collect();
    for (position, instruction) in instructions.iter().enumerate() {
        match instruction {
            '(' => floor_count += 1,
            ')' => floor_count -= 1,
            _ => panic!("Instruction not recognized."),
        }
        if floor_count == -1 {
            return (position + 1) as i64;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use util::io::input_parser::InputParser;

    use crate::day_01::solution::{part_01, part_02};

    #[test]
    fn test_aoc_2015_day_01_part_01() {
        let input = InputParser::<String>::data("../aoc2015/src/day_01/input");
        println!("AoC 2015 Day 1 Part 1: {}.", part_01(&input));
    }

    #[test]
    fn test_aoc_2015_day_01_part_02() {
        let input = InputParser::<String>::data("../aoc2015/src/day_01/input");
        println!("AoC 2015 Day 1 Part 2: {}.", part_02(&input));
    }
}
