use crate::aoc_day::Aoc2021;
use crate::utils;

pub struct Day1 {
    // parsed input
    input: Vec<i32>
}

impl Aoc2021 for Day1 {

    fn part1(&self) -> String {
        let count_increased = utils::count(
            self.input.windows(2), 
            |w| { w[0] < w[1]
        });
        format!("Increased {} times", count_increased)
    }

    fn part2(&self) -> String {
        // Notice, A + B + C < B + C + D <==> A < D
    let sums_increased = utils::count(
        self.input.windows(4),
        |w| {
            w[0] < w[3]
        }
    );
    format!("Sums increased {} times", sums_increased)
    }

    fn new(test_input: bool) -> Box<Self> {
        let contents = utils::get_input(1, test_input);
        let contents: Vec<i32> = contents.split_whitespace().map(|a| a.parse().unwrap()).collect();
        Box::new(Day1 { 
            input : contents
         })
    }
}