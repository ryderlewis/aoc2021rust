#![allow(dead_code)]

mod day2101;
mod day2102;
mod day2103;
mod day2104;
mod day2105;
mod day2106;
mod day2107;
mod day2108;
mod day2109;
mod day2110;
mod day2111;
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
        2101 => day2101::run(part),
        2102 => day2102::run(part),
        2103 => day2103::run(part),
        2104 => day2104::run(part),
        2105 => day2105::run(part),
        2106 => day2106::run(part),
        2107 => day2107::run(part),
        2108 => day2108::run(part),
        2109 => day2109::run(part),
        2110 => day2110::run(part),
        2111 => day2111::run(part),
        2201 => day2201::run(part),
        2202 => day2202::run(part),
        2203 => day2203::run(part),
        _ => print_usage(&args),
    }
}

fn print_usage(args: &Vec<String>) {
    println!("Usage: {} [day] [part], where day is a number between 2101-2125 or 2201-2225, and part is 1 or 2", args[0]);
    exit(1);
}