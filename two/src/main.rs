use std::io::{self, BufRead};
use std::{env, fs};

fn is_invalid_id_part1(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];

    first_half == second_half && !first_half.starts_with('0')
}

fn is_invalid_id_part2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    for pattern_len in 1..=len/2 {
        if len % pattern_len != 0 {
            continue;
        }
        
        let pattern = &s[..pattern_len];
        let mut is_valid = true;
        
        for i in (pattern_len..len).step_by(pattern_len) {
            if &s[i..i+pattern_len] != pattern {
                is_valid = false;
                break;
            }
        }
        
        if is_valid && !pattern.starts_with('0') {
            return true;
        }
    }
    
    false
}

fn main() {
    let file_path = env::args().nth(1).expect("No file path provided");
    let file = fs::File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let first_line = reader
        .lines()
        .next()
        .expect("Failed to read first line")
        .unwrap();

    let ranges: Vec<(u64, u64)> = first_line
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    let mut total_1 = 0;
    let mut total_2 = 0;

    for (start, end) in ranges {
        for num in start..=end {
            if is_invalid_id_part1(num) {
                total_1 += num;
            }
            if is_invalid_id_part2(num) {
                total_2 += num;
            }
        }
    }

    println!("Sum of all invalid IDs: {}", total_1);
    println!("Sum of all invalid IDs: {}", total_2);
}
