use std::cmp::{max, min};
use lazy_static::lazy_static;
use regex::Regex;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let blueprints = Blueprint::parse_input();
    println!("{}", blueprints.iter().map(|bp| bp.quality()).sum::<i32>());
}

fn part2() {
}

#[derive(Debug)]
struct Cost {
    ore: i32,
    clay: i32,
    obsidian: i32,
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

impl Blueprint {
    fn parse(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_ore>\d+) ore. Each clay robot costs (?P<clay_ore>\d+) ore. Each obsidian robot costs (?P<obsidian_ore>\d+) ore and (?P<obsidian_clay>\d+) clay. Each geode robot costs (?P<geode_ore>\d+) ore and (?P<geode_obsidian>\d+) obsidian.").unwrap();
        }

        let caps = RE.captures(line.trim()).unwrap();
        Self {
            id: caps["id"].parse().unwrap(),
            ore: Cost {
                ore: caps["ore_ore"].parse().unwrap(),
                clay: 0,
                obsidian: 0,
            },
            clay: Cost {
                ore: caps["clay_ore"].parse().unwrap(),
                clay: 0,
                obsidian: 0,
            },
            obsidian: Cost {
                ore: caps["obsidian_ore"].parse().unwrap(),
                clay: caps["obsidian_clay"].parse().unwrap(),
                obsidian: 0,
            },
            geode: Cost {
                ore: caps["geode_ore"].parse().unwrap(),
                clay: 0,
                obsidian: caps["geode_obsidian"].parse().unwrap(),
            },
        }
    }

    fn parse_input() -> Vec<Self> {
        input().lines().map(|line| Self::parse(line)).collect()
    }

    fn quality(&self) -> i32 {
        self.max_geodes((0, 0, 0, 0), (1, 0, 0, 0), 24)
    }

    fn max_geodes(&self, resources: (i32, i32, i32, i32), workers: (i32, i32, i32, i32), time: i32) -> i32 {
        let (r_ore, r_clay, r_obsidian, r_geodes) = resources;
        if time == 0 {
            return r_geodes;
        }
        let (w_ore, w_clay, w_obsidian, w_geodes) = workers;

        let mut best = 0;

        // go through options. purchase between 0 and n ore, 0 and n clay, 0 and n obsidion, 0 and n geodes
        let max_ore_workers = r_ore / self.ore.ore;
        for buy_ore in 0..=max_ore_workers {
            let max_clay_workers = (r_ore - buy_ore * self.ore.ore) / self.clay.ore;
            for buy_clay in 0..=max_clay_workers {
                let max_obsidian_workers = min(
                    (r_ore - buy_ore * self.ore.ore - buy_clay * self.clay.ore) / self.obsidian.ore,
                    r_clay / self.obsidian.clay,
                );
                for buy_obsidian in 0..=max_obsidian_workers {
                    let max_geode_workers = min(
                        (r_ore - buy_ore * self.ore.ore - buy_clay * self.clay.ore - buy_obsidian * self.obsidian.ore) / self.geode.ore,
                        r_obsidian / self.geode.obsidian,
                    );
                    for buy_geode in 0..=max_geode_workers {
                        let next_resources = (
                                r_ore + w_ore - buy_ore * self.ore.ore - buy_clay * self.clay.ore - buy_obsidian * self.obsidian.ore - buy_geode * self.geode.ore,
                                r_clay + w_clay - buy_obsidian * self.obsidian.clay,
                                r_obsidian + w_obsidian - buy_geode * self.geode.obsidian,
                                r_geodes + w_geodes,
                            );
                        let next_workers = (
                                w_ore + buy_ore,
                                w_clay + buy_clay,
                                w_obsidian + buy_obsidian,
                                w_geodes + buy_geode,
                            );
                        best = max(best, self.max_geodes(next_resources, next_workers, time - 1));
                    }
                }
            }
        }

        best
    }
}

/*
ore: 4 ore
clay: 2 ore
obsid: 3 ore 14 clay
geod: 2 ore 7 obsid




ore: 2 ore
clay: 3 ore
obsid: 3 ore 8 clay
geod: 3 ore 12 obsid

calc max increase possible at each minute
 */

fn input() -> &'static str {
    r###"
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "###.trim()
}

fn input_real() -> &'static str {
    r###"
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 9 clay. Each geode robot costs 3 ore and 9 obsidian.
Blueprint 2: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 3: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 9 obsidian.
Blueprint 4: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 16 obsidian.
Blueprint 5: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 16 clay. Each geode robot costs 2 ore and 15 obsidian.
Blueprint 6: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 14 obsidian.
Blueprint 7: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 8: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 4 ore and 15 obsidian.
Blueprint 9: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 10: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 2 ore and 19 obsidian.
Blueprint 11: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 12 obsidian.
Blueprint 12: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 13: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 4 ore and 9 obsidian.
Blueprint 14: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 15: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 9 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 16: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 2 ore and 16 obsidian.
Blueprint 17: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 18: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 19: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 20: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 16 clay. Each geode robot costs 2 ore and 15 obsidian.
Blueprint 21: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 7 clay. Each geode robot costs 2 ore and 19 obsidian.
Blueprint 22: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 3 ore and 17 obsidian.
Blueprint 23: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 7 clay. Each geode robot costs 4 ore and 17 obsidian.
Blueprint 25: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 9 obsidian.
Blueprint 26: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 4 ore and 9 obsidian.
Blueprint 27: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 4 ore and 7 obsidian.
Blueprint 28: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 29: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 12 clay. Each geode robot costs 3 ore and 17 obsidian.
Blueprint 30: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 2 ore and 10 obsidian.
    "###.trim()
}
