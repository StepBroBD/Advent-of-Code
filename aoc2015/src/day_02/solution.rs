//! AoC 2015 Day 2.
//!
//! https://adventofcode.com/2015/day/2

use std::str::FromStr;

#[allow(dead_code)]
pub fn part_01(input: &[String]) -> i64 {
    let mut result = 0;
    for line in input {
        let splits: Vec<_> = line.split('x').collect();
        let l = i64::from_str(splits[0]).unwrap();
        let w = i64::from_str(splits[1]).unwrap();
        let h = i64::from_str(splits[2]).unwrap();
        let mut smallest_side_area = 0;
        if l >= w && l >= h {
            smallest_side_area = w * h;
        }
        if w >= l && w >= h {
            smallest_side_area = l * h;
        }
        if h >= l && h >= w {
            smallest_side_area = l * w;
        }
        // println!("l: {}, w: {}, h: {}, a: {}", l, w, h, smallest_side_area);
        result += 2 * l * w + 2 * w * h + 2 * h * l + smallest_side_area;
    }
    result
}

#[allow(dead_code)]
pub fn part_02(input: &[String]) -> i64 {
    let mut result = 0;
    for line in input {
        let splits: Vec<_> = line.split('x').collect();
        let l = i64::from_str(splits[0]).unwrap();
        let w = i64::from_str(splits[1]).unwrap();
        let h = i64::from_str(splits[2]).unwrap();
        let mut partial_ribbon = 0;
        if l >= w && l >= h {
            partial_ribbon = w + w + h + h;
        }
        if w >= l && w >= h {
            partial_ribbon = l + l + h + h;
        }
        if h >= l && h >= w {
            partial_ribbon = l + l + w + w;
        }
        // println!("l: {}, w: {}, h: {}, a: {}", l, w, h, partial_ribbon);
        result += partial_ribbon + l * w * h;
    }
    result
}

#[cfg(test)]
mod tests {
    use util::io::input_parser::InputParser;

    use crate::day_02::solution::{part_01, part_02};

    #[test]
    fn test_aoc_2021_day_02_part_01() {
        let input = InputParser::<String>::data("../aoc2015/src/day_02/input");
        println!("AoC 2015 Day 2 Part 1: {}.", part_01(&input));
    }

    #[test]
    fn test_aoc_2021_day_02_part_02() {
        let input = InputParser::<String>::data("../aoc2015/src/day_02/input");
        println!("AoC 2015 Day 2 Part 2: {}.", part_02(&input));
    }
}
