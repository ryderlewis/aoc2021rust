mod day1;

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
    let day: i8 = day.unwrap();

    let part = args[2].parse();
    if part.is_err() {
        print_usage(&args);
    }
    let part: i8 = part.unwrap();
    if part < 1 || part > 2 {
        print_usage(&args);
    }

    match day {
        1 => day1::run(part),
        /*
        2 => day2(),
        3 => day3(),
        4 => day4(),
        5 => day5(),
        6 => day6(),
        7 => day7(),
        8 => day8(),
        9 => day9(),
        10 => day10(),
        11 => day11(),
        12 => day12(),
        13 => day13(),
        14 => day14(),
        15 => day15(),
        16 => day16(),
        17 => day17(),
        18 => day18(),
        19 => day19(),
        20 => day20(),
        21 => day21(),
        22 => day22(),
        23 => day23(),
        24 => day24(),
        25 => day25(),
         */
        _ => print_usage(&args),
    }
}

fn print_usage(args: &Vec<String>) {
    println!("Usage: {} [day] [part], where day is a number between 1 and 25, and part is 1 or 2", args[0]);
    exit(1);
}