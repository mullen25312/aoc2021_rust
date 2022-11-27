// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust

// extern crate pathfinding;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn neighbours(&self, cavern: &Vec<Vec<i32>>, mode: &i32) -> Vec<(Pos, i32)> {
        let size_of_cavern: i32 = cavern.len() as i32;
        let &Pos(x, y) = self;

        let mut neighbors: Vec<Pos> = Vec::new();
        if (x - 1) >= 0 {
            neighbors.push(Pos(self.0 - 1, self.1));
        }
        if (x + 1) < size_of_cavern * mode {
            neighbors.push(Pos(self.0 + 1, self.1));
        }
        if (y - 1) >= 0 {
            neighbors.push(Pos(self.0, self.1 - 1));
        }
        if (y + 1) < size_of_cavern * mode {
            neighbors.push(Pos(self.0, self.1 + 1));
        }

        neighbors.into_iter().map(|p| (p, value_of_cavern(&cavern, p))).collect()
    }
}

fn value_of_cavern(cavern: &Vec<Vec<i32>>, p: Pos) -> i32 {
    let size_of_cavern: i32 = cavern.len() as i32;

    let rem_x: usize = (p.0 % size_of_cavern) as usize;
    let rem_y: usize = (p.1 % size_of_cavern) as usize;

    (cavern[rem_y][rem_x] - 1 + p.0 / (size_of_cavern) + p.1 / (size_of_cavern)) % 9 + 1
}

// daily puzzle day: d15
fn main() {
    println!("########### d15 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let size_of_cavern: usize = lines.len();
    let mut cavern: Vec<Vec<i32>> = vec![vec![0; size_of_cavern]; size_of_cavern];

    for (idx, line) in lines.iter().enumerate() {
        cavern[idx] = line.chars().map(|c| c.to_digit(10).unwrap_or(0) as i32).collect();
    }

    // part one
    let tile_number1: i32 = 1;
    let goal1: Pos = Pos(cavern.len() as i32 * tile_number1 - 1, cavern.len() as i32 * tile_number1 - 1);
    let result1 = dijkstra(&Pos(0, 0), |p| p.neighbours(&cavern, &tile_number1), |p| *p == goal1);
    println!("Result of part one: {:?}", result1.unwrap_or_default().1);

    // part two
    let tile_number2: i32 = 5;
    let goal2: Pos = Pos(cavern.len() as i32 * tile_number2 - 1, cavern.len() as i32 * tile_number2 - 1);
    let result2 = dijkstra(&Pos(0, 0), |p| p.neighbours(&cavern, &tile_number2), |p| *p == goal2);
    println!("Result of part two: {}", result2.unwrap_or_default().1);
}
