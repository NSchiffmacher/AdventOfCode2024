use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

type T = i32;
pub struct Solution {
    blocked: HashSet<(T, T)>,
    start_pos: (T, T),
    size: (T, T),
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Direction { Up, Down, Left, Right }
impl Direction {
    pub fn to_delta(&self) -> (T, T) {
        match &self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    pub fn turn_right(&self) -> Self {
        match &self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

}

impl Solution {
    pub fn init() -> Self {
        let mut blocked = HashSet::new();
        let mut start_pos = None;
        let mut width = 0;
        let mut height = 0;
        for (y, line) in read_to_string("inputs/day6.txt").unwrap().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    blocked.insert((x as T, y as T));
                } else if c == '^' {
                    start_pos = Some((x as T, y as T));
                }
            }
            height += 1;
            width = line.len() as T;
        }

        Self {
            blocked,
            start_pos: start_pos.unwrap(),
            size: (width, height),
        }
    }

    fn travel(&self, blocked_map: &HashSet<(T, T)>) -> (HashSet<((T,T), Direction)>, bool) {
        let mut pos = self.start_pos.clone();
        let mut direction = Direction::Up;
        let (width, height) = self.size;
        let mut seen_states = HashSet::new();

        while pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
            if !seen_states.insert((pos, direction.clone())) {
                return (seen_states, true);
            }

            let delta = direction.to_delta();
            if blocked_map.contains(&(pos.0 + delta.0, pos.1 + delta.1)) {
                direction = direction.turn_right();
            } else {
                pos = (pos.0 + delta.0, pos.1 + delta.1);
            }
        }

        (seen_states, false)
    }

    fn part1(&mut self) -> usize {
        self
            .travel(&self.blocked)
            .0
            .iter()
            .map(|(pos, _dir)| pos)
            .collect::<HashSet<_>>()
            .len()
    }

    fn part2(&mut self) -> usize {
        let mut res = 0;
        let mut blocked = self.blocked.clone();
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let pos = (x, y);
                if !self.blocked.contains(&pos) && pos != self.start_pos {
                    blocked.insert(pos);
                    if self.travel(&blocked).1 {
                        res += 1;
                    }
                    blocked.remove(&pos);
                }
            }
        }
        res
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
