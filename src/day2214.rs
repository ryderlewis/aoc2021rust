use std::cmp::max;
use std::collections::HashMap;

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut cave = Cave::parse();
    println!("{}", cave.fill());
}

fn part2() {
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord(i32, i32);

impl Coord {
    fn parse(s: &str) -> Self {
        let mut parts = s.split(",");
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        Self(x, y)
    }

    fn path(&self, other: &Self) -> Vec<Self> {
        let mut v = Vec::new();

        let mut c = *self;
        loop {
            if c.0 != other.0 {
                c.0 += if c.0 < other.0 { 1 } else { -1 };
            } else {
                c.1 += if c.1 < other.1 { 1 } else { -1 };
            }
            if c == *other {
                break;
            }
            v.push(c);
        }

        v
    }
}

#[derive(Debug)]
enum Material {
    Rock,
    Sand,
}

#[derive(Debug)]
struct Cave {
    source: Coord,
    materials: HashMap<Coord, Material>,
    bottom_y: i32,
}

impl Cave {
    fn parse() -> Self {
        let mut materials = HashMap::new();
        let mut bottom_y = i32::MIN;

        for line in input_real().lines() {
            let mut segments = line.trim().split(" -> ");
            let mut last_coord: Option<Coord> = None;

            for c in segments {
                let coord = Coord::parse(c);
                bottom_y = max(bottom_y, coord.1);

                materials.insert(coord, Material::Rock);
                if let Some(last_coord) = last_coord {
                    // fill in rocks between last_coord and coord
                    for c in last_coord.path(&coord) {
                        materials.insert(c, Material::Rock);
                    }
                }

                last_coord = Some(coord);
            }
        }

        Self {
            source: Coord(500, 0),
            materials,
            bottom_y,
        }
    }

    fn fill(&mut self) -> usize {
        let mut fill_count = 0;
        let diags = vec![0, -1, 1];

        loop {
            // drop a piece of sand. If its y position exceeds self.lower_y, then it will never rest.
            let mut sand_coord = self.source;

            loop {
                // see if sand can drop down
                let mut next_pos: Option<Coord> = None;
                for d in &diags {
                    let c = Coord(sand_coord.0 + *d, sand_coord.1 + 1);
                    if !self.materials.contains_key(&c) {
                        next_pos = Some(c);
                        break;
                    }
                }

                if let Some(next_pos) = next_pos {
                    sand_coord = next_pos;

                    if sand_coord.1 > self.bottom_y {
                        return fill_count;
                    }
                } else {
                    // come to rest
                    self.materials.insert(sand_coord, Material::Sand);
                    fill_count += 1;
                    break;
                }
            }
        }
    }
}

fn input() -> &'static str {
    r###"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
    "###.trim()
}

fn input_real() -> &'static str {
    r###"
521,154 -> 526,154
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
499,13 -> 499,17 -> 493,17 -> 493,24 -> 507,24 -> 507,17 -> 501,17 -> 501,13
497,80 -> 497,83 -> 489,83 -> 489,87 -> 505,87 -> 505,83 -> 501,83 -> 501,80
471,77 -> 475,77
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
545,157 -> 545,160 -> 544,160 -> 544,167 -> 556,167 -> 556,160 -> 549,160 -> 549,157
477,73 -> 481,73
532,137 -> 537,137
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
513,142 -> 513,143 -> 528,143
497,80 -> 497,83 -> 489,83 -> 489,87 -> 505,87 -> 505,83 -> 501,83 -> 501,80
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
512,128 -> 526,128 -> 526,127
480,71 -> 484,71
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
497,80 -> 497,83 -> 489,83 -> 489,87 -> 505,87 -> 505,83 -> 501,83 -> 501,80
507,95 -> 507,96 -> 515,96 -> 515,95
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
514,154 -> 519,154
478,60 -> 478,62 -> 472,62 -> 472,66 -> 484,66 -> 484,62 -> 482,62 -> 482,60
528,134 -> 533,134
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
535,154 -> 540,154
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
515,140 -> 520,140
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
478,60 -> 478,62 -> 472,62 -> 472,66 -> 484,66 -> 484,62 -> 482,62 -> 482,60
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
497,80 -> 497,83 -> 489,83 -> 489,87 -> 505,87 -> 505,83 -> 501,83 -> 501,80
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
518,137 -> 523,137
490,31 -> 494,31
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
526,146 -> 531,146
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
487,29 -> 491,29
499,13 -> 499,17 -> 493,17 -> 493,24 -> 507,24 -> 507,17 -> 501,17 -> 501,13
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
478,60 -> 478,62 -> 472,62 -> 472,66 -> 484,66 -> 484,62 -> 482,62 -> 482,60
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
534,150 -> 539,150
536,140 -> 541,140
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
478,60 -> 478,62 -> 472,62 -> 472,66 -> 484,66 -> 484,62 -> 482,62 -> 482,60
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
528,154 -> 533,154
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
484,31 -> 488,31
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
507,95 -> 507,96 -> 515,96 -> 515,95
545,157 -> 545,160 -> 544,160 -> 544,167 -> 556,167 -> 556,160 -> 549,160 -> 549,157
512,128 -> 526,128 -> 526,127
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
545,157 -> 545,160 -> 544,160 -> 544,167 -> 556,167 -> 556,160 -> 549,160 -> 549,157
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
499,13 -> 499,17 -> 493,17 -> 493,24 -> 507,24 -> 507,17 -> 501,17 -> 501,13
525,137 -> 530,137
478,60 -> 478,62 -> 472,62 -> 472,66 -> 484,66 -> 484,62 -> 482,62 -> 482,60
497,80 -> 497,83 -> 489,83 -> 489,87 -> 505,87 -> 505,83 -> 501,83 -> 501,80
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
520,150 -> 525,150
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
486,71 -> 490,71
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
517,152 -> 522,152
489,73 -> 493,73
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
530,148 -> 535,148
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
538,152 -> 543,152
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
499,13 -> 499,17 -> 493,17 -> 493,24 -> 507,24 -> 507,17 -> 501,17 -> 501,13
513,142 -> 513,143 -> 528,143
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
524,131 -> 529,131
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
483,69 -> 487,69
474,75 -> 478,75
493,29 -> 497,29
527,150 -> 532,150
545,157 -> 545,160 -> 544,160 -> 544,167 -> 556,167 -> 556,160 -> 549,160 -> 549,157
499,13 -> 499,17 -> 493,17 -> 493,24 -> 507,24 -> 507,17 -> 501,17 -> 501,13
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
499,13 -> 499,17 -> 493,17 -> 493,24 -> 507,24 -> 507,17 -> 501,17 -> 501,13
489,77 -> 493,77
524,152 -> 529,152
496,89 -> 496,90 -> 509,90
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
521,134 -> 526,134
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
496,31 -> 500,31
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
490,27 -> 494,27
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
478,60 -> 478,62 -> 472,62 -> 472,66 -> 484,66 -> 484,62 -> 482,62 -> 482,60
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
545,157 -> 545,160 -> 544,160 -> 544,167 -> 556,167 -> 556,160 -> 549,160 -> 549,157
529,140 -> 534,140
545,157 -> 545,160 -> 544,160 -> 544,167 -> 556,167 -> 556,160 -> 549,160 -> 549,157
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
483,73 -> 487,73
523,148 -> 528,148
542,154 -> 547,154
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
480,75 -> 484,75
495,77 -> 499,77
478,60 -> 478,62 -> 472,62 -> 472,66 -> 484,66 -> 484,62 -> 482,62 -> 482,60
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
522,140 -> 527,140
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
496,89 -> 496,90 -> 509,90
492,75 -> 496,75
499,13 -> 499,17 -> 493,17 -> 493,24 -> 507,24 -> 507,17 -> 501,17 -> 501,13
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
507,95 -> 507,96 -> 515,96 -> 515,95
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
497,80 -> 497,83 -> 489,83 -> 489,87 -> 505,87 -> 505,83 -> 501,83 -> 501,80
477,77 -> 481,77
497,80 -> 497,83 -> 489,83 -> 489,87 -> 505,87 -> 505,83 -> 501,83 -> 501,80
486,75 -> 490,75
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
502,122 -> 502,115 -> 502,122 -> 504,122 -> 504,114 -> 504,122 -> 506,122 -> 506,115 -> 506,122 -> 508,122 -> 508,119 -> 508,122 -> 510,122 -> 510,113 -> 510,122 -> 512,122 -> 512,116 -> 512,122 -> 514,122 -> 514,115 -> 514,122 -> 516,122 -> 516,113 -> 516,122 -> 518,122 -> 518,117 -> 518,122
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
531,152 -> 536,152
545,157 -> 545,160 -> 544,160 -> 544,167 -> 556,167 -> 556,160 -> 549,160 -> 549,157
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
483,77 -> 487,77
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
474,44 -> 474,38 -> 474,44 -> 476,44 -> 476,39 -> 476,44 -> 478,44 -> 478,34 -> 478,44 -> 480,44 -> 480,40 -> 480,44 -> 482,44 -> 482,36 -> 482,44 -> 484,44 -> 484,34 -> 484,44 -> 486,44 -> 486,38 -> 486,44 -> 488,44 -> 488,36 -> 488,44 -> 490,44 -> 490,36 -> 490,44
464,57 -> 464,49 -> 464,57 -> 466,57 -> 466,53 -> 466,57 -> 468,57 -> 468,54 -> 468,57 -> 470,57 -> 470,56 -> 470,57 -> 472,57 -> 472,47 -> 472,57 -> 474,57 -> 474,53 -> 474,57 -> 476,57 -> 476,51 -> 476,57 -> 478,57 -> 478,50 -> 478,57 -> 480,57 -> 480,55 -> 480,57
500,109 -> 500,100 -> 500,109 -> 502,109 -> 502,103 -> 502,109 -> 504,109 -> 504,106 -> 504,109 -> 506,109 -> 506,108 -> 506,109 -> 508,109 -> 508,108 -> 508,109 -> 510,109 -> 510,105 -> 510,109
    "###.trim()
}
