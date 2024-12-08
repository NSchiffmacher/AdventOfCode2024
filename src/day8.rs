use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

type T = i32;
pub struct Solution {
    antennas: HashMap<char, Vec<(T, T)>>,
    size: (T, T),
}

impl Solution {
    pub fn init() -> Self {
        let mut antennas = HashMap::new();
        let mut width = None;
        let mut height = None;
        for (y, line) in read_to_string("inputs/day8.txt").unwrap().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_insert(Vec::new()).push((x as T, y as T));
                }
            }

            width = Some(line.len());
            height = Some(y+1);
        }

        Self {
            antennas,
            size: (width.unwrap() as T, height.unwrap() as T),
        }
    }

    fn part1(&mut self) -> usize {
        let mut antinodes = HashSet::new();
        for (_id, antennas) in &self.antennas {
            for (i, antenna_a) in antennas.iter().enumerate() {
                for antenna_b in antennas[i+1..].iter() {
                    let dx = antenna_b.0 - antenna_a.0;
                    let dy = antenna_b.1 - antenna_a.1;
                    
                    let positions = vec![
                        (antenna_a.0 - dx, antenna_a.1 - dy),
                        (antenna_b.0 + dx, antenna_b.1 + dy),
                    ];
                    for pos in positions {
                        if pos.0 >= 0 && pos.0 < self.size.0 && pos.1 >= 0 && pos.1 < self.size.1 {
                            antinodes.insert(pos);
                        }
                    }
                }
            }
        }

        antinodes.len()
    }

    fn part2(&mut self) -> usize {
        let mut antinodes = HashSet::new();
        for (_id, antennas) in &self.antennas {
            for (i, antenna_a) in antennas.iter().enumerate() {
                for antenna_b in antennas[i+1..].iter() {
                    let dx = antenna_b.0 - antenna_a.0;
                    let dy = antenna_b.1 - antenna_a.1;
                    
                    let mut pos = antenna_a.clone();
                    while pos.0 >= 0 && pos.0 < self.size.0 && pos.1 >= 0 && pos.1 < self.size.1 {
                        antinodes.insert(pos);
                        pos = (pos.0 - dx, pos.1 - dy);
                    }
                    
                    let mut pos = antenna_b.clone();
                    while pos.0 >= 0 && pos.0 < self.size.0 && pos.1 >= 0 && pos.1 < self.size.1 {
                        antinodes.insert(pos);
                        pos = (pos.0 + dx, pos.1 + dy);
                    }
                }
            }
        }

        antinodes.len()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 8 ========");
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