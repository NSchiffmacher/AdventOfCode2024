use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

type T = isize;
pub struct Solution {
    lines: Vec<String>,
    map: HashMap<(T, T), usize>,
}

const SIZE: T = 71;

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day18.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        // Parse the map
        let mut map = HashMap::new();
        for (step, line) in lines.iter().enumerate() {
            let (a, b) = line.split_once(',').unwrap();
            let a = a.parse::<T>().unwrap();
            let b = b.parse::<T>().unwrap();
            map.insert((a, b), step);
        }

        Self {
            lines,
            map,
        }
    }

    pub fn print_map(&self, map: &HashSet<(T, T)>) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                if map.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn get_neighbors_before_step(&self, cur_step: usize, (x, y): (T, T), map: &HashMap<(T, T), usize>) -> Vec<(T, T)> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < SIZE - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < SIZE - 1 {
            neighbors.push((x, y + 1));
        }

        // Can go if it's not in the map, or if the step is higher than the current
        neighbors.into_iter().filter(|n| *map.get(&n).unwrap_or(&usize::MAX) > cur_step).collect()
    }

    fn path_distance_before_step(&self, cur_step: usize, map: &HashMap<(T, T), usize>) -> T {
        // Define the heuristic (distance to size-1, size-1)
        let start = (0, 0);
        let goal = (SIZE - 1, SIZE - 1);
        let heuristic = |(x, y): (T, T)| -> T {
            (goal.0 - x).abs() + (goal.1 - y).abs()
        };

        // Find the shortest path
        let mut open_set = BTreeSet::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();

        open_set.insert((heuristic(start), start));
        g_score.insert(start, 0);
        f_score.insert(start, heuristic(start));

        while let Some((dist, current)) = open_set.pop_first() {
            if current == goal {
                return dist;
            }

            for neighbor in self.get_neighbors_before_step(cur_step, current, &map) {
                let new_gscore = g_score.get(&current).unwrap_or(&(SIZE * SIZE - 1)) + 1;
                if new_gscore < *g_score.get(&neighbor).unwrap_or(&(SIZE * SIZE)) {
                    g_score.insert(neighbor, new_gscore);
                    f_score.insert(neighbor, new_gscore + heuristic(neighbor));
                    open_set.insert((f_score[&neighbor], neighbor));
                }
            }
        }

        -1
    }

    fn part1(&mut self) -> T {
        self.path_distance_before_step(1023, &self.map)
    }

    fn part2(&mut self) -> String {
        // After which step is the path blocked (search in reverse)
        for step in (1024..self.lines.len()).rev() {
            // Check if the path is blocked
            if self.path_distance_before_step(step, &self.map) != -1 {
                // Find the coordinates for the step
                return self.lines[step+1].clone();
            }
        }

        "oups".to_string()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 18 ========");
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
        println!("{} (took {:?})", part2, part2_time);
        println!();
    }
}