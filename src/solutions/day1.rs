use std::fs::File;
use std::io::prelude::*;

fn read_file() -> Vec<i32> {
    let mut file = File::open("./input/input1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|n| n.parse().unwrap()).collect()
}

fn solve_part_1(v: Vec<i32>) -> i32 {
    let mut ans = 0;
    for n in v.iter() {
        let res = 2020 - n;
        // the below would generate a false positive if 1010 were in the input,
        // but I checked it isn't there - so we can ignore this!
        if let Some(found) = v.iter().find(|m| *m == &res) {
            ans = n * found;
            break;
        }
    }
    ans
}

fn solve_part_2(v: Vec<i32>) -> i32 {
    let mut ans = 0;
    for n in v.iter() {
        for m in v.iter() {
            let res = 2020 - m - n;
            // to avoid false positives, we assume there are no duplicates
            if res < 0 || m == n {
                continue;
            }
            if let Some(found) = v.iter().find(|k| *k == &res) {
                ans = n * m * found;
                break;
            }
        }
    }
    ans
}

pub fn part_1() -> i32 {
    let nums = read_file();
    solve_part_1(nums)
}

pub fn part_2() -> i32 {
    let nums = read_file();
    solve_part_2(nums)
}
