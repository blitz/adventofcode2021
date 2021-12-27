#![feature(result_flattening)]

extern crate itertools;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    error::{ParseError, VerboseError},
    sequence::preceded,
    IResult,
};
use std::{
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    take_while(move |c| chars.contains(c))(i)
}

fn integer<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, i32, E> {
    digit1(i).map(|(next_input, res)| {
        (
            next_input,
            i32::from_str(res).expect("failed to parse integer"),
        )
    })
}

fn parse_direction<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Direction, E> {
    alt((tag("forward"), tag("down"), tag("up")))(i)
        .map(|(next_input, res)| {
            preceded(space, integer)(next_input).map(|(next_input, int)| {
                (
                    next_input,
                    match res {
                        "forward" => Direction::Forward(int),
                        "down" => Direction::Down(int),
                        "up" => Direction::Up(int),
                        _ => panic!("unknown direction"),
                    },
                )
            })
        })
        .flatten()
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_direction::<VerboseError<&str>>(s)
            .map(|(_, i)| i)
            .map_err(|_| ())
    }
}

fn to_coordinates(d: Direction) -> (i32, i32) {
    match d {
        Direction::Forward(i) => (i, 0),
        Direction::Down(i) => (0, i),
        Direction::Up(i) => (0, -i),
    }
}

fn main() {
    let stdin = io::stdin();

    let directions: Vec<Direction> = stdin
        .lock()
        .lines()
        .map(|l| l.expect("I/O error"))
        .map(|s| Direction::from_str(&s).expect("parse error"))
        .collect();

    // Part 1
    let coord = directions
        .iter()
        .copied()
        .map(to_coordinates)
        .fold((0, 0), |(acc_0, acc_1), (c_0, c_1)| {
            (acc_0 + c_0, acc_1 + c_1)
        });

    println!("Coordinate: {:?}", coord);
    println!("Answer 1: {}", coord.0 * coord.1);

    // Part 2
    let coord = directions
        .iter()
        .copied()
        .fold((0, 0, 0), |(acc_h, acc_d, acc_a), d| match d {
            Direction::Forward(i) => (acc_h + i, acc_d + acc_a * i, acc_a),
            Direction::Down(i) => (acc_h, acc_d, acc_a + i),
            Direction::Up(i) => (acc_h, acc_d, acc_a - i),
        });

    println!("Coordinate: {:?}", coord);
    println!("Answer 2: {}", coord.0 * coord.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
