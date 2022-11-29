pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let mut counts: [i64; 9] = [0; 9];
    for i in parse_input() {
        counts[i as usize] += 1;
    }

    for day in 0..80 {
        let z = counts[0];
        for i in 0..counts.len()-1 {
            counts[i] = counts[i+1];
        }
        counts[counts.len()-1] = z;
        counts[6] += z;
    }

    println!("{}", counts.iter().sum::<i64>());
}

fn part2() {
    let mut counts: [i64; 9] = [0; 9];
    for i in parse_input() {
        counts[i as usize] += 1;
    }

    for day in 0..256 {
        let z = counts[0];
        for i in 0..counts.len()-1 {
            counts[i] = counts[i+1];
        }
        counts[counts.len()-1] = z;
        counts[6] += z;
    }

    println!("{}", counts.iter().sum::<i64>());
}

fn parse_input() -> Vec<u8> {
    let mut v: Vec<u8> = vec![];

    for x in input().split(',') {
        v.push(x.parse().unwrap());
    }

    v
}

fn input() -> &'static str {
    r###"
1,1,1,1,1,1,1,4,1,2,1,1,4,1,1,1,5,1,1,1,1,1,1,1,1,1,1,1,1,5,1,1,1,1,3,1,1,2,1,2,1,3,3,4,1,4,1,1,3,1,1,5,1,1,1,1,4,1,1,5,1,1,1,4,1,5,1,1,1,3,1,1,5,3,1,1,1,1,1,4,1,1,1,1,1,2,4,1,1,1,1,4,1,2,2,1,1,1,3,1,2,5,1,4,1,1,1,3,1,1,4,1,1,1,1,1,1,1,4,1,1,4,1,1,1,1,1,1,1,2,1,1,5,1,1,1,4,1,1,5,1,1,5,3,3,5,3,1,1,1,4,1,1,1,1,1,1,5,3,1,2,1,1,1,4,1,3,1,5,1,1,2,1,1,1,1,1,5,1,1,1,1,1,2,1,1,1,1,4,3,2,1,2,4,1,3,1,5,1,2,1,4,1,1,1,1,1,3,1,4,1,1,1,1,3,1,3,3,1,4,3,4,1,1,1,1,5,1,3,3,2,5,3,1,1,3,1,3,1,1,1,1,4,1,1,1,1,3,1,5,1,1,1,4,4,1,1,5,5,2,4,5,1,1,1,1,5,1,1,2,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1,1,5,1,1,1,1,1,1,3,1,1,2,1,1
    "###.trim()
}