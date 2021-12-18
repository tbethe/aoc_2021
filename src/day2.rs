//! The solution to this day's Advent of Code is actually found
//!  in the `new` function of the implementation of `Aoc2021`

use crate::{utils, aoc_day::Aoc2021};


/*
 * I think my code was fine when we were just looking at the first part.
 * The only thing I would improve is to not create a new struct, but
 * update the struct instead.
 * The second part made my code a bit more ugly, because I did not want
 * to refactor it, so I reused the Position struct with an extra field.
 * This makes it a bit messy.
 */

pub struct Day2 {
    part1: Position,
    part2: Position,
}

impl Aoc2021 for Day2 {
    fn part1(&self) -> String {
        format!("Final position: {}", self.part1.get_multiplication())
    }

    fn part2(&self) -> String {
        format!("Final position: {}", self.part2.get_multiplication())
    }

    fn new(test_input: bool) -> Box<Self>
        where 
            Self : Sized {
                let input = utils::get_input(2, test_input);
                let mut pos_part_1 = Position {
                    depth: 0,
                    horizontal: 0,
                    aim: 0,
                };
                let mut pos_part_2 = Position {
                    depth: 0,
                    horizontal: 0,
                    aim: 0,
                };
                for line in input.split('\n') {
                    let line : Vec<&str> = line.split_whitespace().collect();
                    
                    // We are expecting two arguments per line.
                    // If this is not the case, we ignore the line.
                    if line.len() != 2 { continue; }
                    
                    let command = line[0];
                    let x: isize = line[1].parse().unwrap();
                    
                    match parse_direction(command) {
                        Some(Direction::Down) => {
                            pos_part_1 = pos_part_1.down(x);
                            pos_part_2 = pos_part_2.down_part_2(x);
                        }
                        Some(Direction::Up) => {
                            pos_part_1 = pos_part_1.up(x);
                            pos_part_2 = pos_part_2.up_part_2(x);
                        }
                        Some(Direction::Forward) => {
                            pos_part_1 = pos_part_1.forward(x);
                            pos_part_2 = pos_part_2.forward_part_2(x);
                        }
                        None => panic!("Could not parse input"),
                    }
                }

        Box::new(
            Day2 { 
                part1 : pos_part_1,
                part2 : pos_part_2
            }
        )
    }
}

 struct Position {
    depth: isize,
    horizontal: isize,
    aim: isize,
}

impl Position {
    pub fn get_multiplication(&self) -> isize {
        self.depth * self.horizontal
    }

    pub fn forward(self, x: isize) -> Position {
        Position {
            horizontal : self.horizontal + x,
            ..self
        }
    }

    pub fn forward_part_2(self, x: isize) -> Position {
        Position {
            horizontal : self.horizontal + x,
            depth : self.depth + self.aim * x, 
            ..self
        }
    }

    pub fn down(self, x: isize) -> Position {
        Position {
            depth : self.depth + x, 
            ..self
        }
    }

    pub fn down_part_2(self, x: isize) -> Position {
        Position {
            aim : self.aim + x,
            ..self
        }
    }

    pub fn up(self, x: isize) -> Position {
        Position {
            depth : self.depth - x, 
            ..self
        }
    }

    pub fn up_part_2(self, x: isize) -> Position {
        Position {
            aim: self.aim - x,
            ..self
        }
    }
}

enum Direction {
    Forward,
    Up,
    Down,
}

fn parse_direction(direction: &str) -> Option<Direction> {
    match direction {
        "forward" => Some(Direction::Forward),
        "up" => Some(Direction::Up),
        "down" => Some(Direction::Down),
        _ => None
    }
}
