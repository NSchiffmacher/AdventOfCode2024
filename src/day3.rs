use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::sync::LazyLock;

#[allow(unused_imports)]
use itertools::Itertools;
use regex::Regex;

pub struct Solution {
    input: String,
}

type T = i128;
static MUL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"mul\((\d+),(\d+)\)").unwrap()
});
static DO_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"do()").unwrap()
});
static DONT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"don't()").unwrap()
});

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day3.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            input: lines.join(""),
        }
    }

    fn part1(&mut self) -> T {
        MUL_RE
            .captures_iter(&self.input)
            .map(|x| x.get(1).unwrap().as_str().parse::<T>().unwrap() * x.get(2).unwrap().as_str().parse::<T>().unwrap())
            .sum()
    }

    fn part2(&mut self) {
        let do_indices: BTreeSet<_> = DO_RE.find_iter(&self.input).map(|x| x.end()).collect();
        let dont_indices: BTreeSet<_> = DONT_RE.find_iter(&self.input).map(|x| x.end()).collect();

        let res = 0;
        for m in MUL_RE.captures_iter(&self.input) {
            let start = m.get(0).unwrap().start();
            let closest_do = do_indices.range(0..start).next_back().unwrap_or(&1);
            let closest_dont = dont_indices.range(0..start).next_back().unwrap_or(&0);
            println!("Closest do: {:?}", closest_do);
            println!("closest_dont do: {:?}", closest_dont);
        }

        println!("Do  : {:?}", do_indices);
        println!("Dont: {:?}", dont_indices);
    }

    pub fn solve(&mut self) {
        println!("========= DAY 3 ========");
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