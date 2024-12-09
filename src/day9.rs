use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

type T = i128;
pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day9.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    fn part1(&mut self) -> T {
        // Parsing
        let mut free_spaces = BTreeSet::new(); // Want smallest first
        let mut used_memory = BTreeMap::new(); // Want biggest first
        let mut current_index = 0;
        for (i, c) in self.lines[0].chars().enumerate() {
            let total_offset = c.to_digit(10).unwrap() as T;
            if i % 2 == 0 {
                // Used memory
                let id = (i / 2) as T;
                for offset in 0..total_offset {
                    used_memory.insert(-current_index - offset, id);
                }
            } else {
                for offset in 0..total_offset {
                    free_spaces.insert(current_index + offset);
                }
            }
            current_index += total_offset;
        }

        // Moving the blocks
        while *free_spaces.first().unwrap() <= -used_memory.first_key_value().unwrap().0 {
            let (neg_address, id) = used_memory.pop_first().unwrap();
            let dest_address = free_spaces.pop_first().unwrap();

            used_memory.insert(-dest_address, id);
            free_spaces.insert(-neg_address);
        }

        // Computing the checksum
        let mut checksum = 0;
        for (neg_address, id) in used_memory {
            checksum -= neg_address * id;
        }

        checksum
    }

    fn find_spot(free_spaces: &BTreeMap<T, T>, size: T) -> Option<(T, T)> {
        for (space_index, space_size) in free_spaces {
            if *space_size >= size {
                return Some((*space_index, *space_size));
            }
        }

        None
    }

    fn print_occupied(used_memory: &Vec<(T, T)>) -> String {
        let max_size = used_memory.iter().map(|(_ad, size)| size).max().unwrap();
        let max_ad = used_memory.iter().map(|(ad, _size)| ad).max().unwrap();
        let mut res = vec!['.'; (*max_ad + *max_size) as usize];

        for (id, (ad, size)) in used_memory.iter().enumerate() {
            for offset in 0..*size {
                res[(*ad + offset) as usize] = id.to_string().chars().next().unwrap();
            }
        }

        res.iter().collect()
    }

    fn part2(&mut self) -> T {
        // Parsing
        let mut free_spaces = BTreeMap::new(); // Map of index -> given size
        let mut used_memory = vec![]; // Id = index  -> (address, size)
        let mut current_index = 0;
        for (i, c) in self.lines[0].chars().enumerate() {
            let total_offset = c.to_digit(10).unwrap() as T;
            if i % 2 == 0 {
                // Used memory
                used_memory.push((current_index, total_offset));
            } else {
                free_spaces.insert(current_index, total_offset);
            }
            current_index += total_offset;
        }

        // Moving the blocks by order of id
        for id in (0..used_memory.len()).rev() {
            let (address, size) = used_memory[id];
            // Find the first place where the block can fit
            if let Some((available_spot, spot_size)) = Self::find_spot(&free_spaces, size) {
                if available_spot < address {
                    // Delete the spot from the available spots
                    free_spaces.remove(&available_spot).unwrap();
                    if spot_size >= size {
                        // Need to break the available spot in two
                        free_spaces.insert(available_spot + size, spot_size - size);
                    }

                    // Add a free spot at the old address
                    let free_spot_size = if let Some(add_size)  = free_spaces.remove(&(address + size)) {
                        size + add_size
                    } else {
                        size
                    };

                    free_spaces.insert(address, free_spot_size);

                    // Update the new address
                    used_memory[id] = (available_spot, size);
                }
            }
        }

        // Compute the checksum
        let mut checksum = 0;
        for (id, (address, size)) in used_memory.iter().enumerate() {
            for x in *address..*address+*size {
                checksum += x * (id as T);
            }
        }

        checksum
    }

    pub fn solve(&mut self) {
        println!("========= DAY 9 ========");
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
