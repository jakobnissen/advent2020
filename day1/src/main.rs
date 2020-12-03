use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let numbers = read_line_file();
    for i in 0..numbers.len() - 1 {
        for j in i+1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                let (a, b) = (numbers[i], numbers[j]);
                println!("{} + {} = 2020", a, b);
                println!("{} * {} = {}", a, b, a * b);
                
            }
        }
    }
}

fn read_line_file() -> Vec<i32> {
    let mut entered = Default::default();
    io::stdin().read_line(&mut entered).unwrap();

    let mut numbers = Vec::new();
    let file = File::open(entered.trim()).unwrap();
    for readline in io::BufReader::new(file).lines() {
        if let Ok(line) = readline {
            let n: i32 = line.trim().parse().unwrap();
            numbers.push(n)
        }
    }
    return numbers    
}
