use std::collections::HashMap;
use std::fmt;
use anyhow;
use peg;
use LineInstruction::*;

fn main() {
    let input = include_str!("test.txt");
    let mut mask = BitSetter::new();
    let mut map_part1: HashMap<usize, u64> = HashMap::new();
    let mut map_part2: HashMap<u64, u64> = HashMap::new();
    input.trim().lines().for_each(|s| {
        let instruction = parse_line(s).expect("Invalid line");
        match instruction {
            SetMask(m) => { mask = m },
            SetBit(i, b) => {
                // For part one, merely set bits
                map_part1.insert(i, mask.set_bits(b));

                // For part two,
                println!("Adress: {}", i);
                for mem in FloatBitIterator::new(&mask, i as u64) {
                    println!("{}", mem);
                    map_part2.insert(mem, b);
                }
            }
        }
    });
    println!("{:#?}", map_part2);
    println!("{}", map_part1.values().sum::<u64>());
    println!("{}", map_part2.values().sum::<u64>());

    /* Test FloatBitIterator */
}

enum LineInstruction {
    SetMask(BitSetter),
    SetBit(usize, u64),
}

fn parse_line(s: &str) -> anyhow::Result<LineInstruction> {
    peg::parser! {
        grammar parser() for str {
            rule mask() -> BitSetter
             = msk:$(['0' | '1' | 'X']+) {? match BitSetter::from_str(msk) {
                Ok(n) => Ok(n),
                Err(e) => Err("Failed to parse bitsetter")
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

#[derive(Debug, Clone, Copy)]
struct ParseBitSetterError;

impl fmt::Display for ParseBitSetterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid string")
    }
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

    fn from_str(string: &str) -> Result<BitSetter, ParseBitSetterError> {
        let (mut or_mask, mut and_mask, mut float_mask):
        (u64, u64, u64) = (0, 0, 0);
        if string.len() > 64 {
            return Err(ParseBitSetterError)
        }
        for chr in string.chars() {
            let (or_or, and_or, float_or) = match chr {
                '0' => (0, 0, 0),
                '1' => (1, 1, 0),
                'X' => (0, 1, 1),
                _ => return Err(ParseBitSetterError),
            };
            or_mask = (or_mask << 1) | or_or;
            and_mask = (and_mask << 1) | and_or;
            float_mask = (float_mask << 1) | float_or;
        }
        Ok(BitSetter{or_mask, and_mask, float_mask})
    }
}

struct FloatBitIterator {
    n: u64,
    mask: u64,
    state: u64,

    // n:     101010100110
    // mask:  000010001000
    // state: 111111110111 <- initial: zero at least sig bit of mask
    // state: 000000001000 <- after one iteration
    // state: 111101111111 <- then advance to next in mask, etc
    // [ ... ]
    // state: 000000000000 <- end of iterator.
}

impl FloatBitIterator {
    fn new(bitmask: &BitSetter, n: u64) -> FloatBitIterator {
        let masked = bitmask.set_ones(n);
        let state: u64 = if bitmask.float_mask == 0 { 0 } else {
            !(1 << bitmask.float_mask.trailing_zeros())
        };
        FloatBitIterator{n: masked, mask: bitmask.float_mask, state}
    }
}

impl Iterator for FloatBitIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let state = self.state;
        // State == 0 signifies end of iterator.
        if state == 0 {
            return None
        }

        let result = Some(
            if state.count_ones() == 1 {
                self.n | state
            } else {
                self.n & state
            });

        self.state = {
            let oneone = state.count_ones() == 1;
            if oneone {
                // Final leftward position of state, and one one is the last
                // state. So we set it to zero ( = finished )
                if state.leading_zeros() == self.mask.leading_zeros() {
                    0
                // Else we advance the state one tick leftwards and invert it
                } else {
                    let tz = (self.mask & !((state << 1) - 1)).trailing_zeros();
                    !(1 << tz)
                }
            } else {
                !state
            }
        };
        return result
    }
}

#[cfg(test)]
mod tests {
    use super::{BitSetter, FloatBitIterator};

    #[test]
    fn test_init_bitsetter() {
        let bitsetter = BitSetter::from_str(&"10X011XXX101X1").unwrap();
        assert_eq!(bitsetter.or_mask,    0b10001100010101);
        assert_eq!(bitsetter.and_mask,   0b10101111110111);
        assert_eq!(bitsetter.float_mask, 0b00100011100010);
    }

    fn test_set_bitsetter() {
        let n: u64 =                0b10100111010100;
        let bs = BitSetter::from_str("10X001X10XX101").unwrap();
        assert_eq!(bs.set_bits(n),  0b10100111010101);
        assert_eq!(bs.set_ones(n),  0b10100111010101);
    }

    fn test_float_iter() {
        let n: u64 =                0b10100011;
        let bs = BitSetter::from_str("10X001X1").unwrap();
        let mut fi = FloatBitIterator::new(&bs, n);
        assert_eq!(fi.next(),  Some(0b10100001)); 
        assert_eq!(fi.next(),  Some(0b10100011)); 
        assert_eq!(fi.next(),  Some(0b10000001)); 
        assert_eq!(fi.next(),  Some(0b10100001)); 
        assert_eq!(fi.next(),  None);    
    }
}
