#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::iter::FromIterator;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Color {
    Brown,
    Hazel,
    Green,
    Grey,
    Blue,
    Ambiguous,
    Other,
    RGB(u8, u8, u8)
}

impl Color {
    fn from_ecl(str: &str) -> Result<Color, ParsePassportError> {
        let color = match str {
            "brn" => Color::Brown,
            "hzl" => Color::Hazel,
            "grn" => Color::Green,
            "gry" => Color::Grey,
            "blu" => Color::Blue,
            "amb" => Color::Ambiguous,
            "oth" => Color::Other,
            _ => return Err(ParsePassportError::InvalidValue)
        };
            
        Ok(color)
    }

    fn from_hcl(str: &str) -> Result<Color, ParsePassportError> {
        let caps = match RE.captures(str) {
            Some(c) => c,
            None => return Err(ParsePassportError::InvalidValue)
        };
        let c1 = u8::from_str_radix(caps.get(1).unwrap().as_str(), 16)?;
        let c2 = u8::from_str_radix(caps.get(2).unwrap().as_str(), 16)?;
        let c3 = u8::from_str_radix(caps.get(3).unwrap().as_str(), 16)?;
        Ok(Color::RGB(c1, c2, c3))
    }
}

// This should never panic
lazy_static! {
    static ref RE: Regex = Regex::new(r"^#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$").unwrap();
    static ref REQUIRED_FIELDS: HashSet<&'static str> = HashSet::from_iter(
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    static ref OPTIONAL_FIELDS: HashSet<&'static str> = HashSet::from_iter(vec!["cid"]);
    
}

fn assert_between<T: std::cmp::PartialOrd>(n: T, low: T, high: T) -> Result<T, ParsePassportError> {
    if low <= n && n <= high {
        Ok(n)
    } else {
        Err(ParsePassportError::InvalidValue)
    }
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

// How do I prevent throwing all the relevant information away
// when an error is converted to ParsePassportError?
#[derive(Debug, Clone)]
enum ParsePassportError {
    MissingField,
    UnexpectedField,
    DuplicateField,
    InvalidValue,
    ParserError(String),
    ReadError,
}

impl From<std::num::ParseIntError> for ParsePassportError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParsePassportError::InvalidValue
    }
}

impl From<std::io::Error> for ParsePassportError {
    fn from(_: std::io::Error) -> Self {
        ParsePassportError::ReadError
    }
}

impl Passport {
    fn from_hashmap(map: &HashMap<String, String>) -> Result<Passport, ParsePassportError> {
        let keyset: HashSet<&str> = HashSet::from_iter(map.keys().map(|s| s.as_ref()));
        let diff: HashSet<&str> = REQUIRED_FIELDS.difference(&keyset).copied().collect();
        
        // Must must contain required fields
        if diff.len() != 0 {
            return Result::Err(ParsePassportError::MissingField)
        }

        // Must not contain other fields
        let nonrequired: HashSet<&str> = keyset.difference(&REQUIRED_FIELDS).copied().collect();
        let superfluous: HashSet<&str> = nonrequired.difference(&OPTIONAL_FIELDS).copied().collect();
        if superfluous.len() != 0 {
            return Result::Err(ParsePassportError::UnexpectedField)
        }

        // Create passport
        let passport = Passport{
            byr: assert_between(map.get("byr").unwrap().parse::<u16>()?, 1920, 2002)?,
            eyr: assert_between(map.get("eyr").unwrap().parse::<u16>()?, 2020, 2030)?,
            iyr: assert_between(map.get("iyr").unwrap().parse::<u16>()?, 2010, 2020)?,

            // This should be formatted like \d+cm or \d+in
            hgt: {
                let hgtstring = map.get("hgt").unwrap();
                if let Some((i, _)) = hgtstring.char_indices().rev().nth(1) {
                    let (factor, low, high) = match &hgtstring[i..] {
                        "cm" => (1.0, 150, 193),
                        "in" => (2.54, 59, 76),
                        _ => return Err(ParsePassportError::InvalidValue)
                    };
                    
                    let value = assert_between(hgtstring[..i].parse::<u8>()?, low, high)?;
                    (factor * (value as f64).round()) as u16    
                } else {
                    return Err(ParsePassportError::InvalidValue)
                }
            },

            hcl: Color::from_hcl(&map.get("hcl").unwrap())?,
            ecl: Color::from_ecl(&map.get("ecl").unwrap())?,
            
            pid: {
                let pid = &map.get("pid").unwrap();
                if pid.len() != 9 {
                    return Err(ParsePassportError::InvalidValue)
                
                };
                pid.parse::<u64>()?
            },
            
            cid: match map.get("cid") {
                Some(str) => {
                    match str.parse::<u16>() {
                        Ok(n) => Some(n),
                        Err(_) => None
                    }
                }
                None => Option::None
            }
        };

        return Ok(passport)
    }
}

struct PassportIterator {
    io: BufReader<File>,
    map: HashMap<String, String>,
    linenumber: usize
}

impl PassportIterator {
    fn from_path(string: &str) -> PassportIterator {
        let iobuf = BufReader::new(File::open(string).expect("Failed to open file"));
        PassportIterator{io: iobuf, map: HashMap::<String, String>::new(), linenumber: 0}
    }
}

impl Iterator for PassportIterator {
    type Item = (usize, Result<Passport, ParsePassportError>);
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut linebuffer: String = String::new();
        let mut yieldpassport = false;
        loop {
            if let 0 = self.io.read_line(&mut linebuffer).expect("Failed to read line") {
                if !self.map.is_empty() {
                    yieldpassport = true;
                } else {
                    return None
                }
            }
            self.linenumber += 1;

            if linebuffer.trim().is_empty() && !self.map.is_empty() {
                yieldpassport = true;
            } else {
                let update = update_hashmap(&mut self.map, &linebuffer); 
                linebuffer.clear();
                match update {
                    Ok(_n) => (),
                    Err(e) => return Some((self.linenumber, Err(e)))
                }
            }
            if yieldpassport {
                let passportresult = Passport::from_hashmap(&mut self.map);
                self.map.clear();
                return Some((self.linenumber, passportresult))
            }
        }
    }
}

fn main() {
    println!("{:?}", count_valid_passports("input.txt"));
}

fn count_valid_passports(path: &str) -> u32 {
    let mut n: u32 = 0;
    let iter = PassportIterator::from_path(path).into_iter();
    for (linenumber, result) in iter {
        match result {
            Ok(_p) => {n += 1},
            Err(ParsePassportError::MissingField) => {},
            Err(ParsePassportError::InvalidValue) => {},
            Err(e) => panic!("{:?} near line {}", e, linenumber)
        }
    }
    n
}

fn update_hashmap(hashmap: &mut HashMap<String, String>, line: &str) -> Result<u32, ParsePassportError> {
    let mut n_inserts: u32 = 0;
    for pair in line.split_whitespace() {
        let parsedpair = parse_keyval_pairs(pair);
        match parsedpair {
            Some((key, val)) => {
                let insertresult = hashmap.insert(key.to_string(), val.to_string());
                if let Some(_value) = insertresult {
                    println!("Field is {}", key);
                    return Err(ParsePassportError::DuplicateField)
                }
                n_inserts += 1;
            },
            None => {
                return Err(ParsePassportError::ParserError(format!("Cannot parse as key-value pair {}", pair)))
            }
        }
    }
    return Ok(n_inserts)
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
