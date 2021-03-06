use CardinalDirection::*;
use Instruction::*;

fn main() {
    let instructions = parse_input("input.txt");
    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}

#[derive(Debug, Clone, Copy)]
enum CardinalDirection {
    East,
    South,
    West,
    North,
}

#[derive(Debug, Clone, Copy)]
struct Rotation(u8); // u8 encodes quarter right turns

impl Rotation {
    fn new(degrees: u32, left: bool) -> Option<Rotation> {
        if degrees % 90 != 0 { return None };
        let turns = ((degrees / 90) & 0x00000003) as u8;
        return Some(if left {
            Rotation((0x04 - turns) & 0x03)
        } else {
            Rotation(turns)
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Rotate(Rotation),
    Forward(u32),
    Translate((CardinalDirection, u32)),
}

fn parse_input(path: &str) -> Vec<Instruction> {
    std::fs::read_to_string(path).expect("Failed to open file").trim().lines()
    .map(|s| parse_line(s).expect(&format!("Failed to parse line: {}", s))).collect()
}

fn parse_line(line: &str) -> Option<Instruction> {
    let firstchar = line.chars().next()?;
    let magnitude = line[1..].parse::<u32>().ok()?;
    let instruction = match firstchar {
        'N' => Translate((North, magnitude)),
        'E' => Translate((East, magnitude)),
        'W' => Translate((West, magnitude)),
        'S' => Translate((South, magnitude)),
        'L' => Rotate(Rotation::new(magnitude, true)?),
        'R' => Rotate(Rotation::new(magnitude, false)?),
        'F' => Forward(magnitude),
        _ => { return None },
    };
    return Some(instruction)
}

static DIRECTIONS: [CardinalDirection; 4] = [
    East, South, West, North,
];

fn rotate_ship(direction: CardinalDirection, rot: Rotation) -> CardinalDirection {
    let offset = match direction {
        East => 0,
        South => 1,
        West => 2,
        North => 3,
    };
    DIRECTIONS[((offset + rot.0) % 4) as usize]
}

fn rotate_waypoint(x: i32, y: i32, rot: Rotation) -> (i32, i32) {
    match rot {
        Rotation(0) => (x, y),
        Rotation(1) => (y, -x),
        Rotation(2) => (-x, -y),
        Rotation(3) => (-y, x),
        _ => panic!("Not possible"),
    }
}

fn translate(x: i32, y: i32, direction: CardinalDirection, mag: u32) -> (i32, i32) {
    let mg = mag as i32;
    match direction {
        East => (x + mg, y),
        South => (x, y - mg),
        West => (x - mg, y),
        North => (x, y + mg),
    }
}

fn part1(instructions: &[Instruction]) -> i32 {
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut direction = East;
    for instruction in instructions {
        match instruction {
            Forward(z) => {
                let pair = translate(x, y, direction, *z);
                x = pair.0;
                y = pair.1;
            },
            Rotate(rotation) => {
                direction = rotate_ship(direction, *rotation)
            },
            Translate((d, z)) => {
                let pair = translate(x, y, *d, *z);
                x = pair.0;
                y = pair.1;
            },
        }
    }
    (x.abs() + y.abs()).into()
}

fn part2(instructions: &[Instruction]) -> i32 {
    let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut dx, mut dy): (i32, i32) = (10, 1);
    for instruction in instructions {
        match instruction {
            Forward(z) => {
                let mg = *z as i32;
                x += mg * dx;
                y += mg * dy;
            },
            Rotate(rotation) => {
                let pair = rotate_waypoint(dx, dy, *rotation);
                dx = pair.0;
                dy = pair.1;
            },
            Translate((d, z)) => {
                let pair = translate(dx, dy, *d, *z);
                dx = pair.0;
                dy = pair.1;
            },
        };
    }
    (x.abs() + y.abs()).into()
}

