// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust

fn populate_fishes(mut fishes: [i64; 9], days: i32) -> [i64; 9] {
    for _ in 0..days {
        let tmp: i64 = fishes[0];
        for cycle in 1..fishes.len() {
            fishes[(cycle - 1) as usize] = fishes[cycle];
            fishes[cycle] = 0;
        }
        fishes[9 - 1] = tmp;
        fishes[9 - 1 - 2] += tmp;
    }
    fishes
}

// daily puzzle day: d06
fn main() {
    println!("########### d06 ###########");

    let file = String::from("./demo.txt");
    // let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let data: Vec<i32> = contents.trim().split(",").map(str::parse).map(Result::unwrap).collect();

    let mut fishes: [i64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for entry in data {
        fishes[entry as usize] += 1
    }

    // part one
    let result1: i64 = populate_fishes(fishes, 80).iter().sum();
    println!("Result of part one: {}", result1);

    // part two
    let result2: i64 = populate_fishes(fishes, 256).iter().sum();
    println!("Result of part two: {}", result2);
}
