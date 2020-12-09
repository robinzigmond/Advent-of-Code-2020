use std::fs::File;
use std::io::prelude::*;

fn read_file() -> Vec<usize> {
    let mut file = File::open("./input/input9.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|n| n.parse().unwrap()).collect()
}

fn pair_sums_to(v: &[usize], n: usize) -> bool {
    let length = v.len();
    for i in 0..length {
        for j in i..length {
            if v[i] + v[j] == n {
                return true;
            }
        }
    }
    false
}

fn solve_part_1(nums: &Vec<usize>) -> usize {
    for i in 25..nums.len() {
        let n = nums[i];
        let previous_twenty_five = &nums[i - 25..i];
        if !pair_sums_to(previous_twenty_five, n) {
            return n;
        }
    }
    panic!("couldn't find a solution!");
}

pub fn part_1() -> usize {
    let nums = read_file();
    solve_part_1(&nums)
}

fn subset_sum(nums: &[usize], target: usize) -> &[usize] {
    for i in 0..(nums.len() - 1) {
        let mut total = nums[i];
        let mut next_index = i;
        while total < target {
            next_index += 1;
            total += nums[next_index];
        }
        if total == target {
            return &nums[i..(next_index + 1)];
        }
    }
    panic!("couldn't find a subset with the right total!");
}

fn solve_part_2(nums: Vec<usize>) -> usize {
    let target = solve_part_1(&nums);
    let subset = subset_sum(&nums, target);
    subset.iter().max().unwrap() + subset.iter().min().unwrap()
}

pub fn part_2() -> usize {
    let nums = read_file();
    solve_part_2(nums)
}
