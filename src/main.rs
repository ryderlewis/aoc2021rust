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
mod day2112;
mod day2113;
mod day2114;
mod day2115;
mod day2116;
mod day2117;
mod day2118;
mod day2119;
mod day2120;
mod day2121;
mod day2122;
mod day2123;
mod day2124;
mod day2125;
mod day2201;
mod day2202;
mod day2203;
mod day2204;
mod day2205;
mod day2206;
mod day2207;
mod day2208;
mod day2209;
mod day2210;
mod day2211;
mod day2212;
mod day2213;
mod day2214;
mod day2215;
mod day2216;
mod day2217;
mod day2218;
mod day2219;
mod day2220;
mod day2221;
mod day2222;
mod day2223;
mod day2224;
mod day2225;

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
        2112 => day2112::run(part),
        2113 => day2113::run(part),
        2114 => day2114::run(part),
        2115 => day2115::run(part),
        2116 => day2116::run(part),
        2117 => day2117::run(part),
        2118 => day2118::run(part),
        2119 => day2119::run(part),
        2120 => day2120::run(part),
        2121 => day2121::run(part),
        2122 => day2122::run(part),
        2123 => day2123::run(part),
        2124 => day2124::run(part),
        2125 => day2125::run(part),
        2201 => day2201::run(part),
        2202 => day2202::run(part),
        2203 => day2203::run(part),
        2204 => day2204::run(part),
        2205 => day2205::run(part),
        2206 => day2206::run(part),
        2207 => day2207::run(part),
        2208 => day2208::run(part),
        2209 => day2209::run(part),
        2210 => day2210::run(part),
        2211 => day2211::run(part),
        2212 => day2212::run(part),
        2213 => day2213::run(part),
        2214 => day2214::run(part),
        2215 => day2215::run(part),
        2216 => day2216::run(part),
        2217 => day2217::run(part),
        2218 => day2218::run(part),
        2219 => day2219::run(part),
        2220 => day2220::run(part),
        2221 => day2221::run(part),
        2222 => day2222::run(part),
        2223 => day2223::run(part),
        2224 => day2224::run(part),
        2225 => day2225::run(part),
        _ => print_usage(&args),
    }
}

fn print_usage(args: &Vec<String>) {
    println!("Usage: {} [day] [part], where day is a number between 2101-2125 or 2201-2225, and part is 1 or 2", args[0]);
    exit(1);
}