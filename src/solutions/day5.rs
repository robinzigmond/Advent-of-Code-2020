use std::fs::File;
use std::io::prelude::*;

fn read_file() -> Vec<u16> {
    let mut file = File::open("./input/input5.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(read_as_binary).collect()
}

// to get the seat ID, rather than split into row and column, we
// can just use the fact that (row * 8 + column) is the same as
// reading each line as a 10-digit binary number, with 'R' and 'B'
// translating to 1 and 'L' and 'F' to 0
fn read_as_binary(s: &str) -> u16 {
    let translated = s
        .replace("R", "1")
        .replace("B", "1")
        .replace("L", "0")
        .replace("F", "0");
    u16::from_str_radix(&translated, 2).unwrap()
}

fn solve_part_1(ids: &Vec<u16>) -> u16 {
    *ids.into_iter().max().unwrap()
}

pub fn part_1() -> u16 {
    let ids = read_file();
    solve_part_1(&ids)
}

fn solve_part_2(ids: Vec<u16>) -> u16 {
    let max = solve_part_1(&ids);
    let all_possible = 1..(max + 1);
    let available = |id| !ids.contains(&id);
    let mut answer: Option<u16> = None;
    for id in all_possible {
        let preceding = id - 1;
        let following = id + 1;
        if available(id) && !available(preceding) && !available(following) {
            answer = Some(id);
            break;
        }
    }
    answer.unwrap()
}

pub fn part_2() -> u16 {
    let ids = read_file();
    solve_part_2(ids)
}
