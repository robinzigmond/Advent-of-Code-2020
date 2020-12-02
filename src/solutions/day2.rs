use std::fs::File;
use std::io::prelude::*;

struct Policy {
    min: usize,
    max: usize,
    char: char,
}

fn read_file() -> Vec<(Policy, String)> {
    let mut file = File::open("./input/input2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|l| parse_line(l.to_owned())).collect()
}

fn parse_line(l: String) -> (Policy, String) {
    let v: Vec<&str> = l.split(": ").collect();
    let policy = v[0];
    let password = v[1];
    (parse_policy(policy), password.to_owned())
}

fn parse_policy(p: &str) -> Policy {
    let v: Vec<&str> = p.split(' ').collect();
    let range = v[0];
    let char = v[1];
    let ends: Vec<&str> = range.split('-').collect();
    Policy {
        min: ends[0].parse().unwrap(),
        max: ends[1].parse().unwrap(),
        char: char.chars().nth(0).unwrap(),
    }
}

fn is_valid(policy: &Policy, pw: &str) -> bool {
    let actual_number = pw.chars().filter(|c| c == &policy.char).count();
    policy.min <= actual_number && policy.max >= actual_number
}

fn solve_part_1(v: Vec<(Policy, String)>) -> usize {
    v.iter().filter(|(pol, pw)| is_valid(pol, pw)).count()
}

pub fn part_1() -> usize {
    let info = read_file();
    solve_part_1(info)
}

fn is_valid_new(policy: &Policy, pw: &str) -> bool {
    let get_char = |pos| pw.chars().nth(pos - 1).unwrap();
    let is_correct = |pos| get_char(pos) == policy.char;
    is_correct(policy.min) != is_correct(policy.max)
}

fn solve_part_2(v: Vec<(Policy, String)>) -> usize {
    v.iter().filter(|(pol, pw)| is_valid_new(pol, pw)).count()
}

pub fn part_2() -> usize {
    let info = read_file();
    solve_part_2(info)
}
