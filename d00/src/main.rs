// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust

// daily puzzle day: d00 (advent of code 2020 day 1 as template)
fn main() {
    println!("########### d00 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let data: Vec<i32> = contents.lines().map(str::parse).map(Result::unwrap).collect();
    let data_size: usize = data.len();

    // part one
    let mut result1: i32 = 0;
    for idx1 in 0..data_size {
        for idx2 in 0..data_size {
            if data[idx1] + data[idx2] == 2020 {
                result1 = data[idx1] * data[idx2];
            }
        }
    }
    println!("Result of part one: {}", result1);

    // part two
    let mut result2: i32 = 0;
    for idx1 in 0..data_size {
        for idx2 in 0..data_size {
            for idx3 in 0..data_size {
                if data[idx1] + data[idx2] + data[idx3] == 2020 {
                    result2 = data[idx1] * data[idx2] * data[idx3];
                }
            }
        }
    }
    println!("Result of part two: {}", result2);
}
