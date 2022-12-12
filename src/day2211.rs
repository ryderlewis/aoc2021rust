use std::collections::VecDeque;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let monkeys = Monkey::parse();

    // set up items as a copy of monkeys' items
    let mut items: Vec<VecDeque<i64>> = Vec::new();
    for m in &monkeys {
        items.push(VecDeque::from(m.items.clone()));
    }

    let mut counts: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for (idx, monkey) in monkeys.iter().enumerate() {
            counts[idx] += items[idx].len();

            while let Some(item) = items[idx].pop_front() {
                let interest = match monkey.operation {
                    Op::Square => item * item,
                    Op::Add(val) => item + val,
                    Op::Mul(val) => item * val,
                } / 3;

                let new_idx = match interest % monkey.div_by {
                    0 => monkey.throw_true,
                    _ => monkey.throw_false,
                };

                items[new_idx].push_back(interest);
            }
        }
    }

    counts.sort();
    println!("{}", counts.pop().unwrap() * counts.pop().unwrap());
}

fn part2() {
    let monkeys = Monkey::parse();
    let lcm = monkeys.iter().map(|x| x.div_by).product::<i64>();

    // set up items as a copy of monkeys' items
    let mut items: Vec<VecDeque<i64>> = Vec::new();
    for m in &monkeys {
        items.push(VecDeque::from(m.items.clone()));
    }

    let mut counts: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..10_000 {
        for (idx, monkey) in monkeys.iter().enumerate() {
            counts[idx] += items[idx].len();

            while let Some(item) = items[idx].pop_front() {
                let interest = match monkey.operation {
                    Op::Square => (item % lcm) * (item % lcm),
                    Op::Add(val) => item + val,
                    Op::Mul(val) => (item % lcm) * (val % lcm),
                } % lcm;

                let new_idx = match interest % monkey.div_by {
                    0 => monkey.throw_true,
                    _ => monkey.throw_false,
                };

                items[new_idx].push_back(interest);
            }
        }
    }

    counts.sort();
    println!("{}", counts.pop().unwrap() * counts.pop().unwrap());
}

#[derive(Debug)]
enum Op {
    Square,
    Add(i64),
    Mul(i64),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Op,
    div_by: i64,
    throw_true: usize,
    throw_false: usize,
}

impl Monkey {
    fn parse() -> Vec<Self> {
        let mut v = Vec::new();
        let mut monkey: Option<Monkey> = None;

        for l in input().lines() {
            let l = l.trim();

            if l.starts_with("Monkey") {
                if monkey.is_some() {
                    v.push(monkey.unwrap());
                }
                monkey = Some(Monkey {
                    items: Vec::new(),
                    operation: Op::Square,
                    div_by: 0,
                    throw_true: 0,
                    throw_false: 0,
                });
                continue;
            }

            if let Some(monkey) = monkey.as_mut() {
                if l.starts_with("Starting items:") {
                    let items = l[16..].split(", ");
                    for i in items {
                        let val = i.parse::<i64>().expect("parse error");
                        monkey.items.push(val);
                    }
                } else if l.starts_with("Operation: new = old") {
                    let op = &l[21..];
                    monkey.operation = match op {
                        op if op == "* old" => Op::Square,
                        op if op.starts_with("* ") => Op::Mul(op[2..].parse::<i64>().unwrap()),
                        _ => Op::Add(op[2..].parse::<i64>().unwrap()),
                    };
                } else if l.starts_with("Test: divisible by") {
                    monkey.div_by = l[19..].parse::<i64>().unwrap();
                } else if l.starts_with("If true: throw to monkey") {
                    monkey.throw_true = l[25..].parse::<usize>().unwrap();
                } else if l.starts_with("If false: throw to monkey") {
                    monkey.throw_false = l[26..].parse::<usize>().unwrap();
                }
            }
        }

        if monkey.is_some() {
            v.push(monkey.unwrap());
        }

        v
    }
}

fn input() -> &'static str {
    r###"
Monkey 0:
  Starting items: 91, 54, 70, 61, 64, 64, 60, 85
  Operation: new = old * 13
  Test: divisible by 2
    If true: throw to monkey 5
    If false: throw to monkey 2

Monkey 1:
  Starting items: 82
  Operation: new = old + 7
  Test: divisible by 13
    If true: throw to monkey 4
    If false: throw to monkey 3

Monkey 2:
  Starting items: 84, 93, 70
  Operation: new = old + 2
  Test: divisible by 5
    If true: throw to monkey 5
    If false: throw to monkey 1

Monkey 3:
  Starting items: 78, 56, 85, 93
  Operation: new = old * 2
  Test: divisible by 3
    If true: throw to monkey 6
    If false: throw to monkey 7

Monkey 4:
  Starting items: 64, 57, 81, 95, 52, 71, 58
  Operation: new = old * old
  Test: divisible by 11
    If true: throw to monkey 7
    If false: throw to monkey 3

Monkey 5:
  Starting items: 58, 71, 96, 58, 68, 90
  Operation: new = old + 6
  Test: divisible by 17
    If true: throw to monkey 4
    If false: throw to monkey 1

Monkey 6:
  Starting items: 56, 99, 89, 97, 81
  Operation: new = old + 1
  Test: divisible by 7
    If true: throw to monkey 0
    If false: throw to monkey 2

Monkey 7:
  Starting items: 68, 72
  Operation: new = old + 8
  Test: divisible by 19
    If true: throw to monkey 6
    If false: throw to monkey 0
    "###.trim()
}
