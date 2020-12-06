use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type PersonAnswers = HashMap<char, bool>;

struct GroupAnswers {
    all: Vec<PersonAnswers>,
}

impl GroupAnswers {
    fn total_at_least_one(&self) -> usize {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let mut total = 0;
        for c in alphabet.chars() {
            for person in &self.all {
                if person.get(&c) == Some(&true) {
                    total += 1;
                    break;
                }
            }
        }
        total
    }

    fn total_all(&self) -> usize {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let mut total = 26;
        for c in alphabet.chars() {
            for person in &self.all {
                if person.get(&c) != Some(&true) {
                    total -= 1;
                    break;
                }
            }
        }
        total
    }
}

fn read_file() -> Vec<GroupAnswers> {
    let mut file = File::open("./input/input6.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .lines()
        .group_by(|s| s.is_empty())
        .into_iter()
        .filter(|(k, _g)| !k)
        .map(|(_k, g)| parse_group(g.collect()))
        .collect()
}

fn parse_group(group: Vec<&str>) -> GroupAnswers {
    GroupAnswers {
        all: group.into_iter().map(parse_answers).collect(),
    }
}

fn parse_answers(s: &str) -> PersonAnswers {
    let mut answers = HashMap::new();
    for c in s.chars() {
        answers.insert(c, true);
    }
    answers
}

fn solve_part_1(info: Vec<GroupAnswers>) -> usize {
    info.iter()
        .map(|answers| answers.total_at_least_one())
        .sum()
}

pub fn part_1() -> usize {
    let info = read_file();
    solve_part_1(info)
}

fn solve_part_2(info: Vec<GroupAnswers>) -> usize {
    info.iter().map(|answers| answers.total_all()).sum()
}

pub fn part_2() -> usize {
    let info = read_file();
    solve_part_2(info)
}
