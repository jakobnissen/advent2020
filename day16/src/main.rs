use std::ops::RangeInclusive;
use std::collections::{HashSet, HashMap};

fn main() {
    let input = include_str!("input.txt");
    let (header, own, nearby) = {
        let mut s = input.split("\n\n");
        let v = (
            s.next().unwrap(),
            s.next().unwrap(),
            s.next().unwrap(),
        );
        assert!(s.next().is_none());
        v
    };
    let nearby_tickets = parse_nearby_tickets(nearby);
    let own_ticket = Ticket(own.lines().nth(1).unwrap().split(',').map(|f| {
        f.parse::<isize>().unwrap()
    }).collect());
    let fields: Vec<Field> = header.lines().map(|h| Field::from_str(h)).collect();
    let nearby_err_rate: isize = nearby_tickets.iter().map(|t| t.error_rate(&fields)).sum();
    println!("Nearby error rate: {}", nearby_err_rate);

    // Part 2
    let valid_tickets: Vec<&Ticket> = nearby_tickets.iter().filter(
        |t| t.is_valid(&fields)).collect();

    let positions = get_positions(&fields, &valid_tickets);
    let resmap = converge(&positions);

    let own_prod: isize = resmap.iter().filter(|(field, _pos)| {
        field.name.starts_with("departure")
    }).map(|(_f, pos)| own_ticket.0[*pos]).product();
    println!("Own product: {}", own_prod);
}

struct Ticket(Vec<isize>);

impl Ticket {
    fn error_rate(&self, fields: &[Field]) -> isize {
        self.0.iter().filter(|num| {
            fields.iter().all(|field| {
                !field.a.contains(num) && !field.b.contains(num)
            })
        }).sum()
    }
    
    fn is_valid(&self, fields: &[Field]) -> bool {
        self.0.iter().filter(|num| {
            fields.iter().all(|field| {
                !field.a.contains(num) && !field.b.contains(num)
            })
        }).count() == 0
    }
}

fn parse_nearby_tickets(string: &str) -> Vec<Ticket> {
    let mut lines = string.lines();
    match lines.next().unwrap().trim() {
        "nearby tickets:" => (),
        line => panic!("Expected 'nearby ticket', got {}", line)
    };
    lines.map(|line| Ticket({
        line.trim().split(',').map(|field| {
            field.parse::<isize>().expect("Failed to parse as integer")
        }).collect()
    })).collect()
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Field {
    name: String,
    a: RangeInclusive<isize>,
    b: RangeInclusive<isize>,
}

fn range_from_str(string: &str) -> RangeInclusive<isize> {
    let (a, b) =  {
        let mut s = string.split('-');
        (
            s.next().unwrap().parse::<isize>().unwrap(),
            s.next().unwrap().parse::<isize>().unwrap(),
        )
    };
    a..=b
}

impl Field {
    fn from_str(string: &str) -> Field {
        let (name, ranges) = {
            let mut s = string.split(": ");
            (s.next().unwrap().to_owned(), s.next().unwrap())
        };
        let (a, b) = {
            let mut s = ranges.split(" or ");
            (
                range_from_str(s.next().unwrap()),
                range_from_str(s.next().unwrap()),
            )
        };
        Field{name, a, b}
    }
}

/// Get the possible positions of each field
fn get_positions<'a>(fields: &'a [Field], tickets: &[&Ticket]) -> HashMap<&'a Field, HashSet<usize>> {
    let mut positions: HashMap<&Field, HashSet<usize>> = {
        fields.iter().map(|f| (f, {
            (0..fields.len()).collect()
        })).collect()
    };
    for ticket in tickets.iter() {
        for (i, n) in ticket.0.iter().enumerate() {
            for field in fields.iter() {
                if !(field.a.contains(n) || field.b.contains(n)) {
                    positions.get_mut(field).unwrap().remove(&i);
                }
            }
        }
    }
    positions
}

/// Given e.g. a map of {a: [0,1] b:[1]}, the only possible solution is a=0,
/// b=1. This is because 1 is unique to b, which means it cannot be present
/// in a. This function infers the unique solution for an input like the above,
/// even if there are many more key-value pairs
fn converge<'a>(positions: &HashMap<&'a Field, HashSet<usize>>) -> HashMap<&'a Field, usize> {
    let mut v: Vec<_> = positions.iter().map(|(f, s)| (*f, s.clone())).collect();
    let mut res: HashMap<&Field, usize> = HashMap::new();
    while !v.is_empty() {
        v.sort_by(|p1, p2| p2.1.len().cmp(&p1.1.len()));
        let (lastname, lastv) = v.pop().unwrap();
        if lastv.len() != 1 {
            panic!("Failed to converge")
        }
        let last = *lastv.iter().next().unwrap();
        res.insert(lastname, last);

        // Remove last from all other sets
        for (_f, s) in v.iter_mut() {
            s.remove(&last);
        }
    }
    res
}
