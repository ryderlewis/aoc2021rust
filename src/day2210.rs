pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let program = Program::parse();
    let x_vals = program.x_vals();
    println!("{:?}", x_vals);

    // 20th, 60th, 100th, 140th, 180th, and 220th
    println!("{}",
             20 * x_vals[19] +
                 60 * x_vals[59] +
                 100 * x_vals[99] +
                 140 * x_vals[139] +
                 180 * x_vals[179] +
                 220 * x_vals[219]);
}

fn part2() {
    let program = Program::parse();
    let x_vals = program.x_vals();

    for (i, x) in x_vals.iter().enumerate() {
        let idx = (i % 40) as i32;

        if (idx-1..=idx+1).contains(x) {
            print!("##");
        } else {
            print!("  ");
        }

        if idx == 39 {
            println!();
        }
    }
    println!();
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn parse() -> Self {
        let mut v = Vec::<Instruction>::new();

        for l in input().lines() {
            let l = l.trim();

            if l == "noop" {
                v.push(Instruction::Noop);
            } else if l.starts_with("addx") {
                let x = l[5..].parse::<i32>().expect("invalid value");
                v.push(Instruction::Addx(x));
            } else {
                panic!("Unknown input: {}", l);
            }
        }

        Self {
            instructions: v,
        }
    }

    fn x_vals(&self) -> Vec<i32> {
        let mut v = Vec::new();
        let mut curr_x = 1;

        for instruction in &self.instructions {
            match instruction {
                Instruction::Noop => {
                    // one cycle, x does not change.
                    v.push(curr_x);
                },
                Instruction::Addx(x) => {
                    // two cycles, x changes at the end.
                    v.push(curr_x);
                    v.push(curr_x);
                    curr_x += x;
                },
            }
        }

        v.push(curr_x); // end of last cycle

        v
    }
}

fn input() -> &'static str {
    r###"
addx 2
addx 4
noop
noop
addx 17
noop
addx -11
addx -1
addx 4
noop
noop
addx 6
noop
noop
addx -14
addx 19
noop
addx 4
noop
noop
addx 1
addx 4
addx -20
addx 21
addx -38
noop
addx 7
noop
addx 3
noop
addx 22
noop
addx -17
addx 2
addx 3
noop
addx 2
addx 3
noop
addx 2
addx -8
addx 9
addx 2
noop
noop
addx 7
addx 2
addx -27
addx -10
noop
addx 37
addx -34
addx 30
addx -29
addx 9
noop
addx 2
noop
noop
noop
addx 5
addx -4
addx 9
addx -2
addx 7
noop
noop
addx 1
addx 4
addx -1
noop
addx -19
addx -17
noop
addx 1
addx 4
addx 3
addx 11
addx 17
addx -23
addx 2
noop
addx 3
addx 2
addx 3
addx 4
addx -22
noop
addx 27
addx -32
addx 14
addx 21
addx 2
noop
addx -37
noop
addx 31
addx -26
addx 5
addx 2
addx 3
addx -2
addx 2
addx 5
addx 2
addx 3
noop
addx 2
addx 9
addx -8
addx 2
addx 11
addx -4
addx 2
addx -15
addx -22
addx 1
addx 5
noop
noop
noop
noop
noop
addx 4
addx 19
addx -15
addx 1
noop
noop
addx 6
noop
noop
addx 5
addx -1
addx 5
addx -14
addx -13
addx 30
noop
addx 3
noop
noop
    "###.trim()
}
