// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust

// daily puzzle day: d02
fn main() {
    println!("########### d02 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let data: Vec<i32> = contents.lines().map(str::parse).map(Result::unwrap).collect();

    // part one
    let mut sum1: i32 = 0;
    for idx in 1..data.len() {
        if data[idx] - data[idx - 1] > 0 {
            sum1 += 1;
        }
    }
    println!("Result of part one: {}", sum1);

    // part two
    let mut filtered: Vec<i32> = Vec::new();
    for idx in 2..data.len() {
        filtered.push(data[idx] + data[idx - 1] + data[idx - 2]);
    }

    let mut sum2: i32 = 0;
    for idx in 1..filtered.len() {
        if filtered[idx] - filtered[idx - 1] > 0 {
            sum2 += 1;
        }
    }
    println!("Result of part two: {}", sum2);
}
