use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    println!("{}", get_your_id("input.txt"));
}

struct Seat {
    index: u8,
}

impl Seat {
    fn parse(string: &str) -> Option<Self>  {
        let mut index: u16 = 0;
        let bytes = string.as_bytes();
        if bytes.len() != 10 { return None };
        for pair in bytes.iter().enumerate() {
            index = (index << 1) + match pair {
                (0..=6, b'B') => 1,
                (0..=6, b'F') => 0,
                (7..=9, b'R') => 1,
                (7..=9, b'L') => 0,
                _ => return None
            };
        }
        return Some(Seat(index))
    }
}


fn get_your_id(path: &str) -> u16 {
    let mut seats = Vec::new();
    let linereader = BufReader::new(File::open(path).expect("Failed to open file")).lines();
    for (lineno, line) in linereader.enumerate()
        .map(|(n, e)| (n, e.expect("Failed to read line.")))
        .filter(|(_n, e)| (!e.trim().is_empty())) {
        match partition_to_number(&line) {
            Some(n) => seats.push(n),
            None => panic!(format!("Failed to parse seat at line {}", lineno))
        }
    };
    seats.sort();
    let mut prev: u16 = *seats.first().unwrap();
    for seat in seats {
        if seat == prev + 2 {
            return prev + 1
        }
        prev = seat
    };
    panic!("Could not find your seat")
}

#[test]
fn test_parse() {
    let input = "FBFBBFFRLR";
    let seat = Seat::parse(input);
    assert_eq!(seat, Seat { row: 44, col: 5 });
}
