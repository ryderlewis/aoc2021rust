use std::collections::{HashMap, HashSet};

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let cave = Cave::parse();

    println!("{}", cave.count_paths());
}

fn part2() {
    let cave = Cave::parse();

    println!("{}", cave.count_double_paths());
}

#[derive(Debug)]
struct Cave {
    rooms: HashMap<&'static str, Room>,
}

impl<'a> Cave {
    fn parse() -> Self {
        let mut cave = Cave{
            rooms: HashMap::new(),
        };
        let rooms = &mut cave.rooms;

        for l in input().lines() {
            let v: Vec<&str> = l.trim().split("-").collect();
            let name1 = v[0];
            let name2 = v[1];
            let room1 = rooms.entry(name1).or_insert(Room::new(name1));
            room1.add_connection(&name2);

            let room2 = rooms.entry(name2).or_insert(Room::new(name2));
            room2.add_connection(&name1);
        }

        cave
    }

    fn get_room(&self, name: &str) -> &Room {
        self.rooms.get(name).unwrap()
    }

    fn count_paths(&self) -> i32 {
        let start = self.get_room("start");
        let mut visited: HashSet<&str> = HashSet::from(["start"]);
        self.count_paths_with_visited(&start, &mut visited)
    }

    fn count_paths_with_visited(&self, room: &'a Room, visited: &mut HashSet<&'static str>) -> i32 {
        if room.name == "end" {
            return 1;
        }

        let mut path_count = 0;

        for s in room.linked() {
            if visited.contains(s) {
                continue;
            }
            let next_room = self.get_room(s);
            if next_room.is_small() {
                visited.insert(s);
            }
            path_count += self.count_paths_with_visited(next_room, visited);
            visited.remove(s);
        }

        path_count
    }

    fn count_double_paths(&self) -> i32 {
        let start = self.get_room("start");
        let mut visited: HashSet<&str> = HashSet::from(["start"]);
        let mut double: Option<&str> = None;
        self.count_paths_with_double_visited(&start, &mut visited, &mut double)
    }

    fn count_paths_with_double_visited(&self, room: &'a Room, visited: &mut HashSet<&'static str>, double: &mut Option<&'static str>) -> i32 {
        if room.name == "end" {
            return 1;
        }

        let mut path_count = 0;

        for s in room.linked() {
            let next_room = self.get_room(s);
            if visited.contains(s) {
                if double.is_some() || !next_room.can_double_visit() {
                    continue;
                }
            }
            if next_room.is_small() {
                if visited.contains(s) {
                    *double = Some(s);
                } else {
                    visited.insert(s);
                }
            }

            path_count += self.count_paths_with_double_visited(next_room, visited, double);

            if *double == Some(s) {
                *double = None;
            } else {
                visited.remove(s);
            }
        }

        path_count
    }
}

#[derive(Debug)]
struct Room {
    name: &'static str,
    connections: HashSet<&'static str>,
}

impl Room {
    fn new(name: &'static str) -> Self {
        Room{
            name,
            connections: HashSet::new(),
        }
    }

    fn add_connection(&mut self, name: &'static str) {
        self.connections.insert(name);
    }

    fn is_big(&self) -> bool {
        self.name.chars().all(|c| c.is_uppercase())
    }

    fn is_small(&self) -> bool {
        !self.is_big()
    }

    fn can_double_visit(&self) -> bool {
        self.is_small() && self.name != "start" && self.name != "end"
    }

    fn linked(&self) -> Vec<&'static str> {
        let mut v = Vec::new();

        for x in self.connections.iter() {
            v.push(*x);
        }

        v
    }
}

fn input() -> &'static str {
    r###"
RT-start
bp-sq
em-bp
end-em
to-MW
to-VK
RT-bp
start-MW
to-hr
sq-AR
RT-hr
bp-to
hr-VK
st-VK
sq-end
MW-sq
to-RT
em-er
bp-hr
MW-em
st-bp
to-start
em-st
st-end
VK-sq
hr-st
    "###.trim()
}
