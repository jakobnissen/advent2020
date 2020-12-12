use CardinalDirection::*;

fn main() {
    let instructions = parse_input("input.txt");
    //println!("{:#?}", instructions);
    println!("{}", part1(&instructions));
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
    Rot(Rotation),
    Forward(u32),
    Cardinal((CardinalDirection, u32)),
}

fn parse_input(path: &str) -> Vec<Instruction> {
    std::fs::read_to_string(path).expect("Failed to open file").trim().lines()
    .map(|s| parse_line(s).expect(&format!("Failed to parse line: {}", s))).collect()
}

fn parse_line(line: &str) -> Option<Instruction> {
    let firstchar = line.chars().next()?;
    let magnitude = line[1..].parse::<u32>().ok()?;
    let instruction = match firstchar {
        'N' => Instruction::Cardinal((North, magnitude)),
        'E' => Instruction::Cardinal((East, magnitude)),
        'W' => Instruction::Cardinal((West, magnitude)),
        'S' => Instruction::Cardinal((South, magnitude)),
        'L' => Instruction::Rot(Rotation::new(magnitude, true)?),
        'R' => Instruction::Rot(Rotation::new(magnitude, false)?),
        'F' => Instruction::Forward(magnitude),
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
    /*
    2, 5
    5, -2
    -2, -5
    -5, 2
    */
    let negx = rot.0 & 0x02 == 0x02;
    let negy = (rot.0 == 0x01) | (rot.0 == 0x02);
    (if negx { -x } else { x }, if negy { -y } else { y } )
}

fn translate_ship(x: i32, y: i32, direction: CardinalDirection, mag: u32) -> (i32, i32) {
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
            Instruction::Forward(z) => {
                let pair = translate_ship(x, y, direction, *z);
                x = pair.0;
                y = pair.1;
            },
            Instruction::Rot(rotation) => {
                direction = rotate_ship(direction, *rotation)
            },
            Instruction::Cardinal((d, z)) => {
                let pair = translate_ship(x, y, *d, *z);
                x = pair.0;
                y = pair.1;
            },
        }
    }
    (x.abs() + y.abs()).into()
}

