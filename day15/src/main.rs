fn main() {
    let input = [20,9,11,0,1,2];
    let mut turns_of = initialize_vec(&input);
    let mut number: usize = *input.last().unwrap();
    for turn in (input.len() + 1).. {
        let (turn_m2, turn_m1) = turns_of[number];
        number = if turn_m2 == 0 {0} else {turn_m1 - turn_m2};
        if turns_of.len() < number + 1 {
            if turns_of.len() < number {
                    turns_of.resize(number, (0, 0));
            }
            turns_of.push((0, turn));
        } else {
            let (_, turn_m1) = turns_of[number];
            turns_of[number] = (turn_m1, turn)    
        }

        if turn == 2020 {
            println!("Turn 2020: {}", number);
        } else if turn == 30_000_000 {
            println!("Turn 30,000,000: {}", number);
            break
        }
    }
    
}

fn initialize_vec(v: &[usize]) -> Vec<(usize, usize)> {
    let max = v.iter().max().expect("Cannot initialize zero-length vector.");
    let mut res: Vec<(usize, usize)> = vec![(0, 0); max + 1];
    v.iter().enumerate().for_each(|(i, n)| {res[*n] = (0, i + 1)});
    res
}
