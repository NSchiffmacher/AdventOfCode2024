use std::fs::read_to_string;
use std::io::{self, Write};
use std::collections::HashSet;

#[allow(unused_imports)]
use itertools::Itertools;

type Cell = u8;
type Index = isize;
pub struct Solution {
    map: Vec<Vec<Cell>>,
    starting_positions: Vec<(Index, Index)>,
    width: Index,
    height: Index,
}

impl Solution {
    pub fn init() -> Self {
        let mut map = Vec::new();
        let mut starting_positions = Vec::new();
        for (y, line) in read_to_string("inputs/day10.txt").unwrap().lines().enumerate() {
            let mut row = vec![];
            for (x, c) in line.chars().enumerate() {
                let height = c.to_digit(10).unwrap() as Cell;
                row.push(height);
                
                if height == 0 {
                    starting_positions.push((x as Index, y as Index));
                }
            }
            map.push(row);
        }

        Self {
            width: map[0].len() as Index,
            height: map.len() as Index,
            map,
            starting_positions,
        }
    }

    fn eval_trailhead(&self, from: (Index, Index), compute_rating: bool) -> usize {
        // Return the number of goals reached (need to ensure they are different)
        let mut queue = vec![(from, 0)];
        let mut reached_goals = HashSet::new();
        let mut res = 0;

        while let Some(((x, y), height)) = queue.pop() {
            if self.map[y as usize][x as usize] == 9 {
                reached_goals.insert((x, y));
                res += 1;
                continue;
            }

            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height && self.map[ny as usize][nx as usize] == height + 1 {
                    queue.insert(0, ((nx, ny), self.map[ny as usize][nx as usize]));
                }
            }
        }

        if compute_rating {
            res
        } else {
            reached_goals.len()
        }
    }

    fn part1(&mut self) -> usize {
        self.starting_positions
            .iter()
            .map(|pos| self.eval_trailhead(pos.clone(), false))
            .sum()
    }

    fn part2(&mut self) -> usize {
        self.starting_positions
            .iter()
            .map(|pos| self.eval_trailhead(pos.clone(), true))
            .sum()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 10 ========");
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
