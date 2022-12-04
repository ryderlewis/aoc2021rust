use std::fmt;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut answer = 0;
    let mut grid = Octogrid::parse();
    for _x in 1..=100 {
        // println!("Turn {}: total {}, grid: {}", x, answer, grid);
        answer += grid.do_turn();
    }

    println!("{}", answer);
}

fn part2() {
    let mut answer = 0;
    let mut grid = Octogrid::parse();
    loop {
        answer += 1;
        if grid.do_turn() == (grid.rows.len() * grid.rows[0].len()) as i32 {
            break;
        }
    }

    println!("{}", answer);
}

#[derive(Debug)]
struct Octogrid {
    rows: Vec<Vec<Octopus>>,
}

impl fmt::Display for Octogrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in self.rows.iter() {
            for c in r.iter() {
                f.write_fmt(format_args!("{}", c.energy)).expect("");
            }
            f.write_fmt(format_args!("{}", "\n")).expect("");
        }
        Ok(())
    }
}

impl Octogrid {
    fn parse() -> Self {
        let mut v = vec![];

        for l in input().lines() {
            let mut row = vec![];
            for c in l.trim().chars() {
                row.push(Octopus::new(c as u8 - b'0'));
            }
            v.push(row);
        }

        Octogrid{
            rows: v,
        }
    }

    // do_turn returns how many flashes occurred
    fn do_turn(&mut self) -> i32 {
        let mut flashes = 0;

        // let every octopus have a turn
        for row in 0..self.rows.len() {
            for col in 0..self.rows[0].len() {
                let ref mut octopus = self.rows[row][col];
                octopus.do_turn()
            }
        }

        // now see how many octopus flash
        loop {
            let mut flash_count = 0;

            for row in 0..self.rows.len() {
                for col in 0..self.rows[0].len() {
                    let ref mut octopus = self.rows[row][col];
                    if octopus.should_flash() {
                        flash_count += 1;
                        for (r, c) in self.neighbors(row, col) {
                            self.rows[r][c].neighbor_flashed();
                        }
                    }
                }
            }

            if flash_count == 0 {
                break;
            }

            flashes += flash_count;
        }

        flashes
    }

    fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let height = self.rows.len();
        let width = self.rows[0].len();

        let mut v = vec![];

        if row > 0 {
            if col > 0 {
                v.push((row-1, col-1));
            }
            v.push((row-1, col));
            if col < width-1 {
                v.push((row-1, col+1));
            }
        }
        if col > 0 {
            v.push((row, col-1));
        }
        if col < width-1 {
            v.push((row, col+1));
        }
        if row < height-1 {
            if col > 0 {
                v.push((row+1, col-1));
            }
            v.push((row+1, col));
            if col < width-1 {
                v.push((row+1, col+1));
            }
        }

        v
    }
}

#[derive(Debug)]
struct Octopus {
    energy: u8,
    flashed: bool,
}

impl Octopus {
    fn new(energy: u8) -> Self {
        Octopus{
            energy,
            flashed: false,
        }
    }

    fn do_turn(&mut self) {
        self.flashed = false;
        self.energy += 1;
    }

    fn should_flash(&mut self) -> bool {
        if self.flashed || self.energy <= 9 {
            return false;
        }

        self.energy = 0;
        self.flashed = true;
        true
    }

    fn neighbor_flashed(&mut self) {
        if !self.flashed && self.energy < 10 {
            self.energy += 1
        }
    }
}

fn input() -> &'static str {
    r###"
1172728874
6751454281
2612343533
1884877511
7574346247
2117413745
7766736517
4331783444
4841215828
6857766273
    "###.trim()
}