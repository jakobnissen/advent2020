use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {println!("Usage: day6 [FILE]"); exit(1)});
    partx(path.trim(), 0, &|a, b| a | b);
    partx(path.trim(), u32::MAX, &|a, b| a & b);
}

struct LineSepIterator<T: BufRead> {
    io: T,
    linebuf: String
}

impl <T: BufRead> Iterator for LineSepIterator<T> {
    type Item = Result<Vec<String>, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.linebuf.clear();
        loop {
            let nread_res = self.io.read_line(&mut self.linebuf);
            let nread = match nread_res {
                Err(e) => return Some(Err(e)),
                Ok(o) => o
            };
            if nread > 1 {
                continue;
            }
            if !self.linebuf.is_empty() {
                return Some(Ok(self.linebuf.trim().split('\n').map(|x| x.to_string()).collect()))
            }
            if nread == 0 {
                return None
            } else if nread == 1 {
                self.linebuf.clear();
            }
        }
    }
}

impl<T: BufRead> LineSepIterator<T> {
    fn new(io: T) -> LineSepIterator<T> {
        LineSepIterator{io: io, linebuf: String::new()}
    }
}

impl LineSepIterator<BufReader<File>> {
    fn from_path(path: &str) -> Result<LineSepIterator<BufReader<File>>, std::io::Error> {
        let file = File::open(path)?;   
        Ok(LineSepIterator::new(BufReader::new(file)))
    }
}

fn partx<F: std::ops::Fn(u32, u32) -> u32>(path: &str, init: u32, op: &F) -> () {
    let reader = LineSepIterator::from_path(path)
    .unwrap_or_else(|_| {println!("Could not open file"); exit(1)});
    let n: u32 = reader.map(|chunks| {
        chunks.unwrap_or_else(|_| {println!("Could not read line"); exit(1)})
        .iter().map(|line| {
            parse_bitset(line)
        }).fold(init, op).count_ones()
    }).sum();
    println!("{}", n);
}

fn parse_bitset(string: &str) -> u32 {
    let mut result: u32 = 0;
    for chr in string.chars() {
        let shift = match chr {
            'a'..='z' => (chr as u32) - ('a' as u32) ,
            _ => panic!(format!("Unrecognized char: {}", chr))
        };
        result |= 1u32 << shift;
    }
    return result
}
