use std::cmp::{max, Ordering};
use std::ops::RangeInclusive;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let sensors = Sensor::parse();
    let mut ranges: Vec<RangeInclusive<i32>> = sensors.iter().map(|s| s.beacon_not_present(2_000_000)).flatten().collect();
    ranges = compact(&ranges);
    println!("{:?}", ranges);
}

fn part2() {
    let sensors = Sensor::parse();
    for y in 0..=4_000_000 {
        let mut ranges: Vec<RangeInclusive<i32>> = sensors.iter().map(|s| s.beacon_not_present(y)).flatten().collect();
        ranges = compact(&ranges);

        if ranges.len() > 1 {
            println!("{}, {:?}", y, ranges);
            let x = (ranges[0].end() + 1) as i64;
            let y = y as i64;
            println!("{}", x * 4_000_000 + y);
        }
    }
}

fn compact(v: &Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    let mut v = v.clone();

    v.sort_by(|a, b| {
        match a.start().cmp(b.start()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match a.end().cmp(b.end()) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            }
        }
    });

    let mut ret = vec![];
    let mut last_range: Option<RangeInclusive<i32>> = None;

    for r in &v {
        if last_range.is_none() {
            last_range = Some(r.clone());
            continue;
        }

        if last_range.as_ref().unwrap().end() >= r.start() {
            last_range = Some((*last_range.as_ref().unwrap().start())..=(max(*last_range.as_ref().unwrap().end(), *r.end())))
        } else {
            ret.push(last_range.unwrap());
            last_range = None;
        }
    }

    if last_range.is_some() {
        ret.push(last_range.unwrap());
    }

    ret
}

#[derive(Debug)]
struct Coord(i32, i32);

#[derive(Debug)]
struct Sensor {
    location: Coord,
    beacon: Coord,
}

impl Sensor {
    fn parse() -> Vec<Self> {
        let mut v = vec![];

        for line in input_real().lines() {
            let line = line.trim();
            let line = line.replace("Sensor at x=", "");
            let line = line.replace(": closest beacon is at x=", " ");
            let line = line.replace(", y=", " ");
            let parts: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

            v.push(Self {
                location: Coord(parts[0], parts[1]),
                beacon: Coord(parts[2], parts[3]),
            })
        }

        v
    }

    fn beacon_not_present(&self, row: i32) -> Option<RangeInclusive<i32>> {
        let mdist = i32::abs(self.location.0 - self.beacon.0) + i32::abs(self.location.1 - self.beacon.1);
        let vert = i32::abs(row-self.location.1);
        let max_horiz = mdist - vert;

        if max_horiz < 0 {
            None
        } else {
            Some(self.location.0-max_horiz..=self.location.0+max_horiz)
        }
    }
}

fn input_test() -> &'static str {
    r###"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "###.trim()
}

fn input_real() -> &'static str {
    r###"
Sensor at x=2983166, y=2813277: closest beacon is at x=3152133, y=2932891
Sensor at x=2507490, y=122751: closest beacon is at x=1515109, y=970092
Sensor at x=3273116, y=2510538: closest beacon is at x=3152133, y=2932891
Sensor at x=1429671, y=995389: closest beacon is at x=1515109, y=970092
Sensor at x=2465994, y=2260162: closest beacon is at x=2734551, y=2960647
Sensor at x=2926899, y=3191882: closest beacon is at x=2734551, y=2960647
Sensor at x=1022491, y=1021177: closest beacon is at x=1515109, y=970092
Sensor at x=1353273, y=1130973: closest beacon is at x=1515109, y=970092
Sensor at x=1565476, y=2081049: closest beacon is at x=1597979, y=2000000
Sensor at x=1841125, y=1893566: closest beacon is at x=1597979, y=2000000
Sensor at x=99988, y=71317: closest beacon is at x=86583, y=-1649857
Sensor at x=3080600, y=3984582: closest beacon is at x=3175561, y=4138060
Sensor at x=3942770, y=3002123: closest beacon is at x=3724687, y=3294321
Sensor at x=1572920, y=2031447: closest beacon is at x=1597979, y=2000000
Sensor at x=218329, y=1882777: closest beacon is at x=1597979, y=2000000
Sensor at x=1401723, y=1460526: closest beacon is at x=1515109, y=970092
Sensor at x=2114094, y=985978: closest beacon is at x=1515109, y=970092
Sensor at x=3358586, y=3171857: closest beacon is at x=3152133, y=2932891
Sensor at x=1226284, y=3662922: closest beacon is at x=2514367, y=3218259
Sensor at x=3486366, y=3717867: closest beacon is at x=3724687, y=3294321
Sensor at x=1271873, y=831354: closest beacon is at x=1515109, y=970092
Sensor at x=3568311, y=1566400: closest beacon is at x=3152133, y=2932891
Sensor at x=3831960, y=3146611: closest beacon is at x=3724687, y=3294321
Sensor at x=2505534, y=3196726: closest beacon is at x=2514367, y=3218259
Sensor at x=2736967, y=3632098: closest beacon is at x=2514367, y=3218259
Sensor at x=3963402, y=3944423: closest beacon is at x=3724687, y=3294321
Sensor at x=1483115, y=2119639: closest beacon is at x=1597979, y=2000000
    "###.trim()
}