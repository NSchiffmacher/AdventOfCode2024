use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<Vec<char>>,
}

static WILDCARD: char = '.'; 

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

    fn convolve_once((sx, sy): (usize, usize), pattern: &Vec<Vec<char>>, array: &Vec<Vec<char>>) -> bool {
        let padding_size = pattern.len(); // assume square pattern

        for i in 0..padding_size {
            for j in 0..padding_size {
                let x = sx + i;
                let y = sy + j;
                if pattern[j][i] != WILDCARD && pattern[j][i] != array[y][x] {
                    return false;
                }
            }
        }

        true
    }

    fn convolve(array: &Vec<Vec<char>>, patterns: &Vec<Vec<Vec<char>>>, with_padding: bool) -> usize {
        let pattern_size = patterns[0].len(); // Assume square pattern

        // Pad the array if necessary
        let array = if with_padding {
            Self::pad_array(array, WILDCARD, pattern_size)
        } else {
            array.clone()
        }; 

        // Apply the convolution
        let mut result = 0;
        for y in 0..=array.len() - pattern_size{
            for x in 0..=array[0].len() - pattern_size {
                for pattern in patterns {
                    if Self::convolve_once((x, y), pattern, &array) {
                        result += 1;
                    }
                }
            }
        }

        result
    }

    fn pad_array(array: &Vec<Vec<char>>, pad_char: char, pad_size: usize) -> Vec<Vec<char>>{
        let width = array[0].len();
        let line_pad = vec![pad_char; width + 2 * pad_size];
        let side_pad = vec![pad_char; pad_size];

        let mut new_array = vec![line_pad.clone(); pad_size];
        for line in array {
            new_array.push(
                side_pad.iter().chain(line).chain(side_pad.iter()).cloned().collect()
            );
        }

        for _ in 0..pad_size {
            new_array.push(line_pad.clone());
        }

        new_array
    }

    fn part1(&mut self) -> usize {
        let w = WILDCARD;
        let patterns = vec![
            vec![vec![ w,   w,   w,   w ],
                 vec!['X', 'M', 'A', 'S'],
                 vec![ w,   w,   w,   w ],
                 vec![ w,   w,   w,   w ]],
            vec![vec![ w,   w,   w,   w ],
                 vec!['S', 'A', 'M', 'X'],
                 vec![ w,   w,   w,   w ],
                 vec![ w,   w,   w,   w ]],
            vec![vec![ w,  'X',   w,   w ],
                 vec![ w,  'M',   w,   w ],
                 vec![ w,  'A',   w,   w ],
                 vec![ w,  'S',   w,   w ]],
            vec![vec![ w,  'S',   w,   w ],
                 vec![ w,  'A',   w,   w ],
                 vec![ w,  'M',   w,   w ],
                 vec![ w,  'X',   w,   w ]],
            vec![vec!['X',  w,    w,   w ],
                 vec![ w,  'M',   w,   w ],
                 vec![ w,   w,   'A',  w ],
                 vec![ w,   w,    w,  'S' ]],
            vec![vec!['S',  w,    w,   w ],
                 vec![ w,  'A',   w,   w ],
                 vec![ w,   w,   'M',  w ],
                 vec![ w,   w,    w,  'X']],
            vec![vec![ w,   w,    w,  'S'],
                 vec![ w,   w,   'A',  w ],
                 vec![ w,  'M',   w,   w ],
                 vec!['X',  w,    w,   w ]],
            vec![vec![ w,   w,    w,  'X'],
                 vec![ w,   w,   'M',  w ],
                 vec![ w,  'A',   w,   w ],
                 vec!['S',  w,    w,   w ]],
        ];

        Self::convolve(&self.lines, &patterns, true)
    }

    fn part2(&mut self) -> usize {
        let w = WILDCARD;
        let patterns = vec![
            vec![vec!['S', w, 'S'],
                 vec![ w, 'A', w],
                 vec!['M', w, 'M']],
            vec![vec!['M', w, 'S'],
                 vec![ w, 'A', w],
                 vec!['M', w, 'S']],
            vec![vec!['S', w, 'M'],
                 vec![ w, 'A', w],
                 vec!['S', w, 'M']],
            vec![vec!['M', w, 'M'],
                 vec![ w, 'A', w],
                 vec!['S', w, 'S']],
        ];

        Self::convolve(&self.lines, &patterns, false)
    }

    pub fn solve(&mut self) {
        println!("========= DAY 4 ========");
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
