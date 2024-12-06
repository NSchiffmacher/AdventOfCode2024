use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

type T = i32;
pub struct Solution {
    blocked: HashSet<(T, T)>
    start_pos: (T, T),
}

pub enum Direction { Up, Down, Left, Right }
impl Direction {
    pub fn to_delta(&self) -> (T, T) {
        match &self {
            Direction::Up => 
        }
    }
}

impl Solution {
    pub fn init() -> Self {
        let mut blocked = HashSet::new();
        let mut start_pos = None;
        for (y, line) in read_to_string("inputs/day6.txt").unwrap().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    blocked.insert((x as T, y as T));
                } else if c == '^' {
                    start_pos = Some((x as T, y as T));
                }
            }
        }

        Self {
            blocked,
            start_pos: start_pos.unwrap(),
        }
    }

    fn part1(&mut self) {
        let mut pos = self.start_pos.clone();
        let mut direction = Direction::UP;

    }

    fn part2(&mut self) {

    }

    pub fn solve(&mut self) {
        println!("========= DAY 6 ========");
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
