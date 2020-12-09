#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let p = parse_rule_file(BufReader::new(File::open("input.txt").unwrap())).unwrap();
    println!("{:#?}", part_1("shiny gold", &p));
    println!("{:#?}", part_2("shiny gold", &p));
}

lazy_static! {
    static ref HEAD_RE: Regex = Regex::new(r"^(\w+ \w+) bags $").unwrap();
    static ref TAIL_RE: Regex =
        Regex::new(r"^(?P<num>\d+) (?P<kind>\w+ \w+) bags?\.?$").unwrap();
}

#[derive(Debug)]
struct ParserError(usize);
type Bag = (String, Vec<(usize, String)>);

// Input is of kind:
// big red bags contain 1 dotted pink bag, 3 metallic orange bags.
fn parse_rule(string: &str) -> Option<Bag> {
    let mut headtail = string.split("contain ");

    // Verify head (the thing before "contain ")
    let match1 = HEAD_RE
        .captures(headtail.next()?)?
        .get(1)?
        .as_str()
        .to_string();
    let tail = headtail.next()?;
    if headtail.next().is_some() {
        return None;
    }

    // Verify tail (the thing after "contain ")
    let mut vec: Vec<(usize, String)> = Vec::new();
    if tail == "no other bags." {
        return Some((match1, vec));
    }

    // Fill in tail
    for str in tail.split(", ") {
        let caps = TAIL_RE.captures(str)?;
        let n = caps.name("num")?.as_str().parse::<usize>().ok()?;
        let kind = caps.name("kind")?.as_str().to_string();
        vec.push((n, kind));
    }
    Some((match1, vec))
}

// Load in the file to a vector of (o, [(n, i) ... ]) where o is the outer bag
// which must contain n of i inner bags etc.
fn parse_rule_file<T: BufRead>(reader: T) -> Result<Vec<Bag>, ParserError> {
    let mut result: Vec<Bag> = Vec::new();
    for (lineno, lineres) in reader.lines().enumerate() {
        let line = match lineres {
            Ok(line) => line,
            Err(_) => return Err(ParserError(lineno)),
        };
        if line.trim().is_empty() {
            continue;
        }
        let elem = match parse_rule(line.trim()) {
            Some(e) => e,
            None => return Err(ParserError(lineno)),
        };
        result.push(elem);
    }
    Ok(result)
}

// Map from e.g. "dotted blue" => [(3, "wavy gold"), (1, "dark maroon")]
fn inner_outer_hashmap(v: &[Bag]) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (outer, inners) in v {
        for (_n, inner) in inners {
            match map.get_mut::<str>(&inner) {
                None => {
                    map.insert(inner, vec![outer]);
                }
                Some(v) => v.push(outer),
            }
        }
    }
    map
}

// We just make sure to not double-count the kinds of bags we have already
// seen, by keeping a set of processed types of bags
fn part_1(inner: &str, bags: &[Bag]) -> usize {
    let map = inner_outer_hashmap(bags);
    let mut unprocessed: Vec<&str> = map
        .get::<str>(inner)
        .expect("Input string not in map")
        .iter()
        .copied()
        .collect();
    let mut processed: HashSet<&str> = HashSet::new();
    loop {
        let bag = match unprocessed.pop() {
            None => return processed.len(),
            Some(b) => b,
        };
        processed.insert(bag);
        if let Some(outers) = map.get(bag) {
            for outer in outers {
                if processed.contains(outer) {
                    continue;
                };
                unprocessed.push(outer);
            }
        }
    }
}

// This is more tricky, because we need to verify that there are no recursive patters.
// We begin by considering bags with no other bags. These contain a total of 0 internal bags.
// We then iterate over the remaining, looking for bags which contain inner bags, for which
// we know their number, and calculate their number of inner bags.
// If we do one round without inferring at least one new bag, and without having already
// inferred our target bag, the problem is unsolvable and we panic.
fn part_2(outer: &str, bags: &[Bag]) -> usize {
    let mut containing_bags: HashMap<&str, usize> = bags
        .iter()
        .filter(|(_h, v)| v.is_empty())
        .map(|(h, _v)| (h.as_str(), 0))
        .collect();

    let mut remaining = bags
        .iter()
        .filter(|(h, _v)| !containing_bags.contains_key::<str>(h))
        .map(|(h, v)| (h, v))
        .collect::<Vec<_>>();
    let mut skipped = Vec::new();

    while !containing_bags.contains_key(outer) {
        let n_elem = remaining.len();

        for (outer, v) in remaining.drain(..) {
            if v.iter().all(|(_n, h)| containing_bags.contains_key::<str>(h)) {
                let n = v
                    .iter()
                    .map(|(n, h)| n * containing_bags.get::<str>(h).unwrap() + n)
                    .sum();
                containing_bags.insert(outer, n);
            } else {
                skipped.push((outer, v));
            }
        }
        if skipped.len() == n_elem {
            panic!("Recursively defined rules or disjoint rule tree")
        };
        std::mem::swap(&mut remaining, &mut skipped);
    }
    *containing_bags.get(outer).unwrap()
}
