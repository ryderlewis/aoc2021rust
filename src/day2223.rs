use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut grove = Grove::parse();
    for _ in 0..10 {
        grove.plan();
        grove.execute();
    }

    println!("{}", grove.tiles());
}

fn part2() {
    let mut grove = Grove::parse();
    let mut turn = 1;

    loop {
        grove.plan();
        if !grove.execute() {
            break;
        }
        turn += 1;
    }

    println!("{}", turn);
}

#[derive(Debug, Clone)]
enum Dir {
    North,
    South,
    West,
    East,
}

const DIRS: [Dir; 4] = [Dir::North, Dir::South, Dir::West, Dir::East];

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Coord(i32, i32);

#[derive(Debug)]
struct Elf {
    coord: Coord,
    next_coord: Option<Coord>,
    next_dir: usize,
}

impl Elf {
    fn neighbors(&self, dir: Option<&Dir>) -> Vec<Coord> {
        let mut v = vec![];

        for dx in -1..=1 {
            for dy in -1..=1 {
                let is_neighbor = if dx == 0 && dy == 0 {
                    false
                } else {
                    match dir {
                        None => true,
                        Some(dir) => {
                            match dir {
                                Dir::North => dy == -1,
                                Dir::South => dy == 1,
                                Dir::East => dx == 1,
                                Dir::West => dx == -1,
                            }
                        },
                    }
                };

                if is_neighbor {
                    v.push(Coord(self.coord.0 + dx, self.coord.1 + dy));
                }
            }
        }

        v
    }
}

#[derive(Debug)]
struct Grove {
    elves: HashMap<Coord, Elf>,
}

impl Grove {
    fn parse() -> Self {
        let mut elves = HashMap::new();

        for (y, line) in input().lines().enumerate() {
            let line = line.trim();

            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    let coord = Coord(x as i32, y as i32);

                    elves.insert(coord, Elf {
                        coord,
                        next_coord: None,
                        next_dir: 0,
                    });
                }
            }
        }

        Self { elves }
    }

    fn plan(&mut self) {
        // copy coordinates into a vec, because borrow checker and stuff
        let mut elf_coords = Vec::<Coord>::new();
        let mut neighbor_coords = HashSet::<Coord>::new();

        for coord in self.elves.keys() {
            elf_coords.push(*coord);
            neighbor_coords.insert(*coord);
        }

        // for each elf that wants to take a turn, see where they will go.
        for coord in &elf_coords {
            let elf = self.elves.get_mut(coord).unwrap();
            elf.next_coord = None;

            // elves need at least one neighbor to do anything on their turn
            let mut consider_turn = false;
            for neighbor in &elf.neighbors(None) {
                if neighbor_coords.contains(neighbor) {
                    consider_turn = true;
                    break;
                }
            }

            let elf = self.elves.get_mut(coord).unwrap();
            if consider_turn {
                for i in 0..4 {
                    let proposed_dir = &DIRS[(elf.next_dir + i) % DIRS.len()];
                    let neighbors = elf.neighbors(Some(proposed_dir));
                    // if there are no neighbors in the given direction, then propose the elf moves in
                    // that direction
                    if neighbors.iter().any(|n| neighbor_coords.contains(n)) {
                        continue;
                    }
                    elf.next_coord = Some(neighbors[1]);
                    break;
                }
            }

            elf.next_dir += 1;
            elf.next_dir %= 4;
        }
    }

    fn execute(&mut self) -> bool {
        // see which elves want to move, vs those that can.
        let mut target_counts= HashMap::<Coord, i32>::new();
        for elf in self.elves.values() {
            if let Some(target) = elf.next_coord {
                *target_counts.entry(target).or_insert(0) += 1;
            }
        }

        // find coordinates of elves that can move, given that they're the only
        // elf that wants to move.
        let mut moving_elves = vec![];
        for elf in self.elves.values() {
            if let Some(target) = elf.next_coord {
                if *target_counts.get(&target).unwrap() == 1 {
                    moving_elves.push(elf.coord);
                }
            }
        }

        // finally actually move the elves
        for old_coord in &moving_elves {
            if let Some(mut elf) = self.elves.remove(old_coord) {
                let new_coord = elf.next_coord.unwrap();
                elf.coord = new_coord;
                elf.next_coord = None;
                self.elves.insert(new_coord, elf);
            }
        }

        !moving_elves.is_empty()
    }

    fn bounds(&self) -> (Coord, Coord) {
        let mut upper_left = Coord(i32::MAX, i32::MAX);
        let mut lower_right = Coord(i32::MIN, i32::MIN);

        for coord in self.elves.keys() {
            upper_left.0 = min(upper_left.0, coord.0);
            upper_left.1 = min(upper_left.1, coord.1);
            lower_right.0 = max(lower_right.0, coord.0);
            lower_right.1 = max(lower_right.1, coord.1);
        }

        (upper_left, lower_right)
    }

    fn tiles(&self) -> i32 {
        let (upper_left, lower_right) = self.bounds();
        (lower_right.1 - upper_left.1 + 1) * (lower_right.0 - upper_left.0 + 1) - self.elves.len() as i32
    }

    fn print(&self) {
        let (upper_left, lower_right) = self.bounds();
        for y in upper_left.1..=lower_right.1 {
            for x in upper_left.0..=lower_right.0 {
                print!("{}", if self.elves.contains_key(&Coord(x, y)) { "#" } else { "." });
            }
            println!();
        }
    }
}

fn input_test() -> &'static str {
    r###"
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
    "###.trim()
}

fn input() -> &'static str {
    r###"
..###.##.#.#.##..#.#.##..#.#.##.#.....#####.#.#..###.#.#....####.##.###.#
..#.##.#..###..###..#..#....#..##..#.###..#....##.##.#.....##.######.####
##....#....#.###..##.#####....##..#.######.#.#..#....####.#...####...####
#.##.#.#.#.#.#.######.#...#.####...###.#.#..#####.....#..#.####.#...####.
....###.....###..#..####...#.#.##.##..####..#.##.......###..#.###.#..###.
.#...####.#..###.#...###....#.......##..#.#..#..##.##.#..#.#.##.#...##..#
#..#.##.#.#.##...#.#####.####.##....#.###.#.#.##.###..#.###.###.#...#.#..
##.#.##.#####...###..#.#....###.#...##.##....#.##.###....#.#.##.#.......#
###.##...###..#.##...####..##.#.#.......#..#.#....###..###.#..###..#...##
#.##.###.#.#...####.#...#.######.#.###..#.####.###.#...###.#..#.#..####..
.....##.#..###.#.#.#.############.##.###.#....##.###....####....###.#####
#.#..###.###..##.#..#####.#..##..#####.....###......#.#..###.#####.#..###
.##.#.##....##.##...#..##...#....#.###..##.#....#..######...#..####..#..#
.######.#.##.#.###.####..##....##.#####...#...##.##.#.##..#...##.#.##....
...##.....#.###..##.#.##..####.#..#.#..##..#....#....#.......#...##...##.
##.#....#######.#..#.#...#####.######.....#.#..#...######.##.####.#####..
######.#.#.#.########..#..#...###..##.##.##...#.###..###..#.#..##.##.#..#
..###.##..####...###..##.#...#.##.#.#.#####.#.##..#..#.#.#.#.#.##..###..#
..##..##...#.######..########....#.###..##.#.#.#..#.#..#....#.##..#.##...
#.#.##.##..#.#...#.#.#.###.#...#....###.###.#.....#....##..#######..##.#.
.##...#.#.####....###..##...#.#..#.#.#.#.###.#.####.#...#####..##..###..#
##..###.####.#####.######..#.......#.####..##....#.#.###..######.#.#####.
.#....###..##.###..#.#.####.#..#..#.###.#.##.#....##.###.###.#.###.##....
#..#..####.#.#..#.#.##.#..#.##..#.##...##.#..#.#...#.####.####..#.###....
..##.........##.#.##.##.#..##..#..#.#.....#.#.#.#.#.###.#...#.###..##.###
.#.#...##...#.......#.#.#...#.##.#..#.######...##.##.##.#.##.#.#..####..#
.#.###.####..#..#...#.....#.....##...##.#....####.##..#..#....##.##....##
#..#..#..##.#..##...#...##...#.###..#........###.#.##..#...###########.##
.####...##.#..#..###.###.#..##..#...##...###.....###.#.##.#...#.###.###.#
.#.#####.#...###.##..........##......##..###...#.#....#.#.###....#.#...##
#.....#####.#.###.##.#.###.##...###.##.######.#...###..#....###...##...##
..####..#.#.#.#####.#.#....#.####...#...#....##.#.#..#.#.#.###.###.##....
....#.#.##..#.##.......######....#.##.###...##.###.....#.###.####.#####..
#.#..#####.##.###.#..#.####.##.....##..###..#....##.###....#..##.##..####
#.#.###.###.##.#.##.#.##.##.###.####.######.###..##..#..##....#..###.#.#.
###..##..#..###.#...##.##..#.#.###.###..#..#....##..##.#........###....#.
##.#..#.......#.#.#.##.##.##...#..##.###..###.#.##.#...#.#.####..#......#
#..###.##....##.#..#.#.#.##.####...#.###.####.#..#..##....####.####.#.#..
#...#.###...###.##.#.##.##..#.#..#..#.....#####..##.###...###.##.#.#.#...
..#.....####....##...#..#.####...###.####.##..#..#..###..#.#..##.#.....##
##.###...##.#.#...###...###.#..#.##.###...########.#..#....##..#..##..###
....#..#..#......####....#.#.##...###.##.#..#.####..##.#..#..#..##..#####
.##.#.##.##...#.#..#.##..###.##.###.#..##....#######.##.##.#.##.###...#..
.#..#.##...###...###.###..###...#..#..###.#..#...###..#.##.#.###.##.###.#
..##.##.###.####.#.###.#.......##..#..###...##...##..####.#######....#..#
#.#..##...##....##.######..#..#.##...#.######..###..##.....#...#.####..##
#..#.....#######...#....#.#..#..#...##.#.####....#.#.###..##...#.#.#.##..
##.##.##..#..#..#...##..#.##.#####.....#.####....#...#.#######..##.###..#
....##.##....####.#...####...##.#...#.....#...##.##.#.#.#..#.##...##..###
####..##.###.#.....#.##.##...#.###.#####.###.#####.##.....#######.......#
.##.....#...#.##.###.#.##....##.#..##.#..##....#.##..####.#..###.##.####.
#..###....#..#.#..##....#...#.##..##.#....############.##...#.##..##...##
....#.#..####..#......#.#.###......#....#..##...#...####.#.#..###.#.###..
.######.....#######.....#.#.##..#..###..#..###..###........#....####.....
...####.###.####..#.##..###......#..#.#.#..#.#.####.#.....#..##..#..####.
.#..###...#.##..###.#.###.##..#.#..#....##.#.#.#.####.########.##.##..#..
..#...####.##....#.###...#..#...#..#..#########.##.#..##.#.#...##..##..##
.##....###...#.#....#.###..######.####..#.#.##....#..##..###.##.#.#.#..##
.##.###..#.###.###..##.##...###.###.#.#....#.#...#.#..#####......#.######
#....####....#####...##.#.#.#.#.#.......##...###..#...#..#...#.###.#..#.#
..#.#.####....###.##..#...##..#.#..##..#.##.####.##..##.##....#..##.###..
..###...##.........#....#..#.....#.###.......#.####.##.#.##..#.#....####.
..#.######.##..#.######.#...##.#.###.....#####.#.#..#.####..###..#####..#
..##.##.##..#..#.####.......#.#....#..#....#.......#....#####.##..##.####
.###.#.###.###..####.#.##.#...#########....##.##.#...#.#.##.#.#.#####.###
.#..##.##...#..#.#.##.#.....###.#.##.#.#.#.#.#...#####.#..#.###....###..#
....###.#.#.###.#...###.#######...#....#....#.##.#.######.#..##.#.###..##
#.#.####....###...####.##.#.#...####.##.##...#.###...#..###.###..###.#...
#.##...###.###.#######...##..#..###....######.#.#.##..######.##.....#..#.
##..###..##.#.#.##.#.#.#.#.#.##...####..##..###.##########..#...#####.###
......#..#####...##.##.#.#.#.##.##...#....#..##.#####..#...#.#..#...###.#
..###.##.##.##.#...#.....##.###.#.###..#......#.#..#.##.#..##..#.###.#.##
.......#..##.#..#.####.####.#......#.#..#.#.#.##.#..#..###...##..#..#####
    "###.trim()
}
