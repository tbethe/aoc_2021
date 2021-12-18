use std::{fs};

/// Takes an iterator and a predicate that returns a bool. 
/// The predicate is applied to each element in the iterator.
/// Returns the number of times the predicate succeeded.
pub fn count<I, P>(iter: I, pred: P) -> usize
    where
        I: Iterator,
        P: Fn(I::Item) -> bool
{
        let mut count: usize = 0;
        for v in iter {
            if pred(v) { count += 1 }
        }
        count
}

/// Returns the (test)-input for a given day. 
/// If readings the file fails, the function will panic.
/// # Arguments
/// * `day` The day to get the input for
/// * `test_input` Whether or not the input should be the test input or the real input.
pub fn get_input(day: i32, test_input: bool) -> String {
    let file_path = match test_input {
        true => format!("input/day{}_test_input.txt", day),
        false => format!("input/day{}_input.txt", day)
    };
    fs::read_to_string(&file_path)
        .expect(format!("Could not open file for day {}. Path: {}.", day, file_path).as_str())
}