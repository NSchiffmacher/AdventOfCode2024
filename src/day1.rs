use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    list_a: Vec<i32>,
    list_b: Vec<i32>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day1.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let mut list_a: Vec<i32> = Vec::new();
        let mut list_b: Vec<i32> = Vec::new();

        for line in &lines {
            let (a, b) = line.split_once("   ").unwrap();
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();

            list_a.push(a);
            list_b.push(b);
        }

        list_a.sort_unstable();
        list_b.sort_unstable();

        Self {
            lines,
            list_a,
            list_b,
        }
    }

    fn part1(&mut self) -> i32 {
        self.list_a
            .iter()
            .zip(&self.list_b)
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    fn make_occurences_map(&self, list: &[i32]) -> HashMap<i32, i32> {
        list
            .iter()
            .copied()
            .fold(HashMap::new(), |mut map, e| {
                *map.entry(e).or_insert(0) += 1;
                map
            })
    }

    fn part2(&mut self) -> i32 {
        let occurences_a = self.make_occurences_map(&self.list_a);
        let occurences_b = self.make_occurences_map(&self.list_b);

        occurences_a
            .iter()
            .fold(0, |acc, (e, count)| {
                acc + e * count * occurences_b.get(e).unwrap_or(&0)
            })
    }

    pub fn solve(&mut self) {
        println!("========= DAY 1 ========");
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