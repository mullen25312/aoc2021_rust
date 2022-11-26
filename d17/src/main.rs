// advent of code website: https://adventofcode.com/2021
// github: https://github.com/mullen25312/aoc2021_rust

fn simulate(mut vel: [i32; 2], target_area: [[i32; 2]; 2]) -> Option<Vec<[i32; 2]>> {
    let mut pos: [i32; 2] = [0, 0];
    let mut sol: Vec<[i32; 2]> = Vec::new();

    while !((target_area[0][0] <= pos[0] && pos[0] <= target_area[0][1])
        && (target_area[1][0] <= pos[1] && pos[1] <= target_area[1][1]))
    {
        pos[0] += vel[0];
        pos[1] += vel[1];
        vel[0] -= vel[0].signum();
        vel[1] -= 1;

        sol.push(pos.clone());

        if pos[0] > target_area[0][1] || pos[1] < target_area[1][0] {
            return None;
        }
    }
    Some(sol)
}

// daily puzzle day: d13
fn main() {
    println!("########### d13 ###########");

    // let target_area: [[i32; 2]; 2] = [[20, 30], [-10, -5]]; // demo
    let target_area: [[i32; 2]; 2] = [[155, 215], [-132, -72]]; // input

    // part one
    let mut sols1: Vec<Vec<[i32; 2]>> = Vec::new();
    let mut vel_min: [i32; 2] = [0, 0];
    let mut vel_max: [i32; 2] = [50, 150];

    for velx in vel_min[0]..vel_max[0] {
        for vely in vel_min[1]..vel_max[1] {
            match simulate([velx, vely], target_area) {
                Some(x) => sols1.push(x),
                None => (),
            }
        }
    }

    let mut max_heights: Vec<i32> = Vec::new();
    for sol in sols1 {
        let tmp: [i32; 2] = *sol.iter().max_by_key(|pos| pos[1]).unwrap();
        max_heights.push(tmp[1]);
    }

    println!("Result of part one: {}", max_heights.iter().max().unwrap());

    // part two
    let mut sols2: Vec<Vec<[i32; 2]>> = Vec::new();
    vel_min = [0, target_area[1][0]];
    vel_max = [target_area[0][1] * 5 / 4, 250];

    for velx in vel_min[0]..vel_max[0] {
        for vely in vel_min[1]..vel_max[1] {
            match simulate([velx, vely], target_area) {
                Some(x) => sols2.push(x),
                None => (),
            }
        }
    }
    println!("Result of part two: {}", sols2.len());
}
