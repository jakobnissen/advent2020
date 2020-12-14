use std::collections::HashMap;
use std::fmt;
use anyhow;
use peg;
use LineInstruction::*;

fn main() {
    let input = include_str!("input.txt");
    let mut mask = BitSetter::new();
    let mut map: HashMap<usize, u64> = HashMap::new();
    input.trim().lines().for_each(|s| {
        let instruction = parse_line(s).expect("Invalid line");
        match instruction {
            SetMask(m) => { mask = m },
            SetBit(i, b) => { map.insert(i, mask.set(b)); }
        }
    });
    println!("{}", map.values().sum::<u64>())
}

enum LineInstruction {
    SetMask(BitSetter),
    SetBit(usize, u64),
}

// TODO: Do proper error handling here
fn parse_line(s: &str) -> anyhow::Result<LineInstruction> {
    peg::parser! {
        grammar parser() for str {
            rule mask() -> BitSetter
             = msk:$(['0' | '1' | 'X']+) { BitSetter::from_str(msk).unwrap() }

            rule maskline() -> BitSetter
             = "mask = " msk:mask() { msk }

            rule number() -> usize
             = n:$(['0'..='9']+) { n.parse().unwrap() }
             
            rule mem() -> usize
             = "mem[" n:number() "]" { n }

            rule memline() -> (usize, u64)
             = mm:mem() " = " n:number() { (mm, n as u64) }

            pub(crate) rule line() -> LineInstruction = precedence! {
                msk: maskline() { SetMask(msk) }
                --
                mm: memline() { SetBit(mm.0, mm.1) }
            }
        }
    }
    Ok(parser::line(s)?)
}

struct BitSetter {
    or_mask: u64,
    and_mask: u64,
}

#[derive(Debug, Clone, Copy)]
struct ParseBitSetterError;

impl fmt::Display for ParseBitSetterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid string")
    }
}

impl BitSetter {
    fn new() -> BitSetter {
        BitSetter{or_mask: 0, and_mask: u64::MAX}    
    }
    
    fn set(&self, n: u64) -> u64 {
        (n & self.and_mask) | self.or_mask
    }

    fn from_str(string: &str) -> Result<BitSetter, ParseBitSetterError> {
        let mut or_mask: u64 = 0;
        let mut and_mask: u64 = u64::MAX;
        if string.len() > 64 {
            return Err(ParseBitSetterError)
        }
        for chr in string.chars() {
            let (or_or, and_or) = match chr {
                '0' => (0, 0),
                '1' => (1, 1),
                'X' => (0, 1),
                _ => return Err(ParseBitSetterError),
            };
            or_mask = (or_mask << 1) | or_or;
            and_mask = (and_mask << 1) | and_or;
        }
        Ok(BitSetter{or_mask, and_mask})
    }
}
