// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust

// daily puzzle day: d03
fn main() {
    println!("########### d03 ###########");

    let file = String::from("./demo.txt");
    // let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\r\n").collect();

    let mut data: Vec<u16> = Vec::new();
    for idx in 0..lines.len() {
        data.push(u16::from_str_radix(lines[idx].trim(), 2).expect("Not a binary number!"));
    }

    let binary_size: usize = lines[0].len();

    // part one
    let mut result1a: u16 = 0;

    for idx in 0..binary_size {
        let mut tmp: u16 = 0;
        for entry in &data {
            tmp += (entry & (1 << idx)) >> idx;
        }
        if usize::from(tmp << 1) > data.len() {
            result1a += 1 << idx;
        }
    }
    let result1b: u16 = (!result1a << (16 - binary_size)) >> (16 - binary_size);

    println!("Result of part one: {}", i32::from(result1a) * i32::from(result1b));

    // part two
    let result2: i32 = 0;
    println!("Result of part two: {}", result2);
}
