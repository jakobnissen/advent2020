#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::iter::FromIterator;

#[derive(Debug)]
enum Color {
    Brown,
    Hazel,
    Green,
    Grey,
    Blue,
    Other,
    RGB(u8, u8, u8)
}

impl Color {
    fn from_str(str: &str) -> Option<Color> {
        let content = match str {
            "brn" => Color::Brown,
            "hzl" => Color::Hazel,
            "grn" => Color::Green,
            "gry" => Color::Grey,
            "blu" => Color::Blue,
            "oth" => Color::Other,
            _ => parse_color_hex(&str)?
        };
        Some(content)
    }    
}

// This should never panic
lazy_static! {
    static ref RE: Regex = Regex::new(r"#?([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})").unwrap();
}

fn parse_color_hex(hex: &str) -> Option<Color> {   
    let caps = RE.captures(hex)?;
    let c1 = u8::from_str_radix(caps.get(1).unwrap().as_str(), 16).ok()?;
    let c2 = u8::from_str_radix(caps.get(2).unwrap().as_str(), 16).ok()?;
    let c3 = u8::from_str_radix(caps.get(3).unwrap().as_str(), 16).ok()?;
    Some(Color::RGB(c1, c2, c3))
}

struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: u16, // in centimeters
    hcl: Color,
    ecl: Color,
    pid: u64,
    cid: Option<u16>
}

/*
impl Passport {
    fn from_hashmap(map: &HashMap) -> Option<Passport> {
        
    }
}
*/

fn main() {
    let mut map = HashMap::new();
    let mut n = 0;
    n += update_hashmap(&mut map, "foo:bar baz:tar");
    let set: HashSet<&str> = HashSet::from_iter(map.keys().copied());
    println!("{:?}", set);
}

fn update_hashmap<'a>(hashmap: &mut HashMap<&'a str, &'a str>, line: &'a str) -> u32 {
    let mut n_inserts: u32 = 0;
    for pair in line.split_whitespace() {
        let parsedpair = parse_keyval_pairs(pair);
        match parsedpair {
            Some((key, val)) => {
                let insertresult = hashmap.insert(key, val);
                if let Some(value) = insertresult {
                    println!("Record has multiple keys \"{}\"", key);
                    std::process::exit(1)
                }
                n_inserts += 1;
            },
            None => {
                println!("Cannot parse key-value pair \"{}\".", pair);
                std::process::exit(1)
            }
        }
    }
    return n_inserts
}

fn parse_keyval_pairs(string: &str) -> Option<(&str, &str)> {
    let mut key = Default::default();
    let mut value = Default::default();
    for (i, substr) in string.split(':').enumerate() {
        match i {
            0 => {key = substr},
            1 => {value = substr},
            _ => return None
        }
    }
    return Some((key, value))
}
