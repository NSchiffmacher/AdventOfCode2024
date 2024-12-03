use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

type T = i32; 

pub struct Solution {
    lines: Vec<String>,
    levels: Vec<Vec<T>>,
    res_1: usize,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day2.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let levels = lines
            .iter()
            .map(|s| s.split_whitespace().map(|token| token.parse::<T>().unwrap()).collect())
            .collect();

        Self {
            lines,
            levels,
            res_1: 0,
        }
    }

    fn is_safe(level: &[T]) -> bool {
        let diff = (level[1] - level[0]).signum();
        diff != 0 && level
            .iter()
            .tuple_windows()
            .all(|(x, y)| (y - x).signum() == diff && (y - x).abs() <= 3)
    }

    fn part1(&mut self) -> usize {
        let res = self.levels
            .iter()
            .map(|level| Self::is_safe(level))
            .filter(|x| *x)
            .count();
        self.res_1 = res;
        res
    }

    fn part2(&mut self) -> usize {
        let invalid_list: Vec<_> = self.levels
            .iter()
            .filter(|level| !Self::is_safe(level))
            .cloned()
            .collect();
        let mut res_2 = 0;
        
        for invalid in invalid_list {
            for i in 0..invalid.len() {
                let mut invalid = invalid.clone();
                invalid.remove(i);

                if Self::is_safe(&invalid) {
                    res_2 += 1;
                    break;
                }
            }
        }
        
        self.res_1 + res_2
    }

    pub fn solve(&mut self) {
        println!("========= DAY 2 ========");
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