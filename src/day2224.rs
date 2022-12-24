use std::collections::{HashSet, VecDeque};
use num::integer::lcm;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let valley = Valley::parse();
    println!("{}", valley.bfs(1));
}

fn part2() {
    let valley = Valley::parse();
    println!("{}", valley.bfs(3));
}

#[derive(Debug, Eq, PartialEq)]
enum Space {
    Wall,
    Open,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
    time: i32,
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct Valley {
    grid: Vec<Vec<Space>>,
    width: i32,
    height: i32,
    maze: HashSet<Coord>, // all the walls
    cycle: i32,
}

impl Valley {
    fn parse() -> Self {
        let mut grid = vec![];
        for line in input().lines() {
            let line = line.trim();
            let mut row = vec![];

            for c in line.chars() {
                row.push(match c {
                    '^' => Space::Up,
                    'v' => Space::Down,
                    '<' => Space::Left,
                    '>' => Space::Right,
                    '#' => Space::Wall,
                    _ => Space::Open,
                })
            }

            grid.push(row);
        }

        let width = grid[0].len() as i32;
        let height = grid.len() as i32;
        let cycle = lcm(width, height);

        let mut maze = HashSet::new();
        for time in 0..cycle {
            for row in 0..height {
                for col in 0..width {
                    // figure out if a square is blocked at a given time
                    let blocked = if Space::Wall == grid[row as usize][col as usize] {
                        true
                    } else if row == 0 || row == height-1 {
                        false
                    } else {
                        // look to see how much time has passed, then look to see which blizzards
                        // are occupying this location at this time
                        let left_offset_col = (col - 1 + time) % (width - 2) + 1;
                        let right_offset_col = ((width - 2) * cycle + col - 1 - time) % (width - 2) + 1;
                        let up_offset_row = (row - 1 + time) % (height - 2) + 1;
                        let down_offset_row = ((height - 2) * cycle + row - 1 - time) % (height - 2) + 1;

                        grid[row as usize][left_offset_col as usize] == Space::Left ||
                            grid[row as usize][right_offset_col as usize] == Space::Right ||
                            grid[up_offset_row as usize][col as usize] == Space::Up ||
                            grid[down_offset_row as usize][col as usize] == Space::Down
                    };

                    if blocked {
                        maze.insert(Coord{time, row, col});
                    }
                }
            }
        }

        Self {
            grid,
            width,
            height,
            maze,
            cycle,
        }
    }

    fn bfs(&self, targets: i32) -> i32 {
        let mut visited = HashSet::<(Coord, i32)>::new();
        let mut to_visit = VecDeque::<(Coord, i32, i32)>::new();

        let starting_coord = Coord{time: 0, row: 0, col: 1};
        visited.insert((starting_coord, targets));
        to_visit.push_back((starting_coord, targets, 0));
        let target_rows = [self.height-1, 0, self.height-1];
        let target_cols = [self.width-2, 1, self.width-2];

        while let Some((coord, targets, distance)) = to_visit.pop_front() {
            // find neighbors
            // println!();
            // println!("Trying: {coord:?}, {distance}");
            let target_row = target_rows[3-targets as usize];
            let target_col = target_cols[3-targets as usize];

            for neighbor in self.neighbors(&coord) {
                // println!("Neighbor: {neighbor:?}");
                let mut targets_remain = targets;
                if neighbor.row == target_row && neighbor.col == target_col {
                    if targets == 1 {
                        return distance + 1;
                    } else {
                        targets_remain -= 1;
                    }
                }

                if !visited.contains(&(neighbor, targets_remain)) {
                    visited.insert((neighbor, targets_remain));
                    to_visit.push_back((neighbor, targets_remain, distance + 1));
                }
            }
        }

        0
    }

    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        let mut v = vec![];

        let mut neighbor = Coord {
            time: (coord.time + 1) % self.cycle,
            row: coord.row,
            col: coord.col,
        };

        // try standing still
        if !self.maze.contains(&neighbor) {
            v.push(neighbor);
        } else {
            // println!("Standing blocked: {neighbor:?}");
        }

        // try up
        if neighbor.row > 0 {
            neighbor.row -= 1;
            if !self.maze.contains(&neighbor) {
                v.push(neighbor);
            } else {
                // println!("Up blocked: {neighbor:?}");
            }
            neighbor.row += 1;
        }

        // try down
        if neighbor.row < self.height - 1 {
            neighbor.row += 1;
            if !self.maze.contains(&neighbor) {
                v.push(neighbor);
            } else {
                // println!("Down blocked: {neighbor:?}");
            }
            neighbor.row -= 1;
        }

        // try left
        if neighbor.col > 0 {
            neighbor.col -= 1;
            if !self.maze.contains(&neighbor) {
                v.push(neighbor);
            } else {
                // println!("Left blocked: {neighbor:?}");
            }
            neighbor.col += 1;
        }

        // try right
        if neighbor.col < self.width - 1 {
            neighbor.col += 1;
            if !self.maze.contains(&neighbor) {
                v.push(neighbor);
            } else {
                // println!("Right blocked: {neighbor:?}");
            }
            neighbor.col -= 1;
        }

        v
    }
}

fn input_test() -> &'static str {
    r###"
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
    "###.trim()
}

fn input() -> &'static str {
    r###"
#.####################################################################################################
#>^<.v^^v>>^>vv..<><v.<^<^<v.^>>^<>>><><<>^v^>.^<>><<^vv><^<>>vv^v.<.^v><<<>.<v><<^v<v^<>.<vv<>vv<<v>#
#<v<vv><^>v<><.<<^^<>^>v<^vvvv^<<v<<^.^.^>.<><^><<^v<^><<v>vv<<>>>^.^<>>>^vvv^><^v^^>>><<vvv>><<><><.#
#>.vv^^^<^>v>vvv>^.<v<>...v^<>v<><<..vvv<^<<v>^>^<^v^>><^^v.v>>><^.^>vv^^v>v<^^.vv<v<v<>>^^>v>vvv^v><#
#.<v>^^<^v^^<^>>^>><v>^vv>^<v<v^^<>^v><v<^<>>.v^>^.v^><vvv^.^.>vvv^.v.>.>^<^.<^>><^><v<<v>^^>><><^^^<#
#<^<<v^vvv>>><>^^>>v.v.v^^>^<>><^>>^^><v>v<^.^.v<..>>>.>>.<^<^^^>.<>^^.v^>>>v>v>^^v>^><>><<.^.^>vv^^.#
#<^^>^v><..v^v<^.<vv^v<.<>.<>.^v^>^^>>v^^v.<.^><>^v.^^v>v<^^vv<<^^v<>><>^<v>>^<><v^.>>>>^v.>v.><.>v>>#
#<>.<<v.><^><<>>v><><>^..^v^<v>v<v.<>^<>>^v.^^>>.v^.<v>v>.v.v<>v.<.vv<<^<>^^^^>>.<vv<><<>vv>^>.v^<^>>#
#<.^<vv<><v^^<^v^.<><>v>>>.<<v<<v>>v^..v<<>^>^v>>.<>>v<^>^>^<vvv<><<<..v><<<v>><v<^v^<<^vvv>^>^<v<vv<#
#>^>^v>><<>>.^<>>^.^<v>^>>.<v^>v<<>vv><.v<.^^^>v<v>.v^v^.>>^^^vvvv<^<^<v>^.^<>v<^.^<>v>>vv<>>^^^>v>><#
#>><^<<v<>>^<^>v>v^v><v<v><^v>v>^>^<>>^v>vv^>^v<vv^v>.<^><.vv^^^<^><..<<<>^>^.^>>>^<>.^<^>^<>^^v>v<v>#
#><<><v><v^v>.v>^.<^><^^v><<<<<.^>>.^^<v>v>>^<^<>.^.>>>.>>.^v>v.<^^v^<^<>v<>^^<v^><<^^^<><v^>v>v<^.v<#
#<v>.v<<><vvv^<>^^^<<v<v^v^><v.><<.^^>vvv^>^v>v><><<.v>v<<<><..>>v>>v><^^>^^.^>v.v..>^><^.<>^<<^<<v<<#
#<vv<<vvv<vv>v><<.v<v<><.>^vv.^<<<<<<<v<^<^v^v>.v><vv<v^>^^^.^vv<<v>.<<><>>><^<^.^<>^>^<>v.v.v^><v<<.#
#.>^.>vv<v<><vv^v.^v<.^.v<><>^^>^.>v^<><v<<><vv<<^v>^>v<>^v>v>>v<>^<v.<v^^<<v><<.^<^>.vv.>>^.vv<><v><#
#>>v>>>.v<v<^^>><^^v^>>v>v^>>^<v^^^^.vv.^vv<vv><v^v<>^v<>>>>^^v>^<v<v^<>><>.v>>v..^>..<v<<>.v<<.<^>v<#
#<><^>>v^<^^vv>v<v.>.v.<^v.^>vvv<^>v><^>v><><.v>>.^^>><<>.v^<<^.<.^^.<v><v.^.v<^^^^<^<^>vv<v<.v<<v<^>#
#<<^^<<^vvv<<<<.^vv^.><<vv>v^><>..^.^^v>v>^^<<v>^v.v>^v><>>^v><<.v>v<>^v<v<>v<^v^.^<vv>>^>v>^><<v><.<#
#<<>v.v>...^^.<<^>>.<>^^^v>^<^><v<<<><<vv^.v^>.v^.>>^<v>>^<v^^<v>>vv<<>.>>.v^v><.^v.>^>^><^^v^^v<v>^<#
#<^vv^<><<<v^^<^>>v><^<<v^^v^>>^>v<>.<>>v>v<><v.>v<><<<<.<<.<<>>>><><vv><^v<<<^>v<^<^^<v^><^<..v>.vv>#
#><^>>>.^.>v<^^>>vv<v><^<<v.<.>^<^<^v>vv>v^^><.^.^>>.v.>v^.>>^<>^^^>vvv>..v^vv<^v>>v<><><<<^^<<v>^^<>#
#>^<vv>^v<<.<<v>^><><>>v.>.^<>^^vv<<>>>v^<>>><<v.v>>^<<vv<^vvv><><.v><.vv^v^.v.>vv^<><.v^>vv.<<<v<><<#
#>v<^>>.>>v>v>><^>^>^.^>^>vv>^<<><^v.<vvv<^.><>>v>v>^<>><^>^^<>v>v>.<^v^>>>v^>vvv.>.>^<><>.v^vv><>^>>#
#>><>v^>^>>^>^^>^>^<^>>vv>>>>>>vvv.v>.^v<v<vvv<<>.<^v.vv.v>^>>vv<<v^>^^^^>>^v.^v>>v<vvv<.<>^^<^v<<<.<#
#<>vv<^v>v^<v>>^<<>v^>vvvv>v^><^>^v<v^^^>><^><<.^>v^vv.v<<<>..v<^<.vv>.v>^>>>^^v^v^<^v^<^^<<vv>.>.>><#
#<<<v><>v.><v>><v^<^>><>^<vv><<.<<<<.v^v>v>v>.><>.<v><^.<^>^^><v>vv.<v>.v.>>.^>^>v<^vvvv>><>^^v<^>v>.#
#>.^>^>.<v.v<.<>vv<v<>^v<.>^>v<^v^>.>^<^..v<>>v.v>^.v<^>>^^^<v.<>v><v.^v^.>vv<<^<v^<.^^^.^>>.v>.vv.^.#
#<^v><>.<^v<v<^v<vv^^>.>v^^<v>^<.<^^<<.^>vv^^<^>v<v<^<v^^>vv^v>v^^>>..v^>.<v>v>v<v^<>^<^><.>^>>><<v.<#
#>>^<vv.<<>v.<>v.^>>^^vv^<^>vv<^>v.v>>.^.v^.v<v>^<>^.>><^>v>.>>v^<^>.>>^v<<v<<^^<vvvvv><^>^>v^^^^>^v>#
#<^.>^v>^<>>^.^<>.>..><>.<v^v^vv^>v^..v<<><>>^.^><^^<^^<^v<<^<^>.<><^.>^<^vv^>>>>^^>>><v<<<<^><.>^>.<#
#.v<>><^^vv^<<v^v><<<v<<<v><>><>>v^>v>^^^v<<<<<<>.^v<>.^<><<^vv<^>>v.^><v>v<^v>^v><v>>v<^^>vv.^<^>.>.#
#>^<.>v^^^><<v><>^.>vvv>v^<^>v><<<><><.^>>.>vv.^<<<<<^<^>^>v<^^<^<><v<.<>v<<><<v<^>>v>>>^<>>>.<<^<^<<#
#>^^>.vv^><.>v<vv>^><>v><^<v.<v^>.^><v<v>v^>^<v^v>v<<v^^<^>.^<v<vv<.>>v>v^v.<vv^^^^><v>^<>^v^>vv>>vv>#
#><>^<^^>^>v<^^<vv^<<^>v<>>v^>^v.v.v>^^><^<<<<>v>vv<v>^^<v>>.v<><<^><^^<<>>>>^>v>^>><><^>v><^^><v^^>>#
#><<<v<v><>^vv.^<>^>><v>vv^v^v<<v.v<v^>>vvvv^v<^<v^.><^^v><><v<^^<>><<>v^vv<^>v><>><<<^>>^^<^>><>v<v<#
#>^v><>v<>>^v>^<v^<vv><>^^>>>>v><v^v^>>>v<^<<<<<v<^<>>v><^<<^^^<^<>v<>^<v><^.>>><<^v.<>v^^.>>^^.^>^v<#
####################################################################################################.#
    "###.trim()
}
