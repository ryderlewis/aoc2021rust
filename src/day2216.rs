use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn input() -> &'static str {
    return input_real();
}

fn part1() {
    let valves = Valve::parse_all();

    // build a map of distances by running BFS from each valve to all other valves
    let mut bfs: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for valve in valves.values() {
        bfs.insert(valve.name.clone(), valve.bfs_distances(&valves));
    }

    // brute force the best
    let mut visited: HashSet<String> = HashSet::new();
    println!("{}", pressure_relief(30, "AA".to_string(), &mut visited, &bfs, &valves));
}

fn pressure_relief(time_remaining: i32, pos: String, visited: &mut HashSet<String>, bfs: &HashMap<String, HashMap<String, i32>>,
                   valves: &HashMap<String, Valve>) -> i32 {
    // if time remaining is 1 minute or less, there's nothing left that can improve pressure relief
    if time_remaining <= 1 {
        return 0;
    }

    let valve = valves.get(&pos).unwrap();
    let dists = bfs.get(&pos).unwrap();

    // this is an optimization, but valve AA is the starting valve, and has flow rate of 0.
    // any other valves that are visited will have a flow rate > 0. So, go ahead and
    // plan on opening the valve at the current location if its rate > 0.
    let mut time_remaining = time_remaining;
    let mut curr_relief = 0;

    if valve.flow > 0 {
        time_remaining -= 1;
        curr_relief = time_remaining * valve.flow;
        visited.insert(pos.clone());
    }

    // go through possible options
    let mut max_relief = 0;

    for (next_pos, dist) in dists {
        if visited.contains(next_pos) {
            continue;
        }
        let next_valve = valves.get(next_pos).unwrap();
        if next_valve.flow == 0 {
            continue;
        }

        max_relief = max(max_relief, pressure_relief(time_remaining - *dist, next_pos.clone(), visited, bfs, valves));
    }

    visited.remove(&pos);
    curr_relief + max_relief
}

fn part2() {
    let valves = Valve::parse_all();

    // build a map of distances by running BFS from each valve to all other valves
    let mut bfs: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for valve in valves.values() {
        if valve.flow > 0 || valve.name == "AA" {
            bfs.insert(valve.name.clone(), valve.bfs_distances(&valves));
        }
    }

    // brute force the best
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("AA".to_string());

    let state = State {
        pos_a: String::from("AA"),
        arrive_a: 26,
        pos_b: String::from("AA"),
        arrive_b: 26,
    };

    println!("{}", pressure_relief_2(&state, &mut visited, &bfs, &valves));
}

struct State {
    pos_a: String,
    arrive_a: i32,
    pos_b: String,
    arrive_b: i32,
}

fn pressure_relief_2(state: &State, visited: &mut HashSet<String>, bfs: &HashMap<String, HashMap<String, i32>>,
                   valves: &HashMap<String, Valve>) -> i32 {
    // decide whether to operate on A or B (person or elephant)
    let op_on_a = state.arrive_a >= state.arrive_b;
    let mut time_remaining = if op_on_a { state.arrive_a } else { state.arrive_b };
    let pos = if op_on_a { state.pos_a.clone() } else { state.pos_b.clone() };

    // if time remaining is 1 minute or less, there's nothing left that can improve pressure relief
    if time_remaining <= 1 {
        return 0;
    }

    let valve = valves.get(&pos).unwrap();
    let dists = bfs.get(&pos).unwrap();

    let mut next_state = State {
        pos_a: state.pos_a.clone(),
        pos_b: state.pos_b.clone(),
        arrive_a: state.arrive_a,
        arrive_b: state.arrive_b,
    };

    let mut curr_relief = 0;
    // check that visited doesn't already contain the pos before opening the valve,
    // this can happen when an actor simply hangs out for a minute at the same spot
    // after opening the valve.
    let mut i_visited = false;
    if valve.flow > 0 && !visited.contains(&pos) {
        time_remaining -= 1;
        curr_relief = time_remaining * valve.flow;
        i_visited = true;
        visited.insert(pos.clone());
    }
    if valve.flow > 0 && !i_visited {
        return 0;
    }

    // go through possible options
    let mut max_relief = 0;

    for (next_pos, dist) in dists {
        if visited.contains(next_pos) {
            continue;
        }
        let next_valve = valves.get(next_pos).unwrap();
        if next_valve.flow == 0 {
            continue;
        }

        if op_on_a {
            next_state.pos_a = next_pos.clone();
            next_state.arrive_a = time_remaining - *dist;
        } else {
            next_state.pos_b = next_pos.clone();
            next_state.arrive_b = time_remaining - *dist;
        }

        max_relief = max(max_relief, pressure_relief_2(&next_state, visited, bfs, valves));
    }

    if i_visited {
        visited.remove(&pos);
    }

    curr_relief + max_relief
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow: i32,
    tunnels: Vec<String>,
}

impl Valve {
    fn parse_all() -> HashMap<String, Self> {
        let mut m = HashMap::new();

        for line in input().lines() {
            let valve = Self::parse(line);
            m.insert(valve.name.clone(), valve);
        }

        m
    }

    fn parse(line: &str) -> Self {
        let line = line.trim().to_string();
        let line = line.replace("Valve ", "");
        let line = line.replace("has flow rate=", "");
        let line = line.replace("; tunnels lead to valves", "");
        let line = line.replace("; tunnel leads to valve", "");
        let line = line.replace(",", "");

        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap().to_string();
        let flow = parts.next().unwrap().parse::<i32>().unwrap();
        let tunnels = parts.map(|s| s.to_string()).collect();

        Self {
            name,
            flow,
            tunnels,
        }
    }

    fn bfs_distances(&self, valves: &HashMap<String, Valve>) -> HashMap<String, i32> {
        let start = &self.name;
        let mut m = HashMap::new();
        let mut v: VecDeque<(String, i32)> = VecDeque::new();

        m.insert(start.clone(), 0);
        v.push_back((start.clone(), 0));

        while let Some((name, dist)) = v.pop_front() {
            let valve = valves.get(&name).unwrap();
            for tunnel in &valve.tunnels {
                if !m.contains_key(tunnel) {
                    m.insert(tunnel.clone(), dist+1);
                    v.push_back((tunnel.clone(), dist+1));
                }
            }
        }

        m
    }

}

fn input_test() -> &'static str {
    r###"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
    "###.trim()
}

fn input_real() -> &'static str {
    r###"
Valve ZN has flow rate=0; tunnels lead to valves SD, ZV
Valve HO has flow rate=17; tunnel leads to valve LT
Valve FT has flow rate=6; tunnels lead to valves DW, BV, JA, FB, TV
Valve AD has flow rate=0; tunnels lead to valves AA, JG
Valve GE has flow rate=0; tunnels lead to valves JG, RD
Valve GI has flow rate=0; tunnels lead to valves WJ, RD
Valve RM has flow rate=0; tunnels lead to valves BU, WJ
Valve GV has flow rate=0; tunnels lead to valves WB, HS
Valve VA has flow rate=0; tunnels lead to valves AA, HS
Valve TJ has flow rate=21; tunnel leads to valve CK
Valve WB has flow rate=0; tunnels lead to valves GV, EV
Valve DV has flow rate=19; tunnels lead to valves OI, NK
Valve EL has flow rate=0; tunnels lead to valves HS, YC
Valve KU has flow rate=0; tunnels lead to valves WJ, OI
Valve WI has flow rate=16; tunnels lead to valves SD, AN, GS, JV
Valve JG has flow rate=3; tunnels lead to valves SV, BU, GC, GE, AD
Valve TC has flow rate=0; tunnels lead to valves TV, WJ
Valve GC has flow rate=0; tunnels lead to valves JG, JA
Valve LS has flow rate=0; tunnels lead to valves JH, YP
Valve OI has flow rate=0; tunnels lead to valves KU, DV
Valve ZH has flow rate=0; tunnels lead to valves YZ, RD
Valve YZ has flow rate=0; tunnels lead to valves ZH, AA
Valve YP has flow rate=0; tunnels lead to valves KS, LS
Valve CK has flow rate=0; tunnels lead to valves EG, TJ
Valve NY has flow rate=0; tunnels lead to valves HS, UU
Valve IQ has flow rate=18; tunnel leads to valve YC
Valve HI has flow rate=0; tunnels lead to valves SS, RD
Valve DW has flow rate=0; tunnels lead to valves FT, JH
Valve EV has flow rate=7; tunnels lead to valves SV, WB, SS, GS
Valve SV has flow rate=0; tunnels lead to valves JG, EV
Valve BU has flow rate=0; tunnels lead to valves JG, RM
Valve GS has flow rate=0; tunnels lead to valves EV, WI
Valve UY has flow rate=0; tunnels lead to valves WJ, FE
Valve AA has flow rate=0; tunnels lead to valves VA, YZ, AD, FB
Valve SD has flow rate=0; tunnels lead to valves WI, ZN
Valve KS has flow rate=23; tunnel leads to valve YP
Valve RD has flow rate=4; tunnels lead to valves GI, HI, BV, ZH, GE
Valve ZV has flow rate=15; tunnel leads to valve ZN
Valve HB has flow rate=0; tunnels lead to valves HS, AN
Valve UU has flow rate=0; tunnels lead to valves EG, NY
Valve SS has flow rate=0; tunnels lead to valves HI, EV
Valve HS has flow rate=12; tunnels lead to valves HB, EL, VA, GV, NY
Valve LT has flow rate=0; tunnels lead to valves DS, HO
Valve JH has flow rate=5; tunnels lead to valves LS, FE, QU, NK, DW
Valve AN has flow rate=0; tunnels lead to valves HB, WI
Valve NK has flow rate=0; tunnels lead to valves DV, JH
Valve JA has flow rate=0; tunnels lead to valves GC, FT
Valve EG has flow rate=14; tunnels lead to valves CK, UU, DS
Valve JV has flow rate=0; tunnels lead to valves QU, WI
Valve WJ has flow rate=8; tunnels lead to valves GI, RM, KU, UY, TC
Valve FE has flow rate=0; tunnels lead to valves JH, UY
Valve TV has flow rate=0; tunnels lead to valves FT, TC
Valve YC has flow rate=0; tunnels lead to valves IQ, EL
Valve QU has flow rate=0; tunnels lead to valves JV, JH
Valve DS has flow rate=0; tunnels lead to valves LT, EG
Valve BV has flow rate=0; tunnels lead to valves FT, RD
Valve FB has flow rate=0; tunnels lead to valves AA, FT
    "###.trim()
}
