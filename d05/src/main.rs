// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust

use std::collections::HashMap;

fn count_overlaps(grid: &HashMap<(i32, i32), i32>) -> i32 {
    let mut result: i32 = 0;
    for point in grid {
        if point.1 >= &2 {
            result += 1;
        }
    }
    result
}

fn generate_range(first: i32, second: i32) -> Vec<i32> {
    if first < second {
        (first..second + 1).collect()
    } else {
        (second..first + 1).rev().collect()
    }
}

fn get_line(start: (i32, i32), end: (i32, i32), mode: i32) -> Vec<(i32, i32)> {
    let x1: i32 = start.0;
    let y1: i32 = start.1;
    let x2: i32 = end.0;
    let y2: i32 = end.1;

    let mut x: Vec<i32> = Vec::new();
    let mut y: Vec<i32> = Vec::new();

    if x1 == x2 {
        y = generate_range(y1, y2);
        for _ in 0..y.len() {
            x.push(x1);
        }
    } else if y1 == y2 {
        x = generate_range(x1, x2);
        for _ in 0..x.len() {
            y.push(y1);
        }
    } else if mode != 0 {
        x = generate_range(x1, x2);
        y = generate_range(y1, y2);
    }

    std::iter::zip(x, y).collect()
}

// daily puzzle day: d05
fn main() {
    println!("########### d05 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut data: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in lines {
        let tmp: Vec<&str> = line.split(" -> ").collect();
        let start: Vec<i32> = tmp[0].split(",").map(|c| c.parse().unwrap()).collect();
        let end: Vec<i32> = tmp[1].split(",").map(|c| c.parse().unwrap()).collect();
        data.push(((start[0], start[1]), (end[0], end[1])));
    }

    // part one
    let mut grid1: HashMap<(i32, i32), i32> = HashMap::new();
    for entry in &data {
        for point in get_line((entry.0 .0, entry.0 .1), (entry.1 .0, entry.1 .1), 0) {
            if grid1.contains_key(&point) {
                grid1.entry(point).and_modify(|value| *value += 1);
            } else {
                grid1.insert(point, 1);
            }
        }
    }

    println!("Result of part one: {}", count_overlaps(&grid1));

    // part two
    let mut grid2: HashMap<(i32, i32), i32> = HashMap::new();
    for entry in &data {
        for point in get_line((entry.0 .0, entry.0 .1), (entry.1 .0, entry.1 .1), 1) {
            if grid2.contains_key(&point) {
                grid2.entry(point).and_modify(|value| *value += 1);
            } else {
                grid2.insert(point, 1);
            }
        }
    }

    println!("Result of part two: {}", count_overlaps(&grid2));
}
