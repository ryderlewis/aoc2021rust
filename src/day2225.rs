use std::collections::HashMap;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let snafus = Snafu::parse();
    let total = snafus.iter().map(|s| s.value).sum::<i64>();
    let answer = Snafu::from(total);
    println!("{}", answer.input);
}

fn part2() {
}

#[derive(Debug)]
struct Snafu {
    input: String,
    value: i64,
}

impl Snafu {
    fn new(line: &str) -> Self {
        let line = line.trim();
        let input = line.to_string();
        let mut value = 0;

        for (i, c) in line.chars().rev().enumerate() {
            let x = match c {
                '=' => -2,
                '-' => -1,
                '1' => 1,
                '2' => 2,
                _ => 0,
            } as i64;

            let base: i64 = 5;
            value += base.pow(i as u32) * x;
        }

        Self {
            input,
            value,
        }
    }

    fn parse() -> Vec<Self> {
        input().lines().map(|line| Self::new(line)).collect()
    }

    fn from(value: i64) -> Self {
        let mut powers: HashMap<i64, i8> = HashMap::new();
        let mut cur_power: i64 = 1;
        let mut value = value;

        while value > 0 {
            let place = value % 5;
            match place {
                1 => *powers.entry(cur_power).or_insert(0) += 1,
                2 => *powers.entry(cur_power).or_insert(0) += 2,
                3 => {
                    *powers.entry(cur_power * 5).or_insert(0) += 1;
                    *powers.entry(cur_power).or_insert(0) -= 2;
                },
                4 => {
                    *powers.entry(cur_power * 5).or_insert(0) += 1;
                    *powers.entry(cur_power).or_insert(0) -= 1;
                },
                _ => (),
            }

            if *powers.entry(cur_power).or_insert(0) > 2 {
                *powers.entry(cur_power * 5).or_insert(0) += 1;
                *powers.entry(cur_power).or_insert(0) -= 5;
            }

            value /= 5;
            cur_power *= 5;
        }

        let mut input: String = "".to_string();
        while cur_power > 0 {
            let v = *powers.entry(cur_power).or_insert(0);
            match v {
                -2 => input += "=",
                -1 => input += "-",
                0 => {
                    if input.len() > 0 {
                        input += "0";
                    }
                },
                1 => input += "1",
                2 => input += "2",
                other => {
                    input += "[";
                    input += &other.to_string();
                    input += "]";
                },
            };
            cur_power /= 5;
        }

        Self {
            input,
            value,
        }
    }
}

fn input() -> &'static str {
    r###"
2-=0102020-02
1-02202=-1=-2==
1==00-101-=0-
211012=12222
112-20
11==0020-=20-=0=12=
122-22110
1==00-1201112==--=
1-10==11=
10=0=110-2
1111=000=2001=211=
22-21===0=-
2=022
11=-0100===
10=10--==-0
12=01-1
12-01--20121-2-=0-1
22-222--022-0-=01-
12-1
2-=2=02-1=
2210
1=2=0211
10=01=001-01
2
201=-1
11---10
1=-0=
1=-021
1-2=02012111=01=-100
1101=20=
20-1-
2-1020---
2=121=0=10=
1022-22211112=-1
2-1-
210--==00
12=1122-10
1=2222=2-=21=0
1=021=
1=10-1201
1=1
1-=-==0201-=1=0
22=2=211-0-1=2=1
1=21-=002111
102112=10021
1===-111-=2001122
10--
1-
1-2-2
11-1-1-02-
1=20--=1=0101-1001
12012-
1220-1--00
1=2=1-
12-0=2022=0100-1
12=2-=-=221==2=
2-2-
1=-1-01-0=
12---2=
1=1=1102-==010
10-=--200=
1=2212=-
12-121=-
1===0-=20-=
1=2020-01---112=1-
101=00111=20=
1=2=01=1-2==0-
2=-100---00
1==1=01--===0-10-
1==-=0-20
1=2011121=10--2
2-=0-01-=--12102=
200
2==22=0002-210
102
1-22221
20==00
1-22--2-0=10=11-1
1=12=2
1=-12-=
10-020-100=0=
1-11111
1=02-002220
20==0120210--1
100-121=2-2-=00=
21
11=-=-
2-=1100=10102-
112--0-2=1--=2=
1-1==100
1--02-0=1-
1102-
1-2-21-
22=1=-000=210-12
1=112-12-
1====120==0
20=2=22=22=2-=2=
1=21201=
2202202-20
1=-2=-==--200101
12010
1--22=2--02
102==-
2-==1-10
12-==12-2
22=-11110=2-211
1-0
1=02=20-1=--
1-20
1102---10
11=2-==2=1-
2=2-
2=10=2==221=01
1--==-1==
1011-111
1-2-=---012-21
1=-==0
1=0011=11112102=
10-0-=202
2-----12=
1=-2221020=01-10
12-2201==
1-102
22-100=0-===
100=0-0
    "###.trim()
}

fn input_test() -> &'static str {
    r###"
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
    "###.trim()
}
