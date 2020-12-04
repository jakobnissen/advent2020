use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let rows = parse_rows();
    println!("{:?}", traverse(&rows))
}

fn traverse(vecs: &Vec<Vec<bool>>) -> i32 {
    let mut n = 0;
    let mut xpos = 0;
    for vec in vecs {
        let p: i32 = vec[xpos].into();
        n += p;
        xpos = (xpos + 3) % vec.len();
    }
    return n
}

fn parse_rows() -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    let file = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    for lineresult in lines {
        let row = parse_row(&lineresult.unwrap());
        result.push(row)
    }

    // Validate all are same length
    if !all_same_length(&result) {
        panic!("Not all vectors are same length")
    }

    return result
}

fn all_same_length<T>(vecs: &Vec<Vec<T>>) -> bool {
    if vecs.len() == 0 {
        return true
    }
    let mut iter = vecs.into_iter();
    let firstlen = iter.next().unwrap().len();
    return iter.all(|x| x.len() == firstlen)
}

fn parse_row(row: &str) -> Vec<bool> {
    let mut result = Vec::new();
    for chr in row.chars() {
        let bl = match chr {
            '#' => true,
            '.' => false,
            _ => panic!("Unknown char: {}", chr)
        };
        result.push(bl)
    }
    return result
}
