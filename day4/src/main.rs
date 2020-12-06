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
    Other,
    RGB(u8, u8, u8)
}

impl Color {
    fn from_str(str: &str) -> Color {
        match str {
            "brn" => Color::Brown,
            "hzl" => Color::Hazel,
            "grn" => Color::Green,
            "gry" => Color::Grey,
            "blu" => Color::Blue,
            _ => Color::Other
        }
    }
}

// I want to try to avoid panicking in this code, so I make a function to exit the program orderly.
fn exit_with(string: &str) -> ! {
    println!("{}", string);
    std::process::exit(1)
}


// This should never panic
lazy_static! {
    static ref RE: Regex = Regex::new(r"#?([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})").unwrap();
    static ref REQUIRED_FIELDS: HashSet<&'static str> = HashSet::from_iter(
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    static ref OPTIONAL_FIELDS: HashSet<&'static str> = HashSet::from_iter(vec!["cid"]);
    
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

// How do I prevent throwing all the relevant information away
// when an error is converted to ParsePassportError?
#[derive(Debug, Clone)]
enum ParsePassportError {
    MissingField,
    UnexpectedField,
    DuplicateField,
    ParserError,
    ReadError,
}

impl From<std::num::ParseIntError> for ParsePassportError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParsePassportError::ParserError
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
            byr: map.get("byr").unwrap().parse::<u16>()?,
            iyr: map.get("iyr").unwrap().parse::<u16>()?,
            eyr: map.get("eyr").unwrap().parse::<u16>()?,

            // This should be formatted like \d+cm or \d+in or \d+
            hgt: {
                let hgtstring = map.get("hgt").unwrap();
                let mut factor = 1.0;
                let mut beginning = Default::default();
                if let Some((i, _)) = hgtstring.char_indices().rev().nth(1) {
                    match &hgtstring[i..] {
                        "cm" => {factor = 1.0; beginning = &hgtstring[..i]},
                        "in" => {factor = 2.54; beginning = &hgtstring[..i]},
                        _ => {factor = 1.0; beginning = &hgtstring}
                    }
                } else {
                    {factor = 1.0; beginning = &hgtstring}
                }
                (beginning.parse::<u16>()? as f32 * factor).round() as u16
            },

            hcl: Color::from_str(&map.get("hcl").unwrap()),
            ecl: Color::from_str(&map.get("ecl").unwrap()),
            
            // We somehow allow the PID to be fucked up.
            //pid: map.get("pid").unwrap().parse::<u64>()?,
            pid: 0,
            
            cid: match map.get("cid") {
                Some(str) => Option::Some(str.parse::<u16>()?),
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
        let map = HashMap::<String, String>::new();
        PassportIterator{io: iobuf, map: map, linenumber: 0}
    }
}

impl Iterator for PassportIterator {
    type Item = (usize, Result<Passport, ParsePassportError>);
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut linebuffer: String = String::new();
        loop {
            if let 0 = self.io.read_line(&mut linebuffer).expect("Failed to read line") {
                return None
            }
            self.linenumber += 1;

            if linebuffer.trim().is_empty() && !self.map.is_empty() {
                let passportresult = Passport::from_hashmap(&mut self.map);
                self.map.clear();
                return Some((self.linenumber, passportresult))
            } else {
                let update = update_hashmap(&mut self.map, &linebuffer); 
                linebuffer.clear();
                match update {
                    Ok(_n) => (),
                    Err(e) => return Some((self.linenumber, Err(e)))
                }
            }
        }
    }
}

fn main() {
    let passport = count_valid_passports("input.txt");
    println!("{:?}", passport);
}

fn count_valid_passports(path: &str) -> u32 {
    let mut n: u32 = 0;
    let iter = PassportIterator::from_path(path).into_iter();
    for (linenumber, result) in iter {
        match result {
            Ok(_p) => {n += 1},
            Err(ParsePassportError::MissingField) => {},
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
                return Err(ParsePassportError::ParserError)
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
