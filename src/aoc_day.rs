//! Defines the trait used to create a day for Advent of Code 2021

/// Defines the behaviour a day of Advent of Code should have,
/// # Methods
/// 
/// * `part1` should return a string solving the first part of the advent of code 
/// * `part2` should return a string solving the first part of the advent of code 
/// * `new` should return a new boxed version of the type implementing this trait. 
///         This is purely for convinience in the main method.
pub(crate) trait Aoc2021 {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
    
    fn new(test_input: bool) -> Box<Self>
        where 
            Self : Sized;
}