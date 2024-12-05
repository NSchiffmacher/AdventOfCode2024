use std::collections::{HashSet, HashMap};
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

type T = i32;

pub struct Solution {
    rules: HashSet<(T, T)> ,
    updates: Vec<Vec<T>>,
}

impl Solution {
    pub fn init() -> Self {
        let file_str = read_to_string("inputs/day5.txt").unwrap();
        let (rules_str, updates_str) = file_str.split_once("\n\n").unwrap();
        let rules = rules_str
            .lines()
            .map(|line| line.split_once('|').unwrap())
            .map(|(x_str, y_str)| (x_str.parse::<T>().unwrap(), y_str.parse::<T>().unwrap()))
            .collect();

        let updates = updates_str
            .lines()
            .map(|line| line.split(',').map(|x_str| x_str.parse::<T>().unwrap()).collect())
            .collect();

        Self {
            rules,
            updates,
        }
    }

    fn is_valid(&self, update: &Vec<T>) -> bool {
        let update_map: HashMap<T, usize> = update.iter().enumerate().map(|(index, e)| (*e, index)).collect();
        self.rules
            .iter()
            .filter(|(x, y)| update_map.contains_key(x) && update_map.contains_key(y))
            .all(|(x, y)| update_map.get(x).unwrap() < update_map.get(y).unwrap())
    }

    fn part1(&mut self) -> T {
        self.updates
            .iter()
            .filter(|update| self.is_valid(update))
            .map(|update| update[update.len() / 2])
            .sum()
    }
    
    fn get_middle_element(&self, update: &Vec<T>) -> T {
        let update_set: HashSet<T> = update.iter().cloned().collect();
        let applying_rules: HashSet<(T, T)> = self.rules
            .iter()
            .cloned()
            .filter(|(x, y)| update_set.contains(x) && update_set.contains(y))
            .collect();
        let mut occurences_first_place: HashMap<T, usize> = HashMap::new();
        for rule in &applying_rules {
            *occurences_first_place.entry(rule.0).or_insert(0) += 1;
        }
        *occurences_first_place.iter().find(|(_e, occ)| **occ ==  update.len() / 2).unwrap().0
    }

    fn part2(&mut self) -> T {
        self.updates
            .iter()
            .filter(|update| !self.is_valid(update))
            .map(|update| self.get_middle_element(update))
            .sum()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 5 ========");
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
