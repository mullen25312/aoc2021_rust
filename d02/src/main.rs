// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust
// daily puzzle day: d02
fn main() {
    println!("########### d02 ###########");

    let file = String::from("./demo.txt");
    // let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    //let data: Vec<i32> = contents.lines().map(str::parse).map(Result::unwrap).collect();
    let lines: Vec<&str> = contents.split("\r\n").collect();

    let mut movement: Vec<&str> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();

    let number_of_moves: usize = lines.len();

    for idx in 0..number_of_moves {
        let line_data: Vec<&str> = lines[idx].split(" ").collect();
        movement.push(line_data[0]);
        distances.push(line_data[1].trim().to_string().parse().unwrap());
    }

    // part one
    let mut pos: i32 = 0;
    let mut dep: i32 = 0;
    for idx in 0..number_of_moves {
        if movement[idx] == "forward" {
            pos += distances[idx];
        } else if movement[idx] == "down" {
            dep += distances[idx];
        } else if movement[idx] == "up" {
            dep -= distances[idx];
        }
    }

    println!("Result of part one: {}", pos * dep);

    // part two
    let mut pos: i32 = 0;
    let mut dep: i32 = 0;
    let mut aim: i32 = 0;
    for idx in 0..number_of_moves {
        if movement[idx] == "forward" {
            pos += distances[idx];
            dep += aim * distances[idx];
        } else if movement[idx] == "down" {
            aim += distances[idx];
        } else if movement[idx] == "up" {
            aim -= distances[idx];
        }
    }

    println!("Result of part two: {}", pos * dep);
}
