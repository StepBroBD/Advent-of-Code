#[allow(dead_code)]
pub fn part_01(input: &Vec<i64>) -> i64 { 0 }

#[allow(dead_code)]
pub fn part_02(input: &Vec<i64>) -> i64 { 0 }

#[cfg(test)]
mod tests {
    use colored::*;

    use util::io::input_parser::InputParser;

    use crate::day_02::solution::{part_01, part_02};

    #[test]
    fn test_aoc_2021_day_02_part_01() {
        let data = InputParser::<i64>::data("../aoc2021/src/day_02/input");
        println!("{}", "##################################################".green());
        println!("AoC 2021 Day 2 Part 1: {}.", part_01(&data));
    }

    #[test]
    fn test_aoc_2021_day_02_part_02() {
        let data = InputParser::<i64>::data("../aoc2021/src/day_02/input");
        println!("AoC 2021 Day 2 Part 2: {}.", part_02(&data));
        println!("{}", "##################################################".green());
    }
}
