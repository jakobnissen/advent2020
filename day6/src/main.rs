use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let mut path: String = String::new();
    println!("Input path to read.");
    std::io::stdin().read_line(&mut path).expect("Failed to read line");
    println!("{}", part1(path.trim()));
    println!("{}", part2(path.trim()));
}

struct LineSepIterator<'a, T> {
    io: T,
    linebuf: &'a mut String,
}

impl <'a, T: BufRead> Iterator for LineSepIterator<'a, T> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.linebuf.clear();
        loop {
            let nread = self.io.read_line(&mut self.linebuf).expect("Failed to read line");
            if nread == 0 {
                return None
            } else if nread == 1 && self.linebuf.trim().is_empty() {
                self.linebuf.clear();
            } else {
                return Some(self.linebuf.split('a').collect())
            }
        }
    }
}

fn part1(path: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut bitset: u32 = 0;
    for line in read_lines(path) {
        if line.is_empty() && bitset != 0 {
            sum += bitset.count_ones();
            bitset = 0;
        } else {
            bitset |= parse_bitset(&line)
        }
    }
    sum += bitset.count_ones();
    return sum
}

fn part2(path: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut bitset: u32 = u32::MAX;
    for line in read_lines(path) {
        if line.is_empty() && bitset != u32::MAX {
            sum += bitset.count_ones();
            bitset = u32::MAX;
        } else {
            bitset &= parse_bitset(&line)
        }
    }
    sum += bitset.count_ones();
    return sum
}

fn parse_bitset(string: &str) -> u32 {
    let mut result: u32 = 0;
    for chr in string.chars() {
        let shift = match chr {
            'a'..='z' => (chr as u32) - ('a' as u32) ,
            _ => panic!(format!("Unrecognized char: {}", chr))
        };
        result |= 1u32 << shift;
    }
    return result
}

fn read_lines(path: &str) -> impl Iterator<Item=String> {
    BufReader::new(File::open(path).expect("Failed to open path")).lines()
                    .map(|x| x.expect("Failed to read line".trim()).to_string())
}
