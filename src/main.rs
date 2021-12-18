mod aoc_day;
mod utils;
mod day1;
mod day2;
mod day3;

extern crate clap;
use clap::{App, Arg, value_t_or_exit};
use aoc_day::Aoc2021;

fn main() {
    // parse commandline arguments using clap
    let matches = App::new("Advent of Code")
                        .author("T. Bethe")
                        .about("Advent of Code 2021")
                        .arg(Arg::with_name("test")
                            .short("t")
                            .help("Uses the test input instead of real input.")
                        )
                        .arg(Arg::with_name("day")
                            .required(true)
                            .index(1)
                            .help("The day that should be executed.")
                        )
                        .get_matches();

    // argument is required, so we can unwrap it
    let day : usize = value_t_or_exit!(matches.value_of("day"), usize);

    let use_test_input : bool = matches.is_present("test");

    let days: Vec<Box<dyn Aoc2021>> = vec![
        day1::Day1::new(use_test_input),
        day2::Day2::new(use_test_input),
    ];

    let aoc_day = days.get(day - 1 ).unwrap_or_else(|| {
        println!(
            "{} Day not found.\n Day {} has not been implemented (yet).",
            ::clap::Format::Error("error:"),
            day
        );
        ::std::process::exit(1);
    });

    print_line();
    println!("#                   Day {:2}                   #", day);
    if use_test_input { 
        println!("#             (using test input)             #");
    }

    print_part(1);
    print_line();
    println!();
    println!("{}", aoc_day.part1());
    println!();
    print_line();

    print_part(2);
    print_line();
    println!();
    println!("{}", aoc_day.part2());
    println!();
    print_line();

}

fn print_line() {
    println!("#--------------------------------------------#");
}

fn print_part(part: i32) {
    println!("#                   Part {}                   #", part);
}