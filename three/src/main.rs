use std::io::{self, BufRead};
use std::{env, fs};

fn max_joltage_1(bank: &str) -> u64 {
    let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

    let mut max = 0;
    for i in 0..digits.len() {
        for j in 0..digits.len() {
            if i < j {
                let num = digits[i] * 10 + digits[j];
                if num > max {
                    max = num;
                }
            }
        }
    }
    max as u64
}

fn max_12_digit_number(bank: &str) -> u64 {
    let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();
    let n = digits.len();
    let k = 12;
    
    if n < k {
        return 0;
    }
    
    let mut result = String::with_capacity(k);
    let mut last_pos = 0;
    let mut remaining = k;
    
    for _ in 0..k {
        let max_pos = n - (remaining - 1) - 1;
        let mut max_digit = 0;
        
        for pos in last_pos..=max_pos {
            if digits[pos] > max_digit {
                max_digit = digits[pos];
                last_pos = pos + 1;
                if max_digit == 9 { break; }
            }
        }
        
        result.push(std::char::from_digit(max_digit, 10).unwrap());
        remaining -= 1;
    }
    
    result.parse().unwrap_or(0)
}

fn main() {
    let file_path = env::args().nth(1).expect("No file path provided");
    let file = fs::File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let banks: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let total_1: u64 = banks.iter().map(|bank| max_joltage_1(&bank)).sum();
    let total_2: u64 = banks.iter().map(|bank| max_12_digit_number(&bank)).sum();

    println!("Part 1 - Total output joltage: {}", total_1);
    println!("Part 2 - Total output joltage: {}", total_2);
}
