// 3-11 j: tjjj
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::convert::TryInto;

fn main() {
    let (re, lines) = setup();
    println!("{}", second_part(&re, lines))
}

fn setup() -> (Regex, io::Lines<io::BufReader<File>>) {
    let re = Regex::new(r"(\d+)\-(\d+) (\w): (\w+)").unwrap();
    (re, read_lines("input.txt"))
}

fn first_part(re: &Regex, lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut n_passed = 0;
    let (re, lines) = setup();
    for lineresult in lines {
        let line = lineresult.unwrap();
        let (password, chr, low, high) = parse_line(&line, &re);
        if password_passes_first(password, chr, low, high) {
            n_passed += 1;
        }
    }
    n_passed
}

fn second_part(re: &Regex, lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut n_passed = 0;
    let (re, lines) = setup();
    for lineresult in lines {
        let line = lineresult.unwrap();
        let (password, chr, low, high) = parse_line(&line, &re);
        if password_passes_second(password, chr, low, high) {
            n_passed += 1;
        }
    }
    n_passed
}

fn parse_line<'a>(line: &'a str, re: &Regex) -> (&'a str, char, i32, i32) {
    let captures = re.captures(&line).unwrap();    
    let low: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
    let high: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
    let chr = captures.get(3).unwrap().as_str().chars().next().unwrap();
    let password = captures.get(4).unwrap().as_str();
    return (password, chr, low, high)
}


fn read_lines(path: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(path).unwrap();
    io::BufReader::new(file).lines()
}

fn password_passes_first(password: &str, chr: char, low: i32, high: i32) -> bool {
    let mut n = 0;
    for i in password.chars() {
        if i == chr {
            n += 1;
        }
    }
    n <= high && low <= n
}

fn password_passes_second(password: &str, chr: char, low: i32, high: i32) -> bool {
    let mut first = false;
    let mut second = false;
    for (i, c) in password.chars().enumerate() {
        if c == chr {
            let pos: i32 = (i + 1).try_into().unwrap();
            if pos == low {
                first = true;
            } else if pos == high {
                second = true
            }
        }
    }
    first ^ second
}
