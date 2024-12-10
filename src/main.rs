mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a number as an argument.");
        return;
    }

    match args[1].parse::<i32>() {
        Ok(number) => run_selected_day(number),
        Err(_) => println!("The provided argument is not a valid number."),
    }
}

fn run_selected_day(day: i32) {
    match day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        8 => day8::run(),
        9 => day9::run(),
        10 => day10::run(),
        _ => println!("no solution for that day yet!"),
    }
}
