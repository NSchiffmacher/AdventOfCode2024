use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

type T = u128;
pub struct Solution {
    lines: Vec<String>,
    equations: Vec<(T, Vec<T>)>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        let mut equations = vec![];
        for line in read_to_string("inputs/day7.txt").unwrap().lines() {
            let line = line.to_string();
            lines.push(line.clone());
            
            let (result_str, operands_str) = line.split_once(": ").unwrap();
            let result = result_str.parse().unwrap();
            let operands = operands_str.split_whitespace().map(|x| x.parse().unwrap()).collect();

            equations.push((result, operands));
        }

        Self {
            lines,
            equations,
        }
    }

    fn concat_int(a: T, b: T) -> T {
        let num_digits = b.checked_ilog10().unwrap_or(0) + 1;
        a * (10 as T).pow(num_digits) + b
    }

    fn try_evaluate(expected_result: T, operands: &[T], accumulator: T, part2: bool) -> bool {
        match operands.split_first() {
            None => { accumulator == expected_result },
            Some((head, tail)) => { 
                (accumulator <= expected_result) &&
                (Self::try_evaluate(expected_result, tail, accumulator + head, part2) || 
                Self::try_evaluate(expected_result, tail, accumulator * head, part2) || 
                (part2 && Self::try_evaluate(expected_result, tail, Self::concat_int(accumulator, *head), part2)))
            }
        }
    }

    fn part1(&mut self) -> T {
        self.equations
            .iter()
            .filter(|(result, operands)| {
                let (head, tail) = operands.split_first().unwrap();
                Self::try_evaluate(*result, tail, *head, false)
            })
            .map(|(result, _operands)| result)
            .sum()
    }

    fn part2(&mut self) -> T {
        self.equations
            .iter()
            .filter(|(result, operands)| {
                let (head, tail) = operands.split_first().unwrap();
                Self::try_evaluate(*result, tail, *head, true)
            })
            .map(|(result, _operands)| result)
            .sum()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 7 ========");
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
    fn test_concat_int() {
        assert_eq!(Solution::concat_int(123, 456), 123456);
        assert_eq!(Solution::concat_int(123, 1), 1231);
        assert_eq!(Solution::concat_int(0, 123), 123);
    }
}