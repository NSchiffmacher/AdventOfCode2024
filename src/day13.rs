use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self, Write};
use regex::Regex;
use std::sync::LazyLock;

#[allow(unused_imports)]
use itertools::Itertools;

static BUTTON_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap());
static PRIZE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap());

type T = i128;

#[derive(Debug, Clone)]
pub struct Game {
    a_dx: T,
    a_dy: T,
    b_dx: T,
    b_dy: T,
    p_x: T,
    p_y: T,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let lines: Vec<_> = value.lines().collect();
        let a_dx = BUTTON_RE.captures(lines[0]).unwrap().get(1).unwrap().as_str().parse().unwrap();
        let a_dy = BUTTON_RE.captures(lines[0]).unwrap().get(2).unwrap().as_str().parse().unwrap();
        let b_dx = BUTTON_RE.captures(lines[1]).unwrap().get(1).unwrap().as_str().parse().unwrap();
        let b_dy = BUTTON_RE.captures(lines[1]).unwrap().get(2).unwrap().as_str().parse().unwrap();
        let p_x = PRIZE_RE.captures(lines[2]).unwrap().get(1).unwrap().as_str().parse().unwrap();
        let p_y = PRIZE_RE.captures(lines[2]).unwrap().get(2).unwrap().as_str().parse().unwrap();

        Self {
            a_dx,
            a_dy,
            b_dx,
            b_dy,
            p_x,
            p_y,
        }
    }
}

impl Game {
    fn cheapest_cost(&self) -> Option<T> {
        let nb = (self.p_x * self.a_dy - self.p_y * self.a_dx) as f64 / ((self.b_dx * self.a_dy - self.b_dy * self.a_dx) as f64);
        let na = (self.p_x as f64 - nb * self.b_dx as f64) / (self.a_dx as f64);

        if na < 0. || nb < 0. || na.fract() != 0. || nb.fract() != 0. {
            return None;
        }

        Some((na * 3. + nb).round() as T)
    }

    fn translated(&self) -> Self {
        let mut new = self.clone();
        new.p_x += 10000000000000;
        new.p_y += 10000000000000;
        new
    }
}

pub struct Solution {
    games: Vec<Game>,
}

impl Solution {
    pub fn init() -> Self {
        let inp = read_to_string("inputs/day13.txt").unwrap();
        let games = inp.split("\r\n\r\n").map(|x| Game::from(x)).collect();

        Self {
            games,
        }
    }

    fn part1(&mut self) -> T {
        self.games.iter().map(|g| g.cheapest_cost().unwrap_or(0)).sum::<T>()
    }

    fn part2(&mut self) -> T {
        self.games.iter().map(|g| g.translated().cheapest_cost().unwrap_or(0)).sum::<T>()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 13 ========");
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