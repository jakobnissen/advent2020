// Todo: Make a simple Matrix type

fn main() {
    let mut layout = parse_layout("input.txt");
    let mut neighbors: Vec<Vec<u8>> = Vec::new();
    let (nrows, ncols) = (layout.len(), layout[1].len());
    for _row in 0..nrows {
        neighbors.push(vec![0; ncols])
    }

    println!("{}", part1(&mut layout, &mut neighbors));
    empty_seats(&mut layout);
    println!("{}", part2(&mut layout, &mut neighbors));
}

fn print_layout(layout: &[Vec<Seat>]) {
    for row in layout.iter() {
        for seat in row.iter() {
            let char = match seat {
                Seat::Empty => 'L',
                Seat::Floor => '.',
                Seat::Occupied => '#',
            };
            print!("{}", char);
        }
        println!();
    }
}

fn part1(layout: &mut [Vec<Seat>], neighbors: &mut [Vec<u8>]) -> u32 {
    let mut changes = 1;
    while changes != 0 {
        fill_neighbors_part1(layout, neighbors);
        changes = iterate(layout, neighbors, 3)
    }
    count_occupied(&layout)
}

fn part2(layout: &mut [Vec<Seat>], neighbors: &mut [Vec<u8>]) -> u32 {
    let mut changes = 1;
    while changes != 0 {
        fill_neighbors_part2(&layout, neighbors);
        changes = iterate(layout, &neighbors, 4);
    }
    count_occupied(&layout)
}

fn count_occupied(layout: &[Vec<Seat>]) -> u32 {
    layout.iter().map(|row| {
        row.iter().map(|seat| {
            match seat {
                Seat::Occupied => 1,
                _ => 0,
            }
        }).sum::<u32>()
    }).sum::<u32>()
}

#[derive(Debug, Clone, Copy)]
enum Seat {
    Floor,
    Occupied,
    Empty,
}

fn parse_layout(path: &str) -> Vec<Vec<Seat>>  {
    let mut result = Vec::new();
    for line in std::fs::read_to_string(path).expect("Failed to read file").trim().lines() {
        let mut row: Vec<Seat> = Vec::new();
        for char in line.chars() {
            let seat = match char {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                '#' => Seat::Occupied,
                _ => panic!(format!("Unparseable char: '{}'", char))
            };
            row.push(seat)
        }
        result.push(row);
    }
    result
}

fn zero_neighbors(neighbors: &mut [Vec<u8>]) {
    let (nrows, ncols) = (neighbors.len(), neighbors[1].len());
    for row in 0..nrows {
        for col in 0..ncols {
            neighbors[row][col] = 0;
        }
    }
}

fn empty_seats(layout: &mut [Vec<Seat>]) {
    let (nrows, ncols) = (layout.len(), layout[1].len());
    for row in 0..nrows {
        for col in 0..ncols {
            if let Seat::Occupied = layout[row][col] {
                layout[row][col] = Seat::Empty
            }
        }
    }
}

fn fill_neighbors_part2(layout: &[Vec<Seat>], neighbors: &mut [Vec<u8>]) {
    let (nrows, ncols) = (layout.len(), layout[1].len());
    zero_neighbors(neighbors);

    for row in 0..nrows {
        for col in 0..ncols {
            for dy in -1i32..=1i32 {
                for dx in -1i32..=1i32 {
                    if dy == 0 && dx == 0 { continue };
                    let mut x = col as i32 + dx;
                    let mut y = row as i32 + dy;
                    while (x >= 0) && (x < ncols as i32) && (y >= 0) && (y < nrows as i32) {
                        match layout[y as usize][x as usize] {
                            Seat::Empty => {break},
                            Seat::Occupied => {neighbors[row][col] += 1; break},
                            _ => (),
                        };
                        x += dx;
                        y += dy;
                    }
                }
            }
        }
    }
}

fn fill_neighbors_part1(layout: &[Vec<Seat>], neighbors: &mut [Vec<u8>]) {
    let (nrows, ncols) = (layout.len(), layout[1].len());
    zero_neighbors(neighbors);
    
    for row in 0..nrows {
        for col in 0..ncols {
            if let Seat::Occupied = layout[row][col] {
                let rowbefore = row != 0;
                let rowafter = row != nrows - 1;
                let colbefore = col != 0;
                let colafter = col != ncols - 1;
                if colbefore {
                    neighbors[row][col - 1] += 1;
                };
                if colafter {
                    neighbors[row][col + 1] += 1;
                };
                if rowbefore {
                    neighbors[row - 1][col] += 1;
                    if colbefore {
                        neighbors[row - 1][col - 1] += 1;        
                    }
                    if colafter {
                        neighbors[row - 1][col + 1] += 1;    
                    }
                };
                if rowafter {
                    neighbors[row + 1][col] += 1;
                    if colbefore {
                        neighbors[row + 1][col - 1] += 1;        
                    }
                    if colafter {
                        neighbors[row + 1][col + 1] += 1;    
                    }
                };                
            };
        };
    };
}

// Returns number of changed seats
fn iterate(layout: &mut [Vec<Seat>], neighbors: &[Vec<u8>], tolerate: u8) -> u32 {
    let (nrows, ncols) = (layout.len(), layout[1].len());
    let mut changes: u32 = 0;
    for row in 0..nrows {
        for col in 0..ncols {
            let n_neighbors = neighbors[row][col];
            let oldseat = layout[row][col];
            let newseat = match oldseat {
                Seat::Empty => {
                    if n_neighbors == 0 {
                        changes += 1;
                        Seat::Occupied
                    } else { Seat::Empty }
                },
                Seat::Occupied => {
                    if n_neighbors > tolerate {
                        changes += 1;
                        Seat::Empty
                    } else { Seat::Occupied }
                },
                _ => oldseat
            };
            layout[row][col] = newseat
        }
    }
    changes       
}
