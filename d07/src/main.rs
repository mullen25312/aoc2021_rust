// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust

fn sum_of_ints(number: i64) -> i64 {
    number * (number + 1) / 2
}

// daily puzzle day: d07
fn main() {
    println!("########### d07 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let mut data: Vec<i32> = contents.trim().split(",").map(str::parse).map(Result::unwrap).collect();

    // part one
    data.sort();
    let median = data[data.len() / 2 as usize];

    let mut result1: i32 = 0;
    for entry in &data {
        result1 += (entry - median).abs();
    }

    println!("Result of part one: {}", result1);

    // part two
    let a1: i64 = *data.first().unwrap() as i64;
    let a2: i64 = *data.last().unwrap() as i64;

    let mut result2: i64 = std::i64::MAX;
    for idx in a1..a2 {
        let mut tmp: i64 = 0;
        for entry in &data {
            tmp += sum_of_ints((*entry as i64 - idx).abs());
        }
        result2 = std::cmp::min(result2, tmp);
    }
    println!("Result of part two: {}", result2);
}
