use crate::{utils, aoc_day::Aoc2021};

pub struct Day3 {
    /// input parsed to u16
    input: Vec<u16>,
    /// number of bits of the numbers in the input
    bit_length: usize,
}

impl Aoc2021 for Day3 {

    fn part1(&self) -> String {

        let mut gamma_rate: u16 = 0;

        for b in 0..self.bit_length {
            
            let mut zeros = 0;
            let mut ones = 0;
            
            for int in &self.input {
                
                if (1 << b) & int != 0 {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }
            if ones > zeros {
                gamma_rate |= 1 << b; 
            }

        }

        let shift = 16 - self.bit_length;
        let epsilon_rate = (!gamma_rate << shift) >> shift;

        format!(
            "Gamma rate: {}\nEpsilon rate: {}\nPower Consumption: {}",
            gamma_rate, epsilon_rate, gamma_rate as usize * epsilon_rate as usize
        )
    }

    fn part2(&self) -> String {
        
        fn filter(
            mut input: Vec<u16>, 
            bit_length : usize,
            criteria: fn(u16, u16) -> bool
        ) -> u16 {
            for b in (0..bit_length).rev() {

                let majority_bit = majority_bit(&input, bit_length, b).unwrap();
                input = input.into_iter()
                    .filter( |i|
                        criteria(majority_bit.as_u16(), ((1 << b) & i) >> b)
                    ).collect();

                if input.len() == 1 {
                    return *input.first().unwrap()
                }
            }
            return 0
        }
        
        let copy = self.input.clone();
        let oxygen = filter(
            copy,
            self.bit_length,
            |maj , b|  maj == b
        );
        
        let copy = self.input.clone();
        let co2 = filter(
            copy,
            self.bit_length,
            |maj , b|  maj != b
        );

        format!(
            "Oxygen rating: {}\nO2 scrubber rating: {}\nLife support rating: {}",
            oxygen, co2, oxygen as usize * co2 as usize
        )
    }

    /// Returns day 3 struct
    /// Load input for day 3.
    /// 
    /// * `input` Input parsed to a vector of u16 integers. 
    /// * `bit_length` bit length of the binary numbers in the input
    fn new(test_input: bool) -> Box<Self>
        where 
            Self : Sized {
        let input_string = utils::get_input(3, test_input);
        let bit_length = input_string.split_once('\n')
            .expect("Input was unexpectedly empty.")
            .0
            .len();
        Box::new(
            Day3 { 
                input : input_string
                    .lines()
                    .map(|s| u16::from_str_radix(s, 2).unwrap())
                    .collect(),
                bit_length
            }
        )
    }
}

/// 
/// Bit, represents either a zero or a one
#[derive(PartialEq, Eq, Debug)]
enum Bit {
    Zero,
    One
}

impl Bit {
    fn as_u16(&self) -> u16 {
        match self {
            Bit::One => 1,
            Bit::Zero => 0,
        }
    }
}

/// gets an array of numbers, the width of those number and a bit position.
/// 
/// if `width` is larger than 16 or `position` is larger than `width - 1`, returns `None`.
/// Position of `0` means the least significant bit.
fn majority_bit(numbers: &[u16], width: usize, position: usize) -> Option<Bit> {
    
    if width > 16 || position > width - 1 {
        return None
    }

    let mut zeros = 0;
    let mut ones = 0;
    
    for int in numbers {
        
        if (1 << position) & *int != 0 {
            ones += 1;
        } else {
            zeros += 1;
        }
    }
    if ones >= zeros {
        Some(Bit::One) 
    } else {
        Some(Bit::Zero)
    }
}