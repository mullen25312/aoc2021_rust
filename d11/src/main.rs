// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust

fn snychronized(octopuses: [[i32; 10]; 10]) -> bool {
    for y in 0..10 {
        for x in 0..10 {
            if octopuses[y][x] != 0 {
                return false;
            }
        }
    }
    return true;
}

fn flash(flasher: [i32; 2], mut octopuses: [[i32; 10]; 10]) -> [[i32; 10]; 10] {
    let neighbors: [[i32; 2]; 8] = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
    for neighbor in neighbors {
        if 0 <= flasher[0] + neighbor[0]
            && flasher[0] + neighbor[0] < 10
            && 0 <= flasher[1] + neighbor[1]
            && flasher[1] + neighbor[1] < 10
        {
            if octopuses[(flasher[0] + neighbor[0]) as usize][(flasher[1] + neighbor[1]) as usize] != 0 {
                octopuses[(flasher[0] + neighbor[0]) as usize][(flasher[1] + neighbor[1]) as usize] += 1;
            }
        }
    }
    octopuses
}

fn increase_energy_level(mut octopuses: [[i32; 10]; 10]) -> [[i32; 10]; 10] {
    for y in 0..10 {
        for x in 0..10 {
            octopuses[y][x] += 1;
        }
    }
    octopuses
}

fn find_flasher(octopuses: [[i32; 10]; 10]) -> Option<[i32; 2]> {
    for y in 0..10 {
        for x in 0..10 {
            if octopuses[y][x] > 9 {
                return Some([y as i32, x as i32]);
            }
        }
    }
    return None;
}

// daily puzzle day: d11
fn main() {
    println!("########### d11 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut octopuses1: [[i32; 10]; 10] = [[0; 10]; 10];

    for (y, line) in lines.iter().enumerate() {
        for (x, number) in line.chars().into_iter().enumerate() {
            octopuses1[y][x] = number.to_string().parse().unwrap();
        }
    }
    let mut octopuses2: [[i32; 10]; 10] = octopuses1.clone();

    // part one
    let number_of_steps: i32 = 100;
    let mut result1: i32 = 0;

    for _ in 0..number_of_steps {
        octopuses1 = increase_energy_level(octopuses1);

        while let Some(flasher) = find_flasher(octopuses1) {
            octopuses1 = flash(flasher, octopuses1);
            result1 += 1;
            octopuses1[flasher[0] as usize][flasher[1] as usize] = 0;
        }
    }

    println!("Result of part one: {}", result1);

    // part two
    let mut result2: i32 = 0;
    while !snychronized(octopuses2) {
        octopuses2 = increase_energy_level(octopuses2);

        while let Some(flasher) = find_flasher(octopuses2) {
            octopuses2 = flash(flasher, octopuses2);
            result1 += 1;
            octopuses2[flasher[0] as usize][flasher[1] as usize] = 0;
        }
        result2 += 1;
    }
    println!("Result of part two: {}", result2);
}
