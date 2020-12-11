pub fn parse_input(path: &str) -> Vec<u32> {
    let mut v: Vec<u32> = std::fs::read_to_string(path).expect("Failed to read file.").trim()
    .lines().map(|s| s.parse::<u32>().expect(&format!("Failed to parse {}", s))).collect();
    v.push(0);
    v.sort();
    v.push(v.last().unwrap() + 3);
    v
}

pub fn part1(nums: &[u32]) -> u32 {
    if nums.len() < 3 { return 0 };
    let mut last = nums.first().unwrap();
    let (mut diff1, mut diff3) = (0, 0);
    for n in nums[1..].iter() {
        diff1 += (n - last == 1) as u32;
        diff3 += (n - last == 3) as u32; 
        last = n;
    };
    diff1 * diff3
}

pub fn part2(nums: &[u32]) -> usize {
    if nums.len() < 3 { return 1 };
    let mut state = (1, 0, 0);
    let mut last = nums.last().unwrap();
    for n in nums[..nums.len()-1].iter().rev() {
        let diff = last - n;
        let (s1, s2, s3) = state;
        state = match diff {
            1 => (s1 + s2 + s3, s1, s2),
            2 => (s1 + s2, 0, s1),
            3 => (s1, 0, 0),
            _ => { return 0 },
        };
        last = n;
    }
    state.0
}
