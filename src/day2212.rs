use std::collections::{HashMap, VecDeque};

pub fn run(part: i8) {
    if part == 1 {
        part1();
    } else {
        part2();
    }
}

fn part1() {
    let use_real = true;
    let g = Grid::parse(use_real);

    println!("{}", g.find_shortest_path(true));
}

fn part2() {
    let use_real = true;
    let g = Grid::parse(use_real);

    println!("{}", g.find_shortest_path(false));
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<u8>>,
    num_rows: usize,
    num_cols: usize,
    start_pos: Coordinate,
    end_pos: Coordinate,
}

impl Grid {
    fn parse(real: bool) -> Self {
        let mut rows = vec![];
        let mut start_pos = Coordinate{row: 0, col: 0};
        let mut end_pos = Coordinate{row: 0, col: 0};

        for (row_num, line) in input(real).lines().enumerate() {
            let line = line.trim();
            let mut row = Vec::with_capacity(line.len());
            for (col_num, b) in line.bytes().enumerate() {
                match b {
                    b'S' => {
                        start_pos = Coordinate {row: row_num, col: col_num};
                        row.push(b'a')
                    },
                    b'E' => {
                        end_pos = Coordinate {row: row_num, col: col_num};
                        row.push(b'z')
                    },
                    _ => row.push(b),
                }
            }

            rows.push(row);
        }

        let num_rows = rows.len();
        let num_cols = rows[0].len();

        Self {
            rows,
            start_pos,
            end_pos,
            num_rows,
            num_cols,
        }
    }

    fn find_shortest_path(&self, ascending: bool) -> usize {
        // use BFS to find shortest path from S to E.
        let mut visited: HashMap<Coordinate, usize> = HashMap::new();
        let mut to_visit: VecDeque<Coordinate> = VecDeque::new();

        if ascending {
            visited.insert(self.start_pos, 0);
            to_visit.push_back(self.start_pos);
        } else {
            visited.insert(self.end_pos, 0);
            to_visit.push_back(self.end_pos);
        }

        while let Some(coordinate) = to_visit.pop_front() {
            let elevation = self.rows[coordinate.row][coordinate.col];
            let curr_distance = *visited.get(&coordinate).unwrap();

            // find neighbors that have not been visited.
            for neighbor in self.neighbors(&coordinate) {
                let neighbor_elevation = self.rows[neighbor.row][neighbor.col];
                let can_travel = match ascending {
                    true => neighbor_elevation <= elevation + 1,
                    false => neighbor_elevation >= elevation - 1,
                };
                if can_travel && !visited.contains_key(&neighbor) {
                    if ascending && neighbor == self.end_pos || !ascending && neighbor_elevation == b'a' {
                        return curr_distance+1;
                    }
                    visited.insert(neighbor, curr_distance+1);
                    to_visit.push_back(neighbor);
                }
            }
        }

        usize::MAX
    }

    fn neighbors(&self, c: &Coordinate) -> Vec<Coordinate> {
        let mut v = vec![];

        if c.row > 0 {
            v.push(Coordinate{row: c.row-1, col: c.col})
        }
        if c.row < self.num_rows - 1 {
            v.push(Coordinate{row: c.row+1, col: c.col})
        }
        if c.col > 0 {
            v.push(Coordinate{row: c.row, col: c.col-1})
        }
        if c.col < self.num_cols - 1 {
            v.push(Coordinate{row: c.row, col: c.col+1})
        }

        v
    }
}

fn input(real: bool) -> &'static str {
    match real {
        true =>
            r###"
abccccaaaaaaaaaaaaaccaaaaaaaacccccccccaaaaaaaaccccccccaaacaaacccccccaaaaaaccccccccccccccccccccccaaaacccccccccccacccccccccccccccccccccccccccccccccccccccccccccccaaaa
abccccaaaaacaaaaaaccccaaaaaaccccccccccaaaaaaacccccccccaaaaaaacccccaaaaaaaaaacccccccccccccccccccaaaaaacccccccccaaaaaaaaccccccccccccccccccccccccccccccccccccccccaaaaa
abcccaaaaaccaaaaaaccccaaaaaaccccccaacccaaaaaacccccccccaaaaaacccaaaaaaaaaaaaaaacaaccacccccccccccaaaaaaccccccccccaaaaaacccccccccccccccccccccccccccccccccccccccccaaaaa
abccccccaaccaaaaaaccaaaaaaaaccccccaaacaaaacaaacccccccaaaaaaaaccaaaaaaaaaaaaaaacaaaaacccccccccccccaaccccccccccccaaaaaaccccccccccccccccccccccccccccacccccccccccaaaaaa
abccccccccccaaccaaccaaaaccaacccccccaaaaaaaccccccccccaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaacccccccaaaaccccccccccccccccaaaaaaaacccccccccccccccccccccccccccaaccccccccccccccaa
abcccccccaaaaacccaaaaaaaacccaaccccaaaaaaccccccccccccaaaaaaaaaaaaaaaacaaaaaaaccaaaaaaccccccaaaaaccccccccccccccaaaaaaaaaaccaccccccccccccccccccccccccaccccccccccccccca
abcccccccaaaaacccaaaaaaaaccaaaaccaaaaaaaaccccccccccccccaaacaaaaaaaaacaaaaaacccccaaaacccccaaaaaaccccccccccccccaaaaaaaaaaaaacccccccccccccccllllllccccdccccccccccccccc
abccccccaaaaaacccccaaaaccccaaaaacaaaaaaaaccccccccccccccaaacccccaaaccccaaaaaacccaaccccccccaaaaaacccccccccccccccccaaaaaaaaaacccccccccccccklllllllllcddddccaccaaaccccc
abccccccaaaaaacccccaaaaaaaaaaaaaaaccaaccccccaacaacccccccaaccccccccccccaaacaacccccccccccccaaaaaacccccccccccccccccaaaaaaaaaacccccccccccckklllppllllcddddddddaaaaccccc
abccccccaaaaaaccccaaacaaaaaaaaaaaaccaaccccccaaaaaccccccccccccccccccccccccccccccccccccccccccaaccccccaaccccccccccccaacaaaaaaaccccccccccckklpppppplllmdddddddddacccccc
abccccccccaaacccccaacccaccaaaaaaccccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccccccccaaaaccccccccccccaaaaaaaaaaccccccccckkkkppppppplmmmmmmddddddaacccc
abccccaaacaaacccccccccccccaaaaaaccccccccccccaaaaaacccccccccccccccccaaaccccccccccccccccccccccccccccaaaaccccccccccccaaaaaaaaaaccccccccckkkppppuppppmmmmmmmmddeeeacccc
abccccaaaaaaacccccccccccccaaaaaacccaccccccccaaaaaacccccccccccccccccaaaacccccccccccccccccccccaaacccaaaacccccccccccaaaacaaaccccccccccckkkpppuuuuuppqqmmmmmmmmeeeacccc
abcccccaaaaaaccccccccccccaaaaaaaacaaccccccccccaaaccccccccccccccccccaaaaccccccccccccccccccccaaaaccccccccccccccccccaaaaaaaacccccccccckkkkpppuuuuupqqqqqqqmmmmeeeccccc
abcccccaaaaaaaacccccccccccaccccaaaaacccccccccccccccccccccccccccccccaaaccccccccccccccaaaccccaaaacccccccccccccaaccaaaaaaaaccccccccckkkkkrrpuuuxuuuqqqqqqqqmmmmeeccccc
abccccaaaaaaaaaccccccccccccccaaaaaacccccccacaacccccccccccccccccccccccccccccccccccccaaaaaacccaaaccccccccccaaaaccaaaaaaacccccccccckkkkrrrrruuuxxuvvvvvvqqqqnnneeccccc
abcccaaaaaaaaaaccccccccccccccaaaaaaaacccccaaaaacccccccccccccccaaaaaccccccccccccccccaaaaaaccccccccccccccccaaaaaaaaaaaaacccccccccjjjkrrrrruuuxxxxvvvvvvvqqqnnneeccccc
abcaaaaacaaacccccccccccccccccaaaaaaaacccccaaaaaccaacccccccccccaaaaaccccccccccccccccaaaaaccccccccccccccccccaaaaaccaaaaaacccccccjjjrrrrruuuuuxxxyvyyyvvvqqqnneeeccccc
abcaaaaacaaaccaaccccccccccccccccaacccccccaaaaaaaaaaaccccccccccaaaaaaccccccccccccccccaaaaaccccccccccccccccaaaaacccaaaaaaaacaaacjjjrrrtttuuxxxxxyyyyyvvvqqnnneeeccccc
abaaaaaccaacccaaaccaacccaaccccccaccccccccaaaaaaaaaacccccccccccaaaaaaccccccccccccccccaacaacccccccccccccccccccaacccaaccccaaaaaacjjjrrrtttxxxxxxxyyyyyvvvrrnnneeeccccc
SbaaaaacccccccaaaaaaaccaaaacccccccccccccccaaaaaaaaacccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccccccaacccaaaaaacjjjrrrtttxxxEzzzzyyyvvvrrnnneeecccccc
abcaaaaacccccccaaaaaaccaaaacccccccccccccccaaaaaaaaacccccccccccccaaccccccccccccccccccccccccccccaaccccccccccaaccccacaaaacaaaaaaajjjrrrtttxxxxxyyyyyvvvrrrnnnfffcccccc
abcaacccccccaaaaaaaacccaaaaccccccccccccccccaaaaaaaaaaccccccccccccccccccccccccccccccccccccccaaaaaccccccccccaaccccaaaaaaaaaaaaaajjjqqqttttxxxxyyyyyyvvrrrnnnfffcccccc
abccccccccccaaaaaaaaaccccccccccccccccccccaaaaaaaaaaaaacccccccccccccccccaacccccccccccccccccccaaaaaccccccaacaaaaaccaaaacaaaaaaaacjjjqqqqttttxxyywyyyywvrrnnnfffcccccc
abccccccccccaaaaaaaaaacccccccccccccccccccaaaaaaaaacaaacccccccccccccaaacaacccccccccccccccccccaaaaaccccccaaaaaaaaccaaaaccccaaacccjjjjqqqqtttxwywwwyywwwrrnnnfffcccccc
abcccccccccccccaaaaaaacccccccccccccccccccaaaaaaaaaaaaaaaacccccccccccaaaaaccccccccccccccccccaaaaacccaaccccaaaaccccaacaacccaaaccccjjjiqqqtttwwywwwwwwwwrrroofffcccccc
abcccccccccccccaaaccccccccccccccccccccccaaaaaaaaaaaaaaaaaccccccccccccaaaaaacccccccccccccccccccaaacaaaccccaaaaaccccccccccccccccccciiiiqqqttwwwwwswwwwrrrroofffcccccc
abcccccccccccccaaccccccccccccaaaacccccccaaaaaaaaccaaaaacccccccccccccaaaaaaacccccccccccccccccccaaaaaaacccaaacaacccccaaaaacccccccccciiiqqqttwwwwsssssrrrrroofffaccccc
abcccccccccccccccccccccccccccaaaaccccccccacaaacccaaaaaaccccccaaccccaaaaaaccccccccaacaaccccccccaaaaaaccccaaaacacccccaaaaacccccccccciiiqqqtsswsssssssrrrrooofffaccccc
abcccccccccccccccccccccccccccaaaaccccccccccaaaccaaaaaaaccccccaaaaccaacaaaccccccccaaaaacccccccccaaaaaaaaccaaacacccccaaaaaacccccccccciiqqqssssssspposrrroooofffaccccc
abccccaaacccccccccccccccccccccaaacccccccccccccccaaacaaaccccaaaaaacccccaaaccccccccaaaaaacccccccaaaaaaaaaaaaaaaaaccccaaaaaaccccccaccciiiqqpsssssppppooooooogffaaccccc
abccccaaaaaacccaaaccccccccccccccccccccccccccccccccccccaccccaaaaacccccccccccccccccaaaaaaccccccaaaaaaaaaaaaaaaaaaccccaaaaaacccaaaaccciiiqqppppppppppoooooogggfaaacccc
abcccaaaaaaacccaaaccccccccccccccccccccccccccccccccccccccccccaaaaaccccccccccccccccaaaaaaccccccaaacaaaccccaaaaaacccccccaacccccaaaaaacciiipppppppphgggggggggggaaaacccc
abccaaaaaaaacccaaacaaacccccccccccccccccccccaacccccccccccccccaacaacccccaacccccccccccaaacccccccccccaaacccccaaaaacccccccccccccccaaaaacciiihppppphhhhgggggggggaaccccccc
abccaaaaaaacaaaaaaaaaacccccccccccccccccccccaaaccccccacccccccccccccccccaaccccccccccccccccccccccccaaaaccccaaaaaaccccccccccccccaaaaacccciihhhhhhhhhhgggggggccaaccccccc
abccccaaaaaaaaaaaaaaacccccccccccccccccccaaaaaaaaccccaaacaaaccccccccccaaaaccaaccccccccaacaacccccaaaaaaacccaacccccccccccccccccaacaaccccchhhhhhhhhaaaacccccccccccccccc
abccccaaaaaacaaaaaaaccccccccccccccccccccaaaaaaaaccccaaaaaaaccccccccccaaaaaaaacaccccccaaaaaccccccaaaaacccccccccccccccccccccccccccccccccchhhhhhacaaaaaccccccccccccccc
abccccaaccccccaaaaaacccccccccccccaaccccccaaaaaacccccaaaaaaccccccaaaaaaaaaaaaaaaccccccaaaaaacccaaaaaaacccccccccccccccccccccccccccccccccccccaaaaccaaacccccccccccaaaca
abccccccccccccaaaaaaaccccccccccccaaccccccaaaaaacccaaaaaaaaccccccaaaaaaaaaaaaaacccccccaaaaaacccaaaaaaaaccccccaaacccccccccccccccccccccccccccaaaaccccccccccccccccaaaaa
abccaaacccccccaaacaaacccccccccaaaaaaaacccaaaaaacccaaaaaaaaacccccaaaaaaaaaaaaaacccccccaaaaaccccaaaaaaaaccccccaaaaccccccccccccccccccccccccccaaaccccccccccccccccccaaaa
abcaaaacccccccaaccccccccccccccaaaaaaaacccaaccaacccaaaaaaaaaaccccccccaaaaaaacaacccccccccaaaccccccaaacaaccccccaaaacccccccccccccccccccccccccccccccccccccccccccccaaaaaa
            "###.trim(),
        false => r###"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
            "###.trim()
    }
}
