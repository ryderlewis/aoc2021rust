use std::collections::HashMap;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut poly = Polymer::parse();
    for _ in 0..10 {
        poly.run_insertion();
    }
    println!("{}", poly.delta());
}

fn part2() {
    let mut poly = Polymer::parse();
    for _ in 0..40 {
        poly.run_insertion();
    }
    println!("{}", poly.delta());
}

#[derive(Debug)]
struct Polymer {
    template: String,
    rules: HashMap<String, String>,
    counts: HashMap<String, u64>,
}

impl Polymer {
    fn parse() -> Self {
        let mut poly = Self{
            template: String::from(""),
            rules: HashMap::new(),
            counts: HashMap::new(),
        };

        for (i, l) in input().lines().enumerate() {
            if i == 0 {
                poly.template = String::from(l.trim());
            } else if l.contains(" -> ") {
                let parts: Vec<&str> = l.trim().split(" -> ").collect();
                poly.rules.insert(String::from(parts[0]), String::from(parts[1]));
            }
        }

        // count the strings in poly.template
        for i in 0..poly.template.len()-1 {
            let fragment = String::from(&poly.template[i..=i+1]);
            *poly.counts.entry(fragment).or_insert(0) += 1;
        }

        poly
    }

    fn run_insertion(&mut self) {
        let mut new_counts: HashMap<String, u64> = HashMap::new();

        for (k, v) in self.counts.iter() {
            // k is made of of two characters, XY
            let insertion = self.rules.get(k).unwrap();
            let a = String::from(&k[0..1]) + insertion;
            let b = insertion.clone() + &k[1..2];

            *new_counts.entry(a).or_insert(0) += *v;
            *new_counts.entry(b).or_insert(0) += *v;
        }

        self.counts = new_counts;
    }

    fn element_counts(&self) -> HashMap<String, u64> {
        let mut counts = HashMap::new();
        let first = String::from(&self.template[..1]);
        counts.insert(first, 1);

        for (k, v) in self.counts.iter() {
            let c2 = String::from(&k[1..]);
            *counts.entry(c2).or_insert(0) += *v;
        }

        counts
    }

    fn delta(&self) -> u64 {
        let mut min: u64 = u64::MAX;
        let mut max: u64 = u64::MIN;

        for v in self.element_counts().values() {
            if *v < min {
                min = *v;
            }
            if *v > max {
                max = *v;
            }
        }

        max - min
    }
}

fn input() -> &'static str {
    r###"
SCSCSKKVVBKVFKSCCSOV

CP -> C
SF -> S
BH -> F
SS -> N
KB -> N
NO -> N
BP -> F
NK -> P
VP -> H
OF -> O
VH -> O
FV -> F
OP -> V
FP -> B
VB -> B
OK -> S
BS -> B
SK -> P
VV -> H
PC -> S
HV -> K
PS -> N
VS -> O
HF -> B
SV -> C
HP -> O
NF -> V
HB -> F
VO -> B
VN -> N
ON -> H
KV -> K
OV -> F
HO -> H
NB -> K
CB -> F
FF -> H
NH -> F
SN -> N
PO -> O
PH -> C
HH -> P
KF -> N
OH -> N
KS -> O
FH -> H
CC -> F
CK -> N
FC -> F
CF -> H
HN -> B
OC -> F
OB -> K
FO -> P
KP -> N
NC -> P
PN -> O
PV -> B
CO -> C
CS -> P
PP -> V
FN -> B
PK -> C
VK -> S
HS -> P
OS -> N
NP -> K
SB -> F
OO -> F
CV -> V
BB -> O
SH -> O
NV -> N
BN -> C
KN -> H
KC -> C
BK -> O
KO -> S
VC -> N
KK -> P
BO -> V
BC -> V
BV -> H
SC -> N
NN -> C
CH -> H
SO -> P
HC -> F
FS -> P
VF -> S
BF -> S
PF -> O
SP -> H
FK -> N
NS -> C
PB -> S
HK -> C
CN -> B
FB -> O
KH -> O
    "###.trim()
}
