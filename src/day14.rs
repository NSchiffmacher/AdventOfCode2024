use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self, Write};

use std::sync::LazyLock;
use regex::Regex;

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    robots: Vec<Robot>,
}

type T = i16;
const HEIGHT: T = 103; // 103 for the real input
const WIDTH: T = 101; // 101 for the real input
static LINE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap());

#[derive(Debug, Clone)]
struct Robot {
    x: T,
    y: T,
    vx: T,
    vy: T,
}

impl From<&str> for Robot {
    fn from(s: &str) -> Self {
        let caps = LINE_RE.captures(s).unwrap();
        Self {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
            vx: caps[3].parse().unwrap(),
            vy: caps[4].parse().unwrap(),
        }
    }
}

impl Robot {
    fn step(&mut self) {
        self.x = (self.x + self.vx).rem_euclid(WIDTH);
        self.y = (self.y + self.vy).rem_euclid(HEIGHT);
    }

    fn quadrant(&self) -> usize {
        if self.x < WIDTH / 2 && self.y < HEIGHT / 2 { // Top left
            0
        } else if self.x >= WIDTH / 2 + 1 && self.y < HEIGHT / 2 { // Top right
            1
        } else if self.x < WIDTH / 2 && self.y >= HEIGHT / 2 + 1 { // Bottom left
            2
        } else if self.x >= WIDTH / 2 + 1 && self.y >= HEIGHT / 2 + 1 { // Bottom right
            3
        } else {
            4
        }
    }
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        let mut robots = Vec::new();
        for line in read_to_string("inputs/day14.txt").unwrap().lines() {
            robots.push(Robot::from(line));
            lines.push(line.to_string());
        }

        Self {
            lines,
            robots,
        }
    }

    fn show_map(&self, robots: &Vec<Robot>) {
        let mut map_int = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
        let mut map_char = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];
        for robot in robots {
            map_int[robot.y as usize][robot.x as usize] += 1;
        }
        for (x, y) in (0..WIDTH).cartesian_product(0..HEIGHT) {
            if map_int[y as usize][x as usize] > 0 {
                map_char[y as usize][x as usize] = map_int[y as usize][x as usize].to_string().chars().next().unwrap();
            }
        }

        for row in map_char {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn part1(&mut self) -> usize {
        let mut robots = self.robots.clone();

        for _step in 0..100 {
            robots.iter_mut().for_each(|r| r.step());
        }

        // Get the quadrants
        let mut quadrants = vec![0; 5]; // Quandrant 4 is the one that is not in the other 4 (aka the center cells)
        for robot in &robots {
            quadrants[robot.quadrant()] += 1;
        }

        // Get the result
        quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
    }

    fn part2(&mut self) -> usize {
        // Assume that they form a christmas tree when they are on unique positions (might be luck with my input ?)
        let mut robots = self.robots.clone();
        let mut step = 1;

        loop {
            // Step the robots
            robots.iter_mut().for_each(|r| r.step());

            // Are they in unique positions ?
            if robots.iter().map(|r| (r.x, r.y)).collect::<HashSet<_>>().len() == robots.len() {
                break step;
            }

            step += 1;
        }
    }

    pub fn solve(&mut self) {
        println!("========= DAY 14 ========");
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