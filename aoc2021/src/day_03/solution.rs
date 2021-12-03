//! AoC 2021 Day 3.
//!
//! https://adventofcode.com/2021/day/3

#[allow(dead_code)]
#[allow(clippy::all)]
pub fn part_01(input: &[String]) -> i64 {
    let mut gamma_rate = "".to_string();
    let mut epsilon_rate = "".to_string();
    {
        // Counting 0s, gamma and epsilon are in two's compliment.
        let mut index_00_count = 0;
        let mut index_01_count = 0;
        let mut index_02_count = 0;
        let mut index_03_count = 0;
        let mut index_04_count = 0;
        let mut index_05_count = 0;
        let mut index_06_count = 0;
        let mut index_07_count = 0;
        let mut index_08_count = 0;
        let mut index_09_count = 0;
        let mut index_10_count = 0;
        let mut index_11_count = 0;
        for line in input {
            if line.chars().nth(0).unwrap() == '0' {
                index_00_count += 1;
            }
            if line.chars().nth(1).unwrap() == '0' {
                index_01_count += 1;
            }
            if line.chars().nth(2).unwrap() == '0' {
                index_02_count += 1;
            }
            if line.chars().nth(3).unwrap() == '0' {
                index_03_count += 1;
            }
            if line.chars().nth(4).unwrap() == '0' {
                index_04_count += 1;
            }
            if line.chars().nth(5).unwrap() == '0' {
                index_05_count += 1;
            }
            if line.chars().nth(6).unwrap() == '0' {
                index_06_count += 1;
            }
            if line.chars().nth(7).unwrap() == '0' {
                index_07_count += 1;
            }
            if line.chars().nth(8).unwrap() == '0' {
                index_08_count += 1;
            }
            if line.chars().nth(9).unwrap() == '0' {
                index_09_count += 1;
            }
            if line.chars().nth(10).unwrap() == '0' {
                index_10_count += 1;
            }
            if line.chars().nth(11).unwrap() == '0' {
                index_11_count += 1;
            }
        }
        if 1000 - index_00_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_01_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_02_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_03_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_04_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_05_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_06_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_07_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_08_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_09_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_10_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        if 1000 - index_11_count > 500 {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
        for c in gamma_rate.chars() {
            if c == '0' {
                epsilon_rate.push('1');
            }
            if c == '1' {
                epsilon_rate.push('0');
            }
        }
    }
    i64::from_str_radix(gamma_rate.as_str(), 2).unwrap() * i64::from_str_radix(epsilon_rate.as_str(), 2).unwrap()
}

#[allow(dead_code)]
pub fn part_02(input: &[String]) -> i64 {
    let o2_rating;
    let co2_rating;
    {
        let mut prefix = "".to_string();
        // Counting 0s.
        let mut zero_counts: Vec<i64> = vec![0; 12];
        // Counting 1s.
        let mut one_counts: Vec<i64> = vec![0; 12];
        let mut selections: Vec<String> = Vec::from(input);
        // Index of each position in binary numbers.
        for char_position in 0..12 {
            // println!("Iteration {}, Selections: {:?}, Prefix: {}.", char_position + 1, selections, prefix);
            // println!("Zero Counts: {:?}, One Counts: {:?}.", zero_counts, one_counts);
            // Loop through each lines.
            for line in &mut *selections {
                if prefix == line[..prefix.len()] {
                    if line.chars().nth(char_position).unwrap() == '0' {
                        zero_counts[char_position] += 1;
                    }
                    if line.chars().nth(char_position).unwrap() == '1' {
                        one_counts[char_position] += 1;
                    }
                }
            }
            if (one_counts[char_position] == 1 && zero_counts[char_position] == 1) ||
                (one_counts[char_position] == 0 && zero_counts[char_position] == 1) ||
                (one_counts[char_position] == 1 && zero_counts[char_position] == 0) {
                prefix.push_str(selections.get(0).unwrap().chars().nth(char_position).unwrap().to_string().as_str());
            } else if one_counts[char_position] >= zero_counts[char_position] {
                prefix.push('1');
            } else if zero_counts[char_position] > one_counts[char_position] {
                prefix.push('0');
            }
            selections = vec![];
            for line in input {
                if prefix == line[..prefix.len()] {
                    selections.push(line.to_string());
                }
            }
        }
        o2_rating = prefix;
        // println!("Zero Counts: {:?}, One Counts: {:?}.", zero_counts, one_counts);
    }
    {
        let mut prefix = "".to_string();
        // Counting 0s.
        let mut zero_counts: Vec<i64> = vec![0; 12];
        // Counting 1s.
        let mut one_counts: Vec<i64> = vec![0; 12];
        let mut selections: Vec<String> = Vec::from(input);
        // Index of each position in binary numbers.
        for char_position in 0..12 {
            // println!("Iteration {}, Selections: {:?}, Prefix: {}.", char_position + 1, selections, prefix);
            // println!("Zero Counts: {:?}, One Counts: {:?}.", zero_counts, one_counts);
            // Loop through each lines.
            for line in &mut *selections {
                if prefix == line[..prefix.len()] {
                    if line.chars().nth(char_position).unwrap() == '0' {
                        zero_counts[char_position] += 1;
                    }
                    if line.chars().nth(char_position).unwrap() == '1' {
                        one_counts[char_position] += 1;
                    }
                }
            }
            if (one_counts[char_position] == 1 && zero_counts[char_position] == 1) ||
                (one_counts[char_position] == 0 && zero_counts[char_position] == 1) ||
                (one_counts[char_position] == 1 && zero_counts[char_position] == 0) {
                prefix.push_str(selections.get(0).unwrap().chars().nth(char_position).unwrap().to_string().as_str());
            } else if one_counts[char_position] >= zero_counts[char_position] {
                prefix.push('0');
            } else if zero_counts[char_position] > one_counts[char_position] {
                prefix.push('1');
            }
            selections = vec![];
            for line in input {
                if prefix == line[..prefix.len()] {
                    selections.push(line.to_string());
                }
            }
        }
        co2_rating = prefix;
        // println!("Zero Counts: {:?}, One Counts: {:?}.", zero_counts, one_counts);
    }
    i64::from_str_radix(o2_rating.as_str(), 2).unwrap() * i64::from_str_radix(co2_rating.as_str(), 2).unwrap()
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
