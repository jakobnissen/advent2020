use std::collections::HashMap;
use peg;
use peg::{error::ParseError, str::LineCol};
use LineInstruction::*;

fn main() {
    let input = include_str!("input.txt");
    let mut mask = BitSetter::new();
    let mut map_part1: HashMap<usize, u64> = HashMap::with_capacity(500);
    let mut map_part2: HashMap<u64, u64> = HashMap::with_capacity(100_000);
    input.trim().lines().for_each(|s| {
        let instruction = parse_line(s).expect("Invalid line");
        match instruction {
            SetMask(m) => { mask = m },
            SetBit(i, b) => {
            
                // For part one, merely set bits
                map_part1.insert(i, mask.set_bits(b));

                // For part two, iterate over all submasks and set
                // the yielded addresses
                for mem in FloatBitIterator::new(&mask, i as u64) {
                    map_part2.insert(mem, b);
                }
            }
        }
    });
    println!("{}", map_part1.values().sum::<u64>());
    println!("{}", map_part2.values().sum::<u64>());
}

enum LineInstruction {
    SetMask(BitSetter),
    SetBit(usize, u64),
}

fn parse_line(s: &str) -> Result<LineInstruction, ParseError<LineCol>> {
    peg::parser! {
        grammar parser() for str {
            rule mask() -> BitSetter
             = msk:$(['0' | '1' | 'X']+) {? match BitSetter::from_str(msk) {
                Some(n) => Ok(n),
                None => Err("Failed to parse bitsetter")
             } }

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
    float_mask: u64,
}

impl BitSetter {
    fn new() -> BitSetter {
        BitSetter{or_mask: 0, and_mask: u64::MAX, float_mask: 0}    
    }
    
    fn set_bits(&self, n: u64) -> u64 {
        (n & self.and_mask) | self.or_mask
    }

    fn set_ones(&self, n: u64) -> u64 {
        n | self.or_mask
    }

    fn from_str(string: &str) -> Option<BitSetter> {
        let (mut or_mask, mut and_mask, mut float_mask):
        (u64, u64, u64) = (0, 0, 0);
        if string.len() > 64 {
            return None
        }
        for chr in string.chars() {
            let (or_or, and_or, float_or) = match chr {
                '0' => (0, 0, 0),
                '1' => (1, 1, 0),
                'X' => (0, 1, 1),
                _ => return None,
            };
            or_mask = (or_mask << 1) | or_or;
            and_mask = (and_mask << 1) | and_or;
            float_mask = (float_mask << 1) | float_or;
        }
        Some(BitSetter{or_mask, and_mask, float_mask})
    }
}

// This iterates all submasks from zero to self.mask. State is the
// inverse of the submask (because the internal algorithm iterates state
// from self.mask to zero)
struct FloatBitIterator {
    n: u64,
    mask: u64,
    state: u64,
    done: bool
}

impl FloatBitIterator {
    fn new(bitmask: &BitSetter, n: u64) -> FloatBitIterator {
        let masked = bitmask.set_ones(n);
        let fm = bitmask.float_mask;
        FloatBitIterator{n: masked, mask: fm, state: fm, done: false }
    }
}

impl Iterator for FloatBitIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let state = self.state;
        // State == 0 signifies end of iterator.
        if self.done {
            return None
        }

        // Iterator counts down, we invert in order for it to count from 0
        // and up to mask
        let inverted_state = self.mask & !state;
        
        // Zero out bits in self.mask, then add in the state
        let result = Some((self.n & !(self.mask)) | inverted_state);

        // Update state
        if state == 0 {
            self.done = true;
        } else {
            self.state = (state - 1) & self.mask;
        }

        return result
    }
}

#[cfg(test)]
mod tests {
    use super::{BitSetter, FloatBitIterator};

    #[test]
    fn test_init_bitsetter() {
        let bitsetter = BitSetter::from_str(&"10X011XXX101X1").unwrap();
        assert_eq!(bitsetter.or_mask,       0b10001100010101);
        assert_eq!(bitsetter.and_mask,      0b10101111110111);
        assert_eq!(bitsetter.float_mask,    0b00100011100010);
    }

    #[test]
    fn test_set_bitsetter() {
        let n: u64 =                0b10100111010100;
        let bs = BitSetter::from_str("10X001X10XX101").unwrap();
        assert_eq!(bs.set_bits(n),  0b10100111010101);
        assert_eq!(bs.set_ones(n),  0b10100111010101);
    }

    #[test]
    fn test_float_iter() {
        let n: u64 =                0b10100011;
        let bs = BitSetter::from_str("10X001X1").unwrap();
        let mut fi = FloatBitIterator::new(&bs, n);
        assert_eq!(fi.next(),  Some(0b10000101)); 
        assert_eq!(fi.next(),  Some(0b10000111)); 
        assert_eq!(fi.next(),  Some(0b10100101)); 
        assert_eq!(fi.next(),  Some(0b10100111)); 
        assert_eq!(fi.next(),  None);    
    }
}
