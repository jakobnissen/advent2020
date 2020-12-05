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
    static ref REQUIRED_FIELDS: HashSet<&'static str> = HashSet::from_iter(vec!["foo", "bar"]);
    static ref OPTIONAL_FIELDS: HashSet<&'static str> = HashSet::from_iter(vec!["qux"]);
    
}

fn parse_color_hex(hex: &str) -> Option<Color> {   
    let caps = RE.captures(hex)?;
    let c1 = u8::from_str_radix(caps.get(1).unwrap().as_str(), 16).ok()?;
    let c2 = u8::from_str_radix(caps.get(2).unwrap().as_str(), 16).ok()?;
    let c3 = u8::from_str_radix(caps.get(3).unwrap().as_str(), 16).ok()?;
    Some(Color::RGB(c1, c2, c3))
}

#[derive(Debug)]
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

fn exit_with(string: &str) {
    println!("{}", string);
    std::process::exit(1)
}

impl Passport {
    // Must be a superset
    fn from_hashmap(map: &HashMap<&str, &str>) -> Passport {
        let keyset = HashSet::from_iter(map.keys().copied());
        let mut diff: HashSet<&str> = REQUIRED_FIELDS.difference(&keyset).copied().collect();
        
        // Must must contain required fields
        if diff.len() != 0 {
            let error_string = diff.iter().copied().collect::<Vec<&str>>().join(", ");
            exit_with(&format!("Missing fields of passport: {:?}", error_string))
        }

        // TODO: Must not contain other fields
        let nonrequired: HashSet<&str> = keyset.difference(&REQUIRED_FIELDS).copied().collect();
        let superfluous: HashSet<&str> = nonrequired.difference(&OPTIONAL_FIELDS).copied().collect();
        if superfluous.len() != 0 {
            let error_string = superfluous.iter().copied().collect::<Vec<&str>>().join(", ");
            exit_with(&format!("Unaccepted fields of passport: {:?}", error_string))
        }

        // Create passport
        
        

        Passport{byr: 1, iyr: 1, eyr: 1, hgt: 1, hcl: Color::Blue, ecl: Color::Other, pid:101, cid: Some(11)}
    }
}

fn main() {
    let mut map = HashMap::new();
    update_hashmap(&mut map, "foo:bar baz:tar");
    let passport = Passport::from_hashmap(&map);
    println!("{:?}", passport);
}

fn update_hashmap<'a>(hashmap: &mut HashMap<&'a str, &'a str>, line: &'a str) -> u32 {
    let mut n_inserts: u32 = 0;
    for pair in line.split_whitespace() {
        let parsedpair = parse_keyval_pairs(pair);
        match parsedpair {
            Some((key, val)) => {
                let insertresult = hashmap.insert(key, val);
                if let Some(_value) = insertresult {
                    exit_with(&format!("Record has multiple keys \"{}\"", key));
                }
                n_inserts += 1;
            },
            None => {
                exit_with(&format!("Cannot parse key-value pair \"{}\".", pair));
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
