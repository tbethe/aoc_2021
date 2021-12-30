use crate::{utils, aoc_day::Aoc2021};

pub struct Day3 {
    // parsed to lines
    input: Vec<String>
    
}

impl Aoc2021 for Day3 {
    fn part1(&self) -> String {

        // Step 1. Get the width of the binary numbers
        let bit_width = self.input.first()
            .expect("The input should not be empty.")
            .len();

        // Step 2. For each position, count the number of 1's.
        // Step 3. map the count to majority.
        let majority = majority(&self.input, bit_width);
        // Step 4. Calculate the gamma rate (parse the integer)
        let gamma_rate = i32::from_str_radix(&majority, 2)
            .expect("We expected the input lines to only have binary strings.");
        // Step 5. Calculate the epsilon rate
        //      logical not (invert the bits)
        //      then get rid of the 1's at the beginning by shifting left and then right.
        //      it is important that we work with 32 bit integers
        let shift: i32 = (32 - bit_width).try_into().unwrap();
        let epsilon_rate = (!gamma_rate << shift) >> shift;
 

        format!(
            "Gamma rate: {}\nEpsilon rate: {}\nPower Consumption: {}",
            gamma_rate, epsilon_rate, gamma_rate * epsilon_rate
        )
    }

    fn part2(&self) -> String {
        let bit_width = self.input.first()
            .expect("The input should not be empty.")
            .len();
        let majority = majority(&self.input, bit_width);

        // Does a pass removing all binary strings that do not 
        // comply with the bit criteria at this index.
        fn pass<T: AsRef<str>>(input: &[T], index: usize, majority: String) -> &[T] {
            input.iter().filter(|s|{
                s.as_ref().cha == majority.chars().[index]
            }).collect();
        }
        
        let mut remainder = self.input.as_slice();
        for i in 0..bit_width {
            pass( remainder, i, majority);
            if remainder.len() <= 1 { break; }
        }
        
        todo!()
    }

    fn new(test_input: bool) -> Box<Self>
        where 
            Self : Sized {
        Box::new(
            Day3 { 
                input: utils::get_input(3, test_input)
                    .lines()
                    .map(|s|s.trim().to_string())
                    .collect::<Vec<String>>() 
            }
        )
    }
}

/// Given a vector of binary strings of width `bit_width`, this function returns
/// a String with on each index of that string a 1 if there was a majority of 1's at that index of the input string
/// 
/// Example:
/// ```rust
/// assert_!(
///     majority(vec!["100", "101", "000"]), "100".to_string()
/// )
/// ```
fn majority<T: AsRef<str>>(input: &[T], bit_width: usize) -> String {
    // For each position, count the number of 1's.
    let mut count_vec: Vec<usize> = vec![0; bit_width];    
    for line in input {
        for (i, chr) in line.as_ref().chars().enumerate() {
            if chr == '1' { 
                count_vec[i] += 1 
            }
        }
    }   
    // map the count to majority.
    let half = input.len() / 2;
    let mut majority = String::new();
    for count in count_vec {
        majority.push(if count < half { 
            '0'
        } else {
            '1'
        });
    }       
    majority
}