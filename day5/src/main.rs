use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    println!("{}", get_your_id("input.txt"));
}

fn partition_to_number(string: &str) -> Option<u32>  {
    let mut index: u32 = 0;
    for pair in string.chars().enumerate() {
        let factor = 512 >> pair.0;
        index += match pair {
            (0..=6, 'B') => factor,
            (0..=6, 'F') => 0,
            (7..=9, 'R') => factor,
            (7..=9, 'L') => 0,
            _ => return None
        };
    }
    return Some(index)
}

/*
fn get_higest_id(path: &str) -> u32 {
    let mut highest: u32 = 0;
    for lineresult in BufReader::new(File::open(path).expect("Failed to open line")).lines() {
        let line = lineresult.expect("Failed to read line.");
        if line.trim().is_empty() {
            continue;
        }
        let id = partition_to_number(&line).unwrap();
        highest = std::cmp::max(id, highest)
    }
    highest
}
*/

fn get_your_id(path: &str) -> u32 {
    let mut seats = Vec::new();
    let linereader = BufReader::new(File::open(path).expect("Failed to open file")).lines();
    for (lineno, line) in linereader.enumerate()
        .map(|(n, e)| (n, e.expect("Failed to read line.")))
        .filter(|(n, e)| (!e.trim().is_empty())) {
        match partition_to_number(&line) {
            Some(n) => seats.push(n),
            None => panic!(format!("Failed to parse seat at line {}", lineno))
        }
    };
    seats.sort();
    let mut prev: u32 = *seats.first().unwrap();
    for seat in seats {
        if seat == prev + 2 {
            return prev + 1
        }
        prev = seat
    };
    panic!("Could not find your seat")
}
