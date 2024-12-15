use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    map: Map,
    moves: Vec<Direction>,
}

type T = isize;

#[derive(Clone)]
struct Map {
    robot_pos: (T, T),
    map: Vec<Vec<char>>,
    width: T,
    height: T,
}

impl Map {
    fn print(&self) {
        let mut map = self.map.clone();
        map[self.robot_pos.1 as usize][self.robot_pos.0 as usize] = '@';
        for row in &map {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }

    fn move_robot(&mut self, cur_move: &Direction) -> bool {
        // Returns true if the robot moved

        let (dx, dy) = cur_move.to_delta();
        let (x, y) = self.robot_pos;
        let new_pos = (x + dx, y + dy);
        
        // Assume that it's always in bounds (because of the walls on the side)
        // Check if we hit a wall 
        if self.map[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            return false; // No move
        }

        // Check if the cell is free
        if self.map[new_pos.1 as usize][new_pos.0 as usize] == '.' {
            self.robot_pos = new_pos;
            return true; 
        }

        // There is a box, can we move it ?
        let mut free_space = (new_pos.0 + dx, new_pos.1 + dy);
        while self.map[free_space.1 as usize][free_space.0 as usize] == 'O' {
            free_space = (free_space.0 + dx, free_space.1 + dy);
        }

        // Now, is it a free space or a wall
        if self.map[free_space.1 as usize][free_space.0 as usize] == '.' {
            self.robot_pos = new_pos; // The robot moved
            self.map[new_pos.1 as usize][new_pos.0 as usize] = '.';
            self.map[free_space.1 as usize][free_space.0 as usize] = 'O';
            return true;
        }

        false // It was a wall, couldn't move the robot
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_delta(&self) -> (T, T) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut map = Vec::new();
        let mut robot_pos = (0, 0);
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    robot_pos = (x as T, y as T);
                    row.push('.');
                } else {
                    row.push(c);
                }
            }
            map.push(row);
        }

        let width = map[0].len() as T;
        let height = map.len() as T;

        Self {
            robot_pos,
            map,
            width,
            height,
        }
    }
}

impl Solution {
    pub fn init() -> Self {
        let text = read_to_string("inputs/day15.txt").unwrap();
        let (map, moves) = text.split_once("\r\n\r\n").unwrap();
        let map = Map::from(map);
        let moves = moves.replace("\r\n", "").chars().map(|c| Direction::from(c)).collect::<Vec<_>>();

        Self {
            map,
            moves,
        }
    }

    fn part1(&mut self) -> usize {
        let mut map = self.map.clone();
        for m in &self.moves {
            map.move_robot(m);
        }
        
        // Compute the GPS coordinates of the boxes
        let mut gps_sum = 0;
        for (y, row) in map.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'O' {
                    gps_sum += x + 100 * y;
                }
            }
        }
        gps_sum
    }

    fn part2(&mut self) {

    }

    pub fn solve(&mut self) {
        println!("========= DAY 15 ========");
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