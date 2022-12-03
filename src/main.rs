#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day2201;
mod day2202;
mod day2203;

use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        print_usage(&args);
    }

    let day = args[1].parse();
    if day.is_err() {
        print_usage(&args);
    }
    let day = day.unwrap();

    let part = args[2].parse();
    if part.is_err() {
        print_usage(&args);
    }
    let part = part.unwrap();
    if part < 1 || part > 2 {
        print_usage(&args);
    }

    match day {
        1 => day1::run(part),
        2 => day2::run(part),
        3 => day3::run(part),
        4 => day4::run(part),
        5 => day5::run(part),
        6 => day6::run(part),
        7 => day7::run(part),
        8 => day8::run(part),
        9 => day9::run(part),
        2201 => day2201::run(part),
        2202 => day2202::run(part),
        2203 => day2203::run(part),
        _ => print_usage(&args),
    }
}

fn print_usage(args: &Vec<String>) {
    println!("Usage: {} [day] [part], where day is a number between 1 and 25, and part is 1 or 2", args[0]);
    exit(1);
}