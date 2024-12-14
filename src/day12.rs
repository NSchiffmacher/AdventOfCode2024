use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

type Id = usize;
type Coord = isize;

// 859336 to low

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

            // Regions detection
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

        // Result computation
        let mut result = 0;
        for (id, region) in regions.into_iter().enumerate() {
            let area = region.len();

            // Construct the outside edge
            let mut outside_edge: HashSet<(isize, isize)> = HashSet::new();
            for (x, y) in region.clone() { // debug
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { //  (1, -1), (-1, 1), (1, -1), (1, 1)
                    let (nx, ny) = (x+dx, y+dy);
                    if nx < 0 || nx >= self.width || ny < 0 || ny >= self.height || self.map[ny as usize][nx as usize] != id {
                        let (xb, yb) = (2*x+1, 2*y+1); // x, y in "big" grid
                        for i in -1..=1 {
                            let nxb = xb + i * dy + dx;
                            let nyb = yb + i * dx + dy;

                            outside_edge.insert((nxb, nyb));
                        }
                    }
                }
            }


            // Count the number of corners, result = corners.len() ?
            // let mut corners = HashSet::new();
            let mut sides = 0;
            let mut corners = HashSet::new();
            for (x, y) in outside_edge.iter().cloned() {
                // Find all neighbor
                let mut deltas = vec![];
                for (nx, ny) in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)] {
                    if outside_edge.contains(&(nx, ny)) {
                        deltas.push((x - nx, y - ny));
                    }
                }

                
                if deltas.len() == 2 {
                    let a = deltas[0];
                    let b = deltas[1];
                    let neg_b = (-b.0, -b.1);
                    if a != neg_b {
                        // Not in a straighline => Corner? 
                        corners.insert((x, y));
                        sides += 1; 
                        // }
                    }
                } else if deltas.len() == 4 {
                    corners.insert((x, y));
                    sides += 2; 
                } else  {
                    println!("({}, {}) -> {:?}", x, y, deltas);
                    panic!("Possible ???");
                }
            }

            let first = region[0];
            let c = self.raw_map[first.1 as usize][first.0 as usize];
            println!("Char: {:?} Region: {:?} Area: {:?} sides {:?}/{}", c, 'X', area, corners.len(), sides);

            result += sides * area;
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
