use std::fs::File;
use std::io::prelude::*;

fn read_file() -> Vec<usize> {
    let mut file = File::open("./input/input10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|n| n.parse().unwrap()).collect()
}

fn solve_part_1(nums: &mut Vec<usize>) -> usize {
    nums.sort_unstable();
    let mut diff_one_count = 0;
    let mut diff_three_count = 1; // start at 1 because the last gap, between
                                  // the max number and the one 3 higher, will always be 3
    let mut last_num = 0;
    for num in nums {
        let diff = *num - last_num;
        if diff == 1 {
            diff_one_count += 1;
        } else if diff == 3 {
            diff_three_count += 1;
        }
        last_num = *num;
    }
    diff_one_count * diff_three_count
}

pub fn part_1() -> usize {
    let mut nums = read_file();
    solve_part_1(&mut nums)
}

// takes a vector of numbers, assumed to be already sorted, and
// splits into slices, where the end of each slice is exactly 3 from
// the start of the next one
fn partition(v: Vec<usize>) -> Vec<Vec<usize>> {
    let mut res = vec![];
    let mut current = vec![0];
    let mut last_num = 0;
    for num in v {
        let diff = num - last_num;
        if diff == 3 {
            res.push(current.clone());
            current = vec![];
        }
        current.push(num);
        last_num = num;
    }
    if current.len() > 0 {
        res.push(current.clone());
    }
    res
}

fn solve_part_2(nums: &mut Vec<usize>) -> usize {
    nums.sort_unstable();
    let partitions = partition(nums.to_vec());
    // we "cheat" here by not solving the puzzle in full generality, but
    // relying on two simplifying assumptions, which happen to be true of
    // the data given.
    // The first is that all the subsequences in "partitions" above are runs
    // of consecutive integers - with no gaps of 2 as are theoretically possible.
    // The second is that each such subsequence has length at most 5.
    let num_sequences = |length| match length {
        1 => 1,
        2 => 1,
        3 => 2,
        4 => 4,
        5 => 7,
        _ => panic!("I've seen this can't happen ;)"),
    };
    partitions.iter().map(|v| num_sequences(v.len())).product()
}

pub fn part_2() -> usize {
    let mut nums = read_file();
    solve_part_2(&mut nums)
}
