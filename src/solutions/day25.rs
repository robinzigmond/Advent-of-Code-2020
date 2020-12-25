use std::fs::File;
use std::io::prelude::*;

fn read_file() -> (usize, usize) {
    let mut file = File::open("./input/input25.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let vals: Vec<usize> = contents.lines().map(|n| n.parse().unwrap()).collect();

    (vals[0], vals[1])
}

fn discrete_logarithm(target: usize, p: usize, base: usize) -> usize {
    let mut power = 1;
    let mut current = base;
    loop {
        if current == target {
            return power;
        }
        power += 1;
        current = (current * base) % p;
    }
}

fn solve_part_1(nums: (usize, usize)) -> usize {
    let base = 20201227;
    let subject = 7;
    let second_log = discrete_logarithm(nums.1, base, subject);

    // use naive loop to calculate power
    let mut res = 1;
    for _ in 0..second_log {
        res = (res * nums.0) % base;
    }
    res
}

pub fn part_1() -> usize {
    let nums = read_file();
    solve_part_1(nums)
}
