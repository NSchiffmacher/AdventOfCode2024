use std::fs::read_to_string;
use std::io::{self, Write};
use std::sync::LazyLock;
use std::collections::HashMap;

type T = isize;
static NUMERIC_KEYBOARD_MAP: LazyLock<HashMap<(T, T), char>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert((0, 0), '7');
    map.insert((1, 0), '8');
    map.insert((2, 0), '9');
    map.insert((0, 1), '4');
    map.insert((1, 1), '5');
    map.insert((2, 1), '6');
    map.insert((0, 2), '1');
    map.insert((1, 2), '2');
    map.insert((2, 2), '3');
    map.insert((1, 3), '0');
    map.insert((2, 3), 'A');
    map
});
static DIRECTIONAL_KEYBOARD_MAP: LazyLock<HashMap<(T, T), char>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert((1, 0), '^');
    map.insert((2, 0), 'A');
    map.insert((0, 1), '<');
    map.insert((1, 1), 'v');
    map.insert((2, 1), '>');
    map
});

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

#[derive(Debug, Clone)]
struct Layer {
    map: HashMap<(T, T), char>,
    position: (T, T),
}

impl Layer {
    pub fn new(map: HashMap<(T, T), char>, position: (T, T)) -> Self {
        Self {
            map,
            position,
        }
    }

    pub fn press(&mut self, order: char) -> Result<Option<char>, ()> {
        if order == 'A' {
            // Order to press the current button, we get the input for the next layer
            return Ok(Some(self.current_char()));
        }

        // Otherwise, it must be a delta
        let delta = Direction::from(order).to_delta();
        self.update(delta)?;
        Ok(None) // No input for the next layer, we just moved
    }

    pub fn update(&mut self, (dx, dy): (T, T)) -> Result<(), ()>{
        // Returns the char for the next layer, if A is pressed
        let (nx, ny) = (self.position.0 + dx, self.position.1 + dy);
        if !self.map.contains_key(&(nx, ny)) {
            return Err(());
        } 
        
        self.position = (nx, ny);
        Ok(())
    }

    pub fn current_char(&self) -> char {
        self.map[&self.position]
    }
}

#[derive(Debug, Clone)]
struct Robot {
    // input_keyboard: Layer,
    dir_keyboard_1: Layer,
    dir_keyboard_2: Layer,
    num_keyboard: Layer,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            // input_keyboard: Layer::new(DIRECTIONAL_KEYBOARD_MAP.clone(), (2, 0)),
            dir_keyboard_1: Layer::new(DIRECTIONAL_KEYBOARD_MAP.clone(), (2, 0)),
            dir_keyboard_2: Layer::new(DIRECTIONAL_KEYBOARD_MAP.clone(), (2, 0)),
            num_keyboard: Layer::new(NUMERIC_KEYBOARD_MAP.clone(), (2, 3)),
        }
    }

    pub fn update(&mut self, input: char) -> Result<Option<char>, ()> {
        if let Some(xxxx) = self.dir_keyboard_1.press(input)? {
            if let Some(yyyy) = self.dir_keyboard_2.press(xxxx)? {
                if yyyy == 'A' {
                    return Ok(Some(self.num_keyboard.current_char()));
                }
                // self.num_keyboard.press(yyyy);
            }
        }

        Ok(None) 
    }

    pub fn print_states(&self) {
        println!("Dir keyboard 1: {:?}", self.dir_keyboard_1.position);
        println!("Dir keyboard 2: {:?}", self.dir_keyboard_2.position);
        println!("Num keyboard: {:?}", self.num_keyboard.position);
    }
}


#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day21.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    // fn find_sequence(&self, code: &str) -> Vec<char> {
    //     let mut open_set = 


    //     vec![]
    // }

    fn part1(&mut self) {
        // println!("==");
        // let mut robot = Robot::new();
        // for inp in "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".chars() {
        //     if let Some(c) = robot.update(inp) {
        //         print!("{}", c);
        //     }
        // }
        // println!("==");
    }

    fn part2(&mut self) {

    }

    pub fn solve(&mut self) {
        println!("========= DAY 21 ========");
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