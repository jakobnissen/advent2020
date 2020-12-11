use day10::{part1, part2, parse_input};

fn main() {
    let v = parse_input("input.txt");
    println!("{}", part1(&v));
    println!("{}", part2(&v));
}
