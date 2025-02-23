use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    computer: Computer,
}

pub type T = i64;


#[derive(Debug)]
pub struct Computer {
    reg_a: T,
    reg_b: T,
    reg_c: T,

    program: Vec<T>,
    pc: usize,
    stdout: Vec<T>,
}

impl Computer {
    pub fn new(reg_a: T, program_str: &str) -> Self {
        let program = program_str
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        Self {
            reg_a,
            reg_b: 0,
            reg_c: 0,
            program,
            pc: 0,
            stdout: vec![],
        }
    }

    pub fn reset(&mut self, reg_a: T) {
        self.reg_a = reg_a;
        self.reg_b = 0;
        self.reg_c = 0;
        self.pc = 0;
        self.stdout = vec![];
    }

    fn get_combo(&self, c: T) -> T {
        match c {
            0..4 => c,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Unknown combo"),
        }
    }

    pub fn step(&mut self) {
        let instr = &self.program[self.pc];
        let arg = &self.program[self.pc + 1];
        match instr {
            0 => {
                let num = self.reg_a;
                let den = (2 as T).pow(self.get_combo(*arg).try_into().expect("Invalid combo (negative)"));
                self.reg_a = num / den;
                self.pc += 2;
            }
            1 => {
                self.reg_b = self.reg_b ^ (*arg);
                self.pc += 2;
            }
            2 => {
                self.reg_b = self.get_combo(*arg) % 8;
                self.pc += 2;
            }
            3 => {
                match self.reg_a {
                    0 => self.pc += 2,
                    _ => self.pc = (*arg).try_into().expect("Invalid jump (negative)"),
                }
            }
            4 => {
                self.reg_b = self.reg_b ^ self.reg_c;
                self.pc += 2;
            }
            5 => {
                self.stdout.push(self.get_combo(*arg) % 8);
                self.pc += 2;
            }
            6 => {
                let num = self.reg_a;
                let den = (2 as T).pow(self.get_combo(*arg).try_into().expect("Invalid combo (negative)"));
                self.reg_b = num / den;
                self.pc += 2;
            }
            7 => {
                let num = self.reg_a;
                let den = (2 as T).pow(self.get_combo(*arg).try_into().expect("Invalid combo (negative)"));
                self.reg_c = num / den;
                self.pc += 2;
            }
            _ => panic!("Unknown instruction"),
        }

    }

    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            self.step();
        }
    }
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day17.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let a = lines[0].split(':').nth(1).unwrap().trim().parse().unwrap();
        let program = lines[4].split(':').nth(1).unwrap().trim();
        let computer = Computer::new(a, program);

        Self {
            lines,
            computer,
        }
    }

    fn part1(&mut self) -> String {
        self.computer.run();
        self.computer.stdout.iter().join(",")
    }

    fn run_with_register(&mut self, reg_a: T) {
        self.computer.reset(reg_a);
        self.computer.run();
    }

    fn part2(&mut self) -> T {
        // Construct the program octal by octal
        let program = self.computer.program.clone();
        let n = program.len();
        let mut queue = BTreeSet::new();
        queue.insert(0);

        for i in 0..n {
            let subprogram: Vec<_> = (&program[n-i-1..]).iter().copied().collect();
            let mut next_queue = BTreeSet::new();
            for &value in &queue {
                for octal_unit in 0..8 {
                    self.run_with_register(value + octal_unit);
                    if self.computer.stdout == subprogram {
                        next_queue.insert((value + octal_unit) * 8);
                    }
                }
            }

            queue = next_queue;
        }

        queue.first().unwrap().clone() / 8
    }

    pub fn solve(&mut self) {
        println!("========= DAY 17 ========");
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