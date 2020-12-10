use std::cmp::{Ord, PartialEq};
use std::ops::Add;

fn main() {
    let numbers: Vec<isize> = {
        let string = std::fs::read_to_string("input.txt").unwrap();
        let result: Result<Vec<isize>, std::num::ParseIntError> = 
        string.trim().lines().map(|s| s.parse::<isize>()).collect();
        result.unwrap()
    };
    let first_bad_number = numbers[first_bad_number(&numbers, 25).unwrap()];
    println!("First bad number: {}", first_bad_number);
    let contiguous = find_contiguous(&numbers, first_bad_number).unwrap();
    let sum = extrema_sum(contiguous).expect("Could not find extrema of slice");
    println!("Sum of extrema: {}", sum);
}

fn extrema_sum<T>(v: &[T]) -> Option<T>
where T: Copy + Ord + Add<T, Output=T>,
{
    Some(*v.iter().max()? + *v.iter().min()?)
}

fn find_contiguous<T>(v: &[T], target: T) -> Option<&[T]>
where T: Add<T, Output=T> + PartialEq + Copy + Ord, 
{
    for i in 0..(v.len()-1) {
        let mut s = v[i];
        for j in (i+1)..v.len() {
            s = s + v[j];
            if s == target {
                return Some(&v[i..j+1])
            } else if s > target {
                break;
            }
        }
    }
    return None
}

fn first_bad_number<T>(v: &[T], preamble: usize) -> Option<usize>
where T: Add<T, Output=T> + PartialEq + Copy,
{
    for i in preamble..v.len() {
        if !number_passes(v, i, preamble) {
            return Some(i)
        }
    }
    return None
}

fn number_passes<T>(v: &[T], index:usize, preamble: usize) -> bool
where T: PartialEq + Add<T, Output=T> + Copy,
{
    if preamble < 2 {
        panic!("Preamble must be 2 or more")
    }
    let target = v[index];
    for i in (index-preamble)..(index-1) {
        let v1 = v[i];
        for j in (i+1)..index {
            let v2 = v[j];
            if v1 + v2 == target {
                return true
            }
        }
    }
    false
}
