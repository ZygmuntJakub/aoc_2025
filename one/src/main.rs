use std::{env, fs};
use std::io::{self, BufRead};

fn main() {
    let file_path = env::args().nth(1).expect("No file path provided");
    let file = fs::File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut value = 50; // must be between 0..99
    
    let mut key_1 = 0;
    let mut key_2 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parsed_line = parse_line(&line);

        if parsed_line.direction == Direction::Left {
            for _ in 0..parsed_line.value {
                value -= 1;
                if value == 0 {
                    key_2 += 1;
                }
                while value < 0 {
                    value += 100;
                }
            }
            
        } else {
            for _ in 0..parsed_line.value {
                value += 1;
                if value == 100 {
                    key_2 += 1;
                }
                while value > 99 {
                    value -= 100;
                }
            }
            
        }
        if value == 0 {
            key_1 += 1;
        }
    }

    println!("Key 1: {}", key_1);
    println!("Key 2: {}", key_2);
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

struct Line {
    direction: Direction,
    value: i32,
}

fn parse_line(line: &str) -> Line {
    let direction = if line.starts_with("L") {
        Direction::Left
    } else {
        Direction::Right
    };
    let value = line[1..].parse().unwrap();
    Line { direction, value }   
}
    
