use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq)]
enum Ground {
    Tree,
    Open,
}

struct Run {
    content: Vec<Vec<Ground>>,
}

impl Run {
    fn ground_at(&self, row: usize, col: usize) -> &Ground {
        let width = self.content[0].len();
        &self.content[row][col % width]
    }

    fn slope_total(&self, right: usize, down: usize) -> usize {
        let mut row = 0;
        let mut col = 0;
        let mut tree_count = 0;
        while row < self.content.len() {
            let res = self.ground_at(row, col);
            if res == &Ground::Tree {
                tree_count += 1;
            }
            row += down;
            col += right;
        }
        tree_count
    }
}

fn read_file() -> Run {
    let mut file = File::open("./input/input3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Run {
        content: contents.lines().map(|l| parse_line(l.to_owned())).collect(),
    }
}

fn parse_line(l: String) -> Vec<Ground> {
    l.chars().map(parse_char).collect()
}

fn parse_char(c: char) -> Ground {
    match c {
        '.' => Ground::Open,
        '#' => Ground::Tree,
        c => panic!(format!("couldn't read character '{}'", c)),
    }
}

fn solve_part_1(r: Run) -> usize {
    r.slope_total(3, 1)
}

pub fn part_1() -> usize {
    let run = read_file();
    solve_part_1(run)
}

fn solve_part_2(r: Run) -> usize {
    r.slope_total(1, 1)
    * r.slope_total(3, 1)
    * r.slope_total(5, 1)
    * r.slope_total(7, 1)
    * r.slope_total(1, 2)
}

pub fn part_2() -> usize {
    let run = read_file();
    solve_part_2(run)
}
