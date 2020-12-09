fn main() {
    let mut instructions = parse_instructions("input.txt");
    let mut visited = vec![false; instructions.len()];
    println!("{}", part1(&instructions, &mut visited).1);
    println!("{}", part2(&mut instructions, &mut visited));
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop,
    Jmp,
    Acc,
}
#[derive(Debug, Clone, Copy)]
struct Command(Instruction, i32);

fn parse_instructions(path: &str) -> Vec<Command> {
    std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.split(' '))
        .map(|mut fields| {
            let instruction = match fields.next().unwrap() {
                "nop" => Instruction::Nop,
                "jmp" => Instruction::Jmp,
                "acc" => Instruction::Acc,
                _ => panic!(),
            };
            Command(instruction, fields.next().unwrap().parse::<i32>().unwrap())
        })
        .collect()
}

// Returns completed_successfully, acc
fn part1(instructions: &[Command], visited: &mut [bool]) -> (bool, i32) {
    for i in 0..visited.len() {
        visited[i] = false
    }
    let mut pointer: usize = 0;
    let mut acc = 0;
    loop {
        if pointer == visited.len() {
            return (true, acc);
        }
        if visited[pointer] {
            return (false, acc);
        }
        visited[pointer] = true;
        match instructions[pointer] {
            Command(Instruction::Nop, _) => pointer += 1,
            Command(Instruction::Jmp, x) => pointer = ((pointer as i32) + x) as usize,
            Command(Instruction::Acc, x) => {
                acc += x as i32;
                pointer += 1
            }
        }
    }
}

fn part2(instructions: &mut [Command], visited: &mut [bool]) -> i32 {
    for i in 0..instructions.len() {
        let oldinstr = instructions[i];
        let newinstr = match oldinstr {
            Command(Instruction::Acc, _) => continue,
            Command(Instruction::Nop, x) => Command(Instruction::Jmp, x),
            Command(Instruction::Jmp, x) => Command(Instruction::Nop, x),
        };
        instructions[i] = newinstr;
        let (terminated, acc) = part1(instructions, visited);
        instructions[i] = oldinstr;
        if terminated {
            return acc;
        }
    }
    panic!("No changes could save the program");
}
