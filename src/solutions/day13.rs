use ring_algorithm::chinese_remainder_theorem;
use std::fs::File;
use std::io::prelude::*;

struct PuzzleInfo {
    current_time: usize,
    buses: Vec<Option<usize>>,
}

fn read_file() -> PuzzleInfo {
    let mut file = File::open("./input/input13.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();
    let current_time = lines.nth(0).unwrap().parse().unwrap();
    let buses = lines
        .nth(0)
        .unwrap()
        .split(",")
        .map(|s| s.parse().ok())
        .collect();
    PuzzleInfo {
        current_time,
        buses,
    }
}

fn find_bus(buses: &Vec<Option<usize>>, time_to_test: usize) -> Option<usize> {
    let found_option = buses.iter().find(|b| match b {
        None => false,
        Some(id) => time_to_test % id == 0,
    });
    match found_option {
        Some(&bus) => bus,
        None => None,
    }
}

fn solve_part_1(info: PuzzleInfo) -> usize {
    let PuzzleInfo {
        current_time,
        buses,
    } = info;
    let mut counter = 0;
    loop {
        if let Some(id) = find_bus(&buses, current_time + counter) {
            return id * counter;
        }
        counter += 1;
    }
}

pub fn part_1() -> usize {
    let info = read_file();
    solve_part_1(info)
}

fn solve_part_2(buses: Vec<Option<usize>>) -> isize {
    let (remainders, moduli): (Vec<isize>, Vec<isize>) = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| match b {
            None => false,
            _ => true,
        })
        .map(|(i, m)| (-(i as isize), m.unwrap() as isize))
        .unzip();
    chinese_remainder_theorem(&remainders[0..], &moduli[0..]).unwrap()
}

pub fn part_2() -> isize {
    let info = read_file();
    solve_part_2(info.buses)
}
