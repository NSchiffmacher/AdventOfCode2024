use std::arch::x86_64::_MM_MASK_INEXACT;
use std::fs::read_to_string;
use std::io::{self, Write};

use std::sync::LazyLock;
use regex::Regex;

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<Vec<char>>,
}

static XMAS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"XMAS").unwrap());
static SAMX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"SAMX").unwrap());

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day4.txt").unwrap().lines() {
            lines.push(line.chars().collect());
        }

        Self {
            lines,
        }
    }

    fn char_vec_to_string_vec(array: &Vec<Vec<char>>) -> Vec<String> {
        array
            .iter()
            .map(|chars| chars.into_iter().join(""))
            .collect()
    }
    
    fn generate_columns(array: &Vec<Vec<char>>) -> Vec<String> {
        // Assume that array is a square
        // In place matrix transposition
        let n = array.len();
        let mut out = vec![];

        for i in 0..n {
            let mut row = vec![];
            for j in 0..n {
                row.push(array[j][i]);
            }
            out.push(row);
        }

        Self::char_vec_to_string_vec(&out)
    }

    fn generate_diagonals(array: &Vec<Vec<char>>, antidiag: bool) -> Vec<String> {
        let mut out = vec![];
        let n = array.len();

        for diag in 0..2*n {
            let mut row = vec![];
            for i in 0..=diag {
                let j = diag - i;
                if i < n && j < n {
                    row.push(if antidiag { array[n-j-1][n-i-1] } else { array[j][i] });
                }
            }
            out.push(row);
        }

        Self::char_vec_to_string_vec(&out)
    }

    fn count_mas(array: &Vec<String>) -> usize {
        let mut res = 0;
        for line in array {
            res += XMAS_RE.find_iter(&line).count() + SAMX_RE.find_iter(&line).count();
        }

        res
    }

    fn part1(&mut self) -> usize {
        let rows = Self::char_vec_to_string_vec(&self.lines);
        let cols = Self::generate_columns(&self.lines);
        let diags = Self::generate_diagonals(&self.lines, false);
        let adiags = Self::generate_diagonals(&self.lines, true);
        
        Self::count_mas(&rows) + Self::count_mas(&cols) + Self::count_mas(&diags) + Self::count_mas(&adiags) 
    }

    fn part2(&mut self) {

    }

    pub fn solve(&mut self) {
        println!("========= DAY 4 bis ========");
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