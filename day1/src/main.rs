use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let numbers = read_line_file();
    for i in 0..numbers.len() - 2 {
        for j in i+1..numbers.len() - 1 {
            for k in j+1..numbers.len() {
                let (a, b, c) = (numbers[i], numbers[j], numbers[k]);
                if a + b + c == 2020 {
                    println!("{} + {} + {} = 2020", a, b, c);
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);                    
                }   
            }
        }
    }
}

fn read_line_file() -> Vec<i32> {
    let mut numbers = Vec::new();
    let file = File::open("input.txt").unwrap();
    for readline in io::BufReader::new(file).lines() {
        if let Ok(line) = readline {
            let n: i32 = line.trim().parse().unwrap();
            numbers.push(n)
        }
    }
    return numbers    
}
