pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut ship = Ship::parse();
    ship.do_instructions();
    println!("{}", ship.message());
}

fn part2() {
    let mut ship = Ship::parse();
    ship.do_instructions_2();
    println!("{}", ship.message());
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Ship {
    fn parse() -> Self {
        let mut ship = Self {
            stacks: Vec::new(),
            instructions: Vec::new(),
        };

        let mut stacking = true;
        for l in input().lines() {
            if l.starts_with(" 1") {
                stacking = false;

                // reverse all stacks
                for v in &mut ship.stacks {
                    v.reverse();
                }
            } else if stacking {
                for (i, c) in l.trim().chars().enumerate() {
                    if i % 4 == 1 {
                        if c == '1' {
                            stacking = false;
                            break;
                        }
                        let stack_num = i / 4 + 1;
                        if ship.stacks.len() < stack_num {
                            ship.stacks.push(Vec::new());
                        }
                        if c >= 'A' && c <= 'Z' {
                            ship.stacks[stack_num-1].push(c);
                        }
                    }
                }
            } else if l.starts_with("move") {
                // move <quantity> from <source> to <dest>
                let mut inst = Instruction{quantity: 0, from: 0, to: 0};
                for (i, s) in l.trim().split_whitespace().enumerate() {
                    match i {
                        1 => inst.quantity = s.parse().unwrap(),
                        3 => inst.from = s.parse().unwrap(),
                        5 => inst.to = s.parse().unwrap(),
                        _ => (),
                    }
                }
                ship.instructions.push(inst);
            }
        }

        ship
    }

    fn do_instructions(&mut self) {
        for inst in &self.instructions {
            let from = &mut self.stacks[inst.from-1];
            let mut v: Vec<char> = Vec::new();
            for _ in 0..inst.quantity {
                v.push(from.pop().unwrap());
            }

            let to = &mut self.stacks[inst.to-1];
            to.append(&mut v);
        }
    }

    fn do_instructions_2(&mut self) {
        for inst in &self.instructions {
            let from = &mut self.stacks[inst.from-1];
            let mut v: Vec<char> = Vec::new();
            for _ in 0..inst.quantity {
                v.push(from.pop().unwrap());
            }

            let to = &mut self.stacks[inst.to-1];
            while !v.is_empty() {
                to.push(v.pop().unwrap());
            }
        }
    }

    fn message(&self) -> String {
        let mut s = String::new();
        for v in &self.stacks {
            s.push(*v.last().unwrap());
        }
        s
    }
}

fn input() -> &'static str {
    r###"
[T]     [Q]             [S]
[R]     [M]             [L] [V] [G]
[D] [V] [V]             [Q] [N] [C]
[H] [T] [S] [C]         [V] [D] [Z]
[Q] [J] [D] [M]     [Z] [C] [M] [F]
[N] [B] [H] [N] [B] [W] [N] [J] [M]
[P] [G] [R] [Z] [Z] [C] [Z] [G] [P]
[B] [W] [N] [P] [D] [V] [G] [L] [T]
 1   2   3   4   5   6   7   8   9

move 5 from 4 to 9
move 3 from 5 to 1
move 12 from 9 to 6
move 1 from 6 to 9
move 3 from 2 to 8
move 6 from 3 to 9
move 2 from 2 to 9
move 2 from 3 to 5
move 9 from 8 to 1
move 1 from 6 to 9
move 1 from 8 to 3
move 14 from 1 to 2
move 8 from 2 to 6
move 2 from 2 to 7
move 2 from 5 to 8
move 5 from 2 to 6
move 9 from 7 to 8
move 1 from 9 to 8
move 5 from 6 to 9
move 1 from 3 to 8
move 1 from 7 to 5
move 1 from 1 to 5
move 4 from 1 to 7
move 15 from 6 to 1
move 4 from 7 to 6
move 2 from 5 to 7
move 9 from 8 to 7
move 13 from 1 to 3
move 8 from 6 to 9
move 1 from 6 to 8
move 1 from 7 to 5
move 2 from 1 to 3
move 4 from 7 to 1
move 13 from 3 to 6
move 2 from 1 to 3
move 1 from 5 to 8
move 2 from 3 to 4
move 5 from 7 to 1
move 4 from 1 to 9
move 2 from 4 to 5
move 4 from 6 to 2
move 3 from 2 to 5
move 6 from 8 to 1
move 7 from 6 to 7
move 1 from 3 to 5
move 1 from 2 to 4
move 8 from 1 to 8
move 4 from 6 to 2
move 3 from 5 to 3
move 1 from 4 to 3
move 2 from 1 to 3
move 8 from 8 to 5
move 2 from 3 to 8
move 4 from 5 to 3
move 1 from 9 to 2
move 1 from 8 to 3
move 1 from 2 to 1
move 15 from 9 to 3
move 6 from 7 to 5
move 1 from 7 to 3
move 2 from 2 to 8
move 6 from 9 to 4
move 22 from 3 to 6
move 3 from 8 to 6
move 1 from 1 to 2
move 2 from 9 to 8
move 6 from 4 to 7
move 6 from 7 to 2
move 16 from 6 to 9
move 8 from 2 to 1
move 4 from 6 to 1
move 2 from 3 to 4
move 9 from 5 to 4
move 1 from 7 to 9
move 1 from 6 to 2
move 3 from 5 to 7
move 16 from 9 to 4
move 2 from 7 to 1
move 4 from 6 to 3
move 1 from 9 to 5
move 1 from 9 to 7
move 1 from 7 to 6
move 1 from 7 to 9
move 2 from 9 to 2
move 1 from 6 to 1
move 2 from 8 to 1
move 11 from 4 to 2
move 9 from 2 to 6
move 9 from 6 to 1
move 15 from 4 to 6
move 1 from 4 to 2
move 1 from 5 to 3
move 6 from 6 to 4
move 3 from 2 to 1
move 2 from 4 to 6
move 3 from 6 to 2
move 7 from 6 to 2
move 1 from 4 to 7
move 1 from 7 to 2
move 5 from 3 to 6
move 1 from 5 to 4
move 1 from 4 to 5
move 8 from 1 to 6
move 1 from 4 to 8
move 12 from 6 to 1
move 1 from 3 to 4
move 1 from 4 to 1
move 1 from 3 to 4
move 2 from 6 to 5
move 31 from 1 to 7
move 2 from 5 to 7
move 1 from 8 to 2
move 1 from 5 to 8
move 1 from 8 to 6
move 3 from 4 to 9
move 3 from 9 to 4
move 2 from 4 to 3
move 2 from 1 to 6
move 2 from 3 to 8
move 1 from 4 to 9
move 4 from 2 to 9
move 17 from 7 to 8
move 3 from 8 to 2
move 2 from 9 to 4
move 4 from 2 to 5
move 1 from 1 to 4
move 1 from 9 to 3
move 8 from 8 to 4
move 1 from 9 to 4
move 4 from 8 to 3
move 8 from 2 to 5
move 2 from 2 to 3
move 1 from 2 to 1
move 1 from 8 to 4
move 2 from 8 to 1
move 1 from 7 to 2
move 1 from 8 to 6
move 3 from 4 to 5
move 8 from 4 to 7
move 1 from 2 to 8
move 1 from 8 to 1
move 2 from 4 to 7
move 8 from 5 to 9
move 7 from 5 to 2
move 6 from 3 to 1
move 6 from 1 to 2
move 9 from 9 to 4
move 5 from 7 to 4
move 2 from 1 to 2
move 9 from 4 to 2
move 3 from 6 to 2
move 1 from 6 to 8
move 1 from 8 to 9
move 1 from 3 to 5
move 6 from 7 to 5
move 4 from 4 to 2
move 19 from 2 to 3
move 1 from 4 to 6
move 7 from 7 to 5
move 2 from 1 to 8
move 12 from 3 to 4
move 3 from 4 to 1
move 1 from 6 to 3
move 8 from 5 to 9
move 3 from 9 to 7
move 6 from 4 to 3
move 3 from 1 to 2
move 13 from 3 to 7
move 3 from 4 to 6
move 4 from 9 to 4
move 14 from 7 to 8
move 3 from 5 to 2
move 3 from 2 to 6
move 1 from 6 to 2
move 1 from 3 to 9
move 4 from 4 to 6
move 11 from 2 to 7
move 2 from 9 to 6
move 3 from 5 to 6
move 1 from 9 to 7
move 14 from 6 to 5
move 1 from 5 to 1
move 4 from 5 to 8
move 2 from 5 to 6
move 4 from 2 to 5
move 1 from 2 to 9
move 14 from 8 to 5
move 2 from 8 to 4
move 3 from 8 to 7
move 5 from 5 to 4
move 13 from 5 to 7
move 5 from 7 to 6
move 31 from 7 to 9
move 7 from 6 to 7
move 6 from 5 to 7
move 1 from 8 to 9
move 1 from 5 to 3
move 1 from 3 to 5
move 1 from 1 to 8
move 6 from 4 to 3
move 1 from 8 to 5
move 1 from 4 to 1
move 33 from 9 to 3
move 13 from 7 to 1
move 29 from 3 to 2
move 3 from 3 to 8
move 1 from 5 to 2
move 20 from 2 to 6
move 19 from 6 to 4
move 1 from 7 to 4
move 5 from 1 to 7
move 1 from 8 to 7
move 2 from 8 to 5
move 10 from 2 to 8
move 6 from 3 to 9
move 4 from 7 to 1
move 1 from 3 to 5
move 1 from 1 to 2
move 1 from 7 to 6
move 1 from 2 to 8
move 1 from 8 to 7
move 4 from 9 to 7
move 2 from 5 to 2
move 1 from 8 to 5
move 1 from 8 to 6
move 7 from 8 to 3
move 2 from 9 to 4
move 3 from 5 to 1
move 2 from 2 to 5
move 5 from 7 to 8
move 10 from 4 to 1
move 5 from 8 to 5
move 10 from 1 to 3
move 2 from 6 to 4
move 1 from 7 to 3
move 1 from 8 to 1
move 3 from 5 to 8
move 12 from 4 to 7
move 3 from 5 to 3
move 16 from 1 to 7
move 2 from 3 to 7
move 1 from 5 to 6
move 3 from 8 to 4
move 1 from 4 to 7
move 1 from 6 to 3
move 14 from 3 to 1
move 5 from 3 to 8
move 1 from 3 to 5
move 1 from 7 to 6
move 1 from 6 to 2
move 13 from 7 to 2
move 1 from 5 to 3
move 3 from 4 to 2
move 1 from 3 to 5
move 3 from 8 to 9
move 2 from 8 to 9
move 1 from 6 to 4
move 5 from 2 to 4
move 3 from 2 to 5
move 7 from 7 to 3
move 7 from 4 to 7
move 5 from 3 to 7
move 8 from 2 to 3
move 5 from 9 to 5
move 11 from 1 to 9
move 4 from 3 to 1
move 1 from 2 to 7
move 4 from 1 to 7
move 22 from 7 to 3
move 5 from 3 to 4
move 1 from 7 to 1
move 1 from 1 to 4
move 3 from 4 to 6
move 3 from 1 to 3
move 2 from 6 to 1
move 2 from 4 to 9
move 13 from 9 to 1
move 1 from 6 to 5
move 4 from 7 to 1
move 3 from 1 to 6
move 19 from 3 to 9
move 5 from 3 to 1
move 18 from 9 to 8
move 1 from 9 to 3
move 11 from 1 to 7
move 1 from 4 to 5
move 13 from 8 to 1
move 7 from 5 to 8
move 7 from 8 to 5
move 3 from 6 to 5
move 2 from 3 to 9
move 1 from 3 to 7
move 5 from 5 to 2
move 10 from 1 to 5
move 9 from 7 to 9
move 11 from 5 to 2
move 2 from 8 to 4
move 1 from 4 to 3
move 2 from 7 to 3
move 1 from 7 to 4
move 3 from 8 to 3
move 8 from 5 to 2
move 2 from 3 to 8
move 4 from 3 to 8
move 6 from 2 to 6
move 5 from 1 to 8
move 8 from 2 to 7
move 2 from 4 to 7
move 9 from 2 to 9
move 4 from 7 to 8
move 5 from 1 to 8
move 3 from 7 to 4
move 1 from 8 to 3
move 3 from 7 to 2
move 3 from 1 to 9
move 1 from 4 to 9
move 1 from 6 to 3
move 18 from 8 to 5
move 1 from 8 to 2
move 2 from 4 to 9
move 3 from 2 to 1
move 2 from 2 to 3
move 24 from 9 to 8
move 3 from 3 to 7
move 15 from 8 to 2
move 12 from 2 to 5
move 1 from 7 to 4
move 1 from 3 to 1
move 28 from 5 to 4
move 1 from 7 to 9
move 2 from 2 to 1
move 4 from 6 to 3
move 1 from 5 to 3
move 1 from 5 to 9
move 1 from 2 to 6
move 5 from 3 to 5
move 8 from 4 to 2
move 2 from 6 to 2
move 1 from 7 to 3
move 4 from 2 to 8
move 3 from 1 to 2
move 5 from 2 to 5
move 3 from 5 to 4
move 2 from 1 to 5
move 2 from 2 to 1
move 4 from 9 to 2
move 7 from 8 to 9
move 1 from 3 to 1
move 1 from 1 to 7
move 2 from 8 to 3
move 4 from 9 to 3
move 9 from 5 to 7
move 3 from 3 to 5
move 1 from 5 to 3
move 7 from 7 to 9
move 1 from 7 to 9
move 1 from 5 to 9
move 1 from 5 to 1
move 1 from 8 to 5
move 9 from 9 to 1
move 2 from 7 to 2
move 1 from 5 to 6
move 4 from 3 to 2
move 11 from 2 to 4
move 1 from 8 to 4
move 1 from 8 to 2
move 1 from 2 to 8
move 1 from 6 to 5
move 1 from 8 to 6
move 6 from 1 to 7
move 1 from 5 to 6
move 1 from 6 to 5
move 3 from 9 to 8
move 3 from 8 to 1
move 3 from 7 to 8
move 1 from 6 to 9
move 1 from 2 to 4
move 1 from 9 to 7
move 2 from 7 to 9
move 10 from 1 to 6
move 2 from 9 to 3
move 1 from 5 to 7
move 3 from 7 to 5
move 3 from 5 to 3
move 4 from 6 to 3
move 18 from 4 to 2
move 3 from 4 to 1
move 1 from 1 to 3
move 2 from 1 to 2
move 8 from 2 to 9
move 1 from 4 to 7
move 1 from 7 to 1
move 3 from 9 to 2
move 3 from 8 to 6
move 1 from 4 to 9
move 7 from 2 to 8
move 7 from 6 to 7
move 3 from 9 to 2
move 3 from 2 to 5
move 6 from 4 to 6
move 2 from 5 to 6
move 3 from 3 to 6
move 6 from 6 to 3
move 5 from 7 to 5
move 2 from 4 to 8
move 5 from 5 to 2
move 1 from 7 to 2
move 4 from 6 to 4
move 1 from 7 to 8
move 1 from 6 to 4
move 1 from 5 to 7
move 1 from 3 to 4
move 1 from 6 to 4
move 2 from 9 to 1
move 3 from 1 to 3
move 1 from 3 to 1
move 9 from 2 to 1
move 8 from 1 to 5
move 1 from 7 to 1
move 1 from 9 to 1
move 4 from 5 to 7
move 4 from 7 to 5
move 1 from 1 to 9
move 5 from 2 to 4
move 1 from 9 to 6
move 8 from 8 to 9
move 18 from 4 to 9
move 3 from 5 to 4
move 2 from 6 to 5
move 1 from 8 to 5
move 17 from 9 to 6
move 2 from 8 to 1
move 1 from 4 to 6
move 8 from 6 to 3
move 1 from 1 to 8
move 5 from 5 to 3
move 1 from 1 to 7
move 1 from 8 to 6
move 2 from 4 to 5
move 6 from 9 to 4
move 1 from 7 to 5
move 7 from 6 to 8
move 2 from 6 to 5
move 6 from 8 to 3
move 1 from 9 to 6
move 2 from 9 to 5
move 1 from 3 to 1
move 1 from 8 to 6
move 7 from 5 to 6
move 7 from 6 to 7
move 5 from 4 to 9
move 1 from 4 to 5
move 2 from 9 to 6
move 3 from 1 to 7
move 5 from 6 to 8
move 1 from 1 to 5
move 21 from 3 to 6
move 3 from 7 to 2
move 2 from 9 to 3
move 1 from 9 to 7
move 5 from 5 to 7
move 7 from 6 to 7
move 14 from 7 to 1
move 3 from 2 to 8
move 12 from 1 to 4
move 5 from 7 to 6
move 1 from 7 to 4
move 8 from 8 to 3
move 8 from 3 to 5
move 6 from 5 to 6
move 1 from 5 to 3
move 2 from 1 to 8
move 2 from 8 to 3
move 10 from 3 to 7
move 8 from 4 to 3
move 3 from 4 to 9
move 3 from 9 to 2
move 1 from 2 to 5
move 2 from 2 to 9
move 13 from 3 to 1
move 1 from 4 to 1
move 2 from 1 to 7
move 1 from 5 to 8
move 1 from 9 to 6
move 1 from 9 to 2
move 1 from 4 to 9
move 8 from 6 to 2
move 1 from 9 to 5
move 1 from 2 to 8
move 1 from 5 to 9
move 2 from 2 to 3
move 12 from 6 to 8
move 1 from 3 to 7
move 8 from 8 to 4
move 1 from 9 to 1
move 13 from 1 to 3
move 2 from 4 to 5
move 12 from 7 to 2
move 1 from 5 to 8
move 3 from 3 to 8
move 2 from 4 to 1
move 1 from 1 to 9
    "###.trim()
}
