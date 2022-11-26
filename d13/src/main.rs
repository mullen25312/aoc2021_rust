// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust

use std::collections::HashSet;

fn generate_display_string(paper: HashSet<[i32; 2]>) -> String {
    let mut output = String::from("");
    let max_x: &[i32; 2] = paper.iter().max_by_key(|n| n[0]).unwrap();
    let max_y: &[i32; 2] = paper.iter().max_by_key(|n| n[1]).unwrap();
    for y in 0..max_y[1] {
        for x in 0..max_x[0] {
            if paper.contains(&[x, y]) {
                output.push('#');
            } else {
                output.push('.')
            }
        }
        output.push('\n');
    }
    output
}

fn fold_action(mut paper: HashSet<[i32; 2]>, fold: [i32; 2]) -> HashSet<[i32; 2]> {
    let tmp: HashSet<[i32; 2]> = paper.clone();
    if fold[0] != 0 {
        for point in tmp {
            if point[0] >= fold[0] {
                paper.insert([fold[0] - (point[0] - fold[0]), point[1]]);
                paper.remove(&point);
            }
        }
    } else if fold[1] != 0 {
        for point in tmp {
            if point[1] >= fold[1] {
                paper.insert([point[0], fold[1] - (point[1] - fold[1])]);
                paper.remove(&point);
            }
        }
    }
    paper
}

// daily puzzle day: d13
fn main() {
    println!("########### d13 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut paper1: HashSet<[i32; 2]> = HashSet::new();
    for line in lines {
        let line_data: Vec<&str> = line.split(",").collect();
        paper1.insert([
            line_data[0].trim().to_string().parse().unwrap(),
            line_data[1].trim().to_string().parse().unwrap(),
        ]);
    }
    let mut paper2: HashSet<[i32; 2]> = paper1.clone();

    // let folds: Vec<[i32; 2]> = vec![[0, 7], [5, 0]];
    let folds: Vec<[i32; 2]> = vec![
        [655, 0],
        [0, 447],
        [327, 0],
        [0, 223],
        [163, 0],
        [0, 111],
        [81, 0],
        [0, 55],
        [40, 0],
        [0, 27],
        [0, 13],
        [0, 6],
    ];

    // part one
    let result1: i32 = fold_action(paper1, folds[0]).len() as i32;
    println!("Result of part one: {}", result1);

    // part two
    let result2: String = String::from("FAGURZHE");

    for fold in folds {
        paper2 = fold_action(paper2, fold);
    }

    println!("Result of part two: {}", result2);
    print!("{}", generate_display_string(paper2));
}
