fn main() {
    let (start_time, periods, pos) = parse_input("input.txt");
    println!("{}", part1(start_time, &periods));
    println!("{}", part2(&periods, &pos));
}

fn part2(periods: &[u64], pos: &[u64]) -> u64 {
    let mut jump = *periods.iter().next().unwrap();
    let mut start = 0;
    for (d, p) in periods[1..].iter().zip(pos[1..].iter()) {
        while start % d != d - p % d {
            start += jump;
        }
        jump *= d;
    }
    start
}

fn part1(start: u64, periods: &[u64]) -> u64 {
    for time in start.. {
        if let Some(b) = periods.iter().copied().find(|b| time % b == 0) {
            return b * (time - start)
        }
    }
    panic!("No solution for day 1");
}

fn parse_input(path: &str) -> (u64, Vec<u64>, Vec<u64>) {
    let string = std::fs::read_to_string(path).unwrap();
    let mut lines = string.trim().lines();
    let start: u64 = lines.next().unwrap().parse().unwrap();
    let fields: Vec<&str> = lines.next().unwrap().split(',').collect();
    let nums = fields.iter().filter(|s| *s != &"x").map(|s| s.parse::<u64>().unwrap()).collect();
    let pos = fields.iter().enumerate().filter(|p| p.1 != &"x").map(|p| p.0 as u64).collect();
    (start, nums, pos)
}
