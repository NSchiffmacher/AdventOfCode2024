use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

type Id = usize;
type Coord = isize;

pub struct Solution {
    raw_map: Vec<Vec<char>>,
    map: Vec<Vec<Id>>,
    width: Coord,
    height: Coord,
}

impl Solution {
    pub fn init() -> Self {
        let mut raw_map = Vec::new();
        let mut map = Vec::new();
        for line in read_to_string("inputs/day12.txt").unwrap().lines() {
            let chars: Vec<_> = line.chars().collect();
            map.push(vec![0; chars.len()]);
            raw_map.push(chars);
        }

        Self {
            width: map[0].len() as Coord,
            height: map.len() as Coord,
            raw_map,
            map,
        }
    }

    fn part1(&mut self) -> usize {
        let mut current_id = 0;
        let mut regions = Vec::new();
        let mut to_visit = HashSet::new();
        for (x, y) in (0..self.width).cartesian_product(0..self.height) {
            to_visit.insert((x, y));
        }

        while !to_visit.is_empty() {
            let base_pos = to_visit.iter().next().unwrap().clone();
            to_visit.remove(&base_pos);

            let mut queue = vec![base_pos];
            let mut region = vec![base_pos];
            let region_char = self.raw_map[base_pos.1 as usize][base_pos.0 as usize];
            self.map[base_pos.1 as usize][base_pos.0 as usize] = current_id;
            while let Some((x, y)) = queue.pop() {
                for (nx, ny) in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)] {
                    if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height && self.raw_map[ny as usize][nx as usize] == region_char && to_visit.contains(&(nx, ny)) {
                        to_visit.remove(&(nx, ny));
                        region.push((nx, ny));
                        queue.push((nx, ny));
                        self.map[ny as usize][nx as usize] = current_id;
                    }
                }
            }

            regions.push(region);
            current_id += 1;
        }

        // println!("{:?}", self.raw_map);
        // println!("{:?}", self.map);

        let mut result = 0;
        for (id, region) in regions.into_iter().enumerate() {
            let area = region.len();
            let mut perimeter = 0;
            for (x, y) in region {
                for (nx, ny) in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)] {
                    if nx < 0 || nx >= self.width || ny < 0 || ny >= self.height || self.map[ny as usize][nx as usize] != id {
                        perimeter += 1;
                    }
                }
            }

            // let first = region[0];
            // let c = self.raw_map[first.1 as usize][first.0 as usize];
            // println!("Char: {:?} Region: {:?} Area: {:?} Perimeter {:?}", c, region, area, perimeter);

            result += perimeter * area;
        }
        result
    }

    fn part2(&mut self) -> usize {
        let mut current_id = 0;
        let mut regions = Vec::new();
        let mut to_visit = HashSet::new();
        for (x, y) in (0..self.width).cartesian_product(0..self.height) {
            to_visit.insert((x, y));
        }

        while !to_visit.is_empty() {
            let base_pos = to_visit.iter().next().unwrap().clone();
            to_visit.remove(&base_pos);

            let mut queue = vec![base_pos];
            let mut region = vec![base_pos];
            let region_char = self.raw_map[base_pos.1 as usize][base_pos.0 as usize];
            self.map[base_pos.1 as usize][base_pos.0 as usize] = current_id;
            while let Some((x, y)) = queue.pop() {
                for (nx, ny) in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)] {
                    if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height && self.raw_map[ny as usize][nx as usize] == region_char && to_visit.contains(&(nx, ny)) {
                        to_visit.remove(&(nx, ny));
                        region.push((nx, ny));
                        queue.push((nx, ny));
                        self.map[ny as usize][nx as usize] = current_id;
                    }
                }
            }

            regions.push(region);
            current_id += 1;
        }

        // println!("{:?}", self.raw_map);
        // println!("{:?}", self.map);

        let mut result = 0;
        for (id, region) in regions.into_iter().enumerate() {
            let area = region.len();
            let mut edge = HashSet::new();
            for (x, y) in region.clone() { // debug
                let mut is_in = true;
                for (nx, ny) in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)] {
                    if !(nx < 0 || nx >= self.width || ny < 0 || ny >= self.height || self.map[ny as usize][nx as usize] != id) {
                        is_in = false;
                    }
                }

                if !is_in {
                    edge.insert((x, y));
                }
            }

            // Count the number of sides
            // Traverse in one direction 
            // Bordel

            // let first = region[0];
            // let c = self.raw_map[first.1 as usize][first.0 as usize];
            // println!("Char: {:?} Region: {:?} Area: {:?} edge {:?}", c, region, area, edge);

            result += 1 * area;
        }
        result
    }

    pub fn solve(&mut self) {
        println!("========= DAY 12 ========");
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