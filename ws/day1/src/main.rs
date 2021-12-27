extern crate itertools;

use itertools::Itertools;
use std::{
    io::{self, BufRead},
    ops::Add,
};

fn to_three_value_window_sums<T: Add<Output = T> + Copy>(values: &[T]) -> Vec<T> {
    values
        .iter()
        .copied()
        .tuple_windows::<(T, T, T)>()
        .map(|(a, b, c)| a + b + c)
        .collect()
}

fn count_increases<T: Ord + Copy>(values: &[T]) -> usize {
    values
        .iter()
        .copied()
        .tuple_windows::<(T, T)>()
        .filter(|(a, b)| a < b)
        .count()
}

fn main() {
    let stdin = io::stdin();

    let input_values: Vec<i32> = stdin
        .lock()
        .lines()
        .map(|l| {
            l.expect("no I/O error")
                .parse::<i32>()
                .expect("integer input")
        })
        .collect();

    println!("Puzzle 1: {}", count_increases(&input_values));
    println!(
        "Puzzle 2: {}",
        count_increases(&to_three_value_window_sums(&input_values))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_value_window_sums_works() {
        assert_eq!(to_three_value_window_sums(&[1, 2, 3, 4]), vec![6, 9]);
    }

    #[test]
    fn count_increases_works() {
        assert_eq!(count_increases(&[1, 1, 2, 2, 3]), 2);
    }
}
