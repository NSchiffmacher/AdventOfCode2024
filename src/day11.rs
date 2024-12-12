use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
}

type T = i32;
type UT = u32; 

fn count_digits(num: T) -> UT {
    num.checked_ilog10().unwrap_or(0) + 1
}

fn apply_rule(num: T) -> Vec<T> {
    if num == 0 { return vec![1]; }

    let num_digits = count_digits(num);
    match num_digits {
        num_digits if num_digits % 2 == 0 => split_num(num),
        _ => vec![2024 * num],
    }
}

fn split_num(num: T) -> Vec<T> {
    let digits = count_digits(num);
    let denom = (10 as T).pow(digits / 2);
    let a = num / denom;
    let b = num % denom;

    vec![a, b]
}

fn blink(nums: &mut Vec<T>) {
    // let /C
}

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

    fn part1(&mut self) {

    }

    fn part2(&mut self) {

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
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(5), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(123), 3);
        assert_eq!(count_digits(1000), 4);
    }

    #[test]
    fn test_split_num() {
        assert_eq!(split_num(1234), vec![12, 34]);
        assert_eq!(split_num(5678), vec![56, 78]);
        assert_eq!(split_num(90), vec![9, 0]);
    }

    #[test]
    fn test_apply_rule() {
        assert_eq!(apply_rule(0), vec![1]);
        assert_eq!(apply_rule(1), vec![2024]);
        assert_eq!(apply_rule(10), vec![1, 0]);
        assert_eq!(apply_rule(99), vec![9, 9]);
        assert_eq!(apply_rule(999), vec![2021976]);
    }
}

