use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
}

type T = i64;

// fn update_stones(nums: &mut Vec<T>) {
//     let mut n = 0;
//     while n < nums.len() {
//         let new = apply_rule(nums[n]);
//         match new.len() {
//             1 => nums[n] = new[0],
//             2 => { nums[n] = new[1]; nums.insert(n, new[0]); n += 1; },
//             _ => panic!("??"),
//         }
//         n += 1;
//     }
// }

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day11.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    fn count_digits(num: T) -> u32 {
        num.checked_ilog10().unwrap_or(0) + 1
    }
    
    fn apply_rule(num: T) -> Vec<T> {
        if num == 0 { return vec![1]; }
    
        let num_digits = Self::count_digits(num);
        match num_digits {
            num_digits if num_digits % 2 == 0 => Self::split_num(num),
            _ => vec![2024 * num],
        }
    }
    
    fn split_num(num: T) -> Vec<T> {
        let digits = Self::count_digits(num);
        let denom = (10 as T).pow(digits / 2);
        let a = num / denom;
        let b = num % denom;
    
        vec![a, b]
    }

    fn count_stones(num: T, remaining_blinks: usize, mut cache: &mut HashMap<(usize, T), usize>) -> usize {
        // End case
        if remaining_blinks == 0 {
            return 1;
        }

        // Cache
        if let Some(res) = cache.get(&(remaining_blinks, num)) {
            return *res;
        }

        // Compute
        let next_nums = Self::apply_rule(num);
        next_nums.iter().map(|n| {
            let res = Self::count_stones(*n, remaining_blinks - 1, &mut cache);
            cache.insert((remaining_blinks - 1, *n), res);
            res
        }).sum()
    }

    fn part1(&mut self) -> usize {
        let mut cache = HashMap::new();
        self
            .lines[0]
            .split_whitespace()
            .map(|x| x.parse::<T>().unwrap())
            .map(|num| Self::count_stones(num, 25, &mut cache))
            .sum()
    }

    fn part2(&mut self) -> usize {
        let mut cache = HashMap::new();
        self
            .lines[0]
            .split_whitespace()
            .map(|x| x.parse::<T>().unwrap())
            .map(|num| Self::count_stones(num, 75, &mut cache))
            .sum()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 11 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();

        let start = std::time::Instant::now();
        let part1 = self.part1();
        let part1_time = start.elapsed();
        println!("{:?} (took {:?})", part1, part1_time);

        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        let start = std::time::Instant::now();
        let part2 = self.part2();
        let part2_time = start.elapsed();
        println!("{:?} (took {:?})", part2, part2_time);
        println!();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(Solution::count_digits(0), 1);
        assert_eq!(Solution::count_digits(5), 1);
        assert_eq!(Solution::count_digits(10), 2);
        assert_eq!(Solution::count_digits(123), 3);
        assert_eq!(Solution::count_digits(1000), 4);
    }

    #[test]
    fn test_split_num() {
        assert_eq!(Solution::split_num(1234), vec![12, 34]);
        assert_eq!(Solution::split_num(5678), vec![56, 78]);
        assert_eq!(Solution::split_num(90), vec![9, 0]);
    }

    #[test]
    fn test_apply_rule() {
        assert_eq!(Solution::apply_rule(0), vec![1]);
        assert_eq!(Solution::apply_rule(1), vec![2024]);
        assert_eq!(Solution::apply_rule(10), vec![1, 0]);
        assert_eq!(Solution::apply_rule(99), vec![9, 9]);
        assert_eq!(Solution::apply_rule(999), vec![2021976]);
    }
}

