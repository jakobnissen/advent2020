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

    fn force_from_str(str: &str) -> Color {
        if let Some(color) = Color::from_str(str) {
            color
        } else {
            exit_with(&format!("Cannot parse \"{}\" as color.", str))
        }
    }
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

fn exit_with(string: &str) -> ! {
    println!("{}", string);
    std::process::exit(1)
}

fn try_parse_u16(str: &str) -> u16 {
    let res = str.parse::<u16>();
    if let Ok(n) = res {
        return n
    } else {
        exit_with(&format!("Cannot parse as u16: {}", str))
    }
}

fn try_parse_u64(str: &str) -> u64 {
    let res = str.parse::<u64>();
    if let Ok(n) = res {
        n
    } else {
        exit_with(&format!("Cannot parse as u64: {}", str))   
    }
}

impl Passport {
    // Must be a superset
    fn from_hashmap(map: &HashMap<&str, &str>) -> Passport {
        let keyset = HashSet::from_iter(map.keys().copied());
        let diff: HashSet<&str> = REQUIRED_FIELDS.difference(&keyset).copied().collect();
        
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
        Passport{
            byr: try_parse_u16(map.get("byr").unwrap()),
            iyr: try_parse_u16(map.get("iyr").unwrap()),
            eyr: try_parse_u16(map.get("eyr").unwrap()),
            hgt: try_parse_u16(map.get("hgt").unwrap()),
            hcl: Color::force_from_str(&map.get("hcl").unwrap()),
            ecl: Color::force_from_str(&map.get("ecl").unwrap()),
            pid: try_parse_u64(map.get("pid").unwrap()),
            cid: match map.get("cid") {
                Some(str) => Option::Some(try_parse_u16(str)),
                None => Option::None
            }
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    update_hashmap(&mut map, "byr:1985 iyr:2000 eyr:2005 hgt:205 ecl:blu hcl:#14a2f2 pid:35232342");
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
