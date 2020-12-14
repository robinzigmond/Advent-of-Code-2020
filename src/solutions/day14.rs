use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
enum BitSetting {
    Zero,
    One,
    Leave,
}

impl BitSetting {
    fn from_string(s: &str) -> [BitSetting; 36] {
        let as_vec: Vec<BitSetting> = s
            .chars()
            .rev()
            .map(|c| match c {
                '0' => BitSetting::Zero,
                '1' => BitSetting::One,
                'X' => BitSetting::Leave,
                c => panic!("unexpected bit character: {}", c),
            })
            .collect();
        as_vec.try_into().unwrap()
    }
}

fn apply_mask(num: usize, mask: [BitSetting; 36]) -> usize {
    let mut as_binary = format!("{:036b}", num).into_bytes();
    let mut str_index = 36;
    for setting in mask.iter() {
        str_index -= 1;
        match setting {
            BitSetting::Zero => as_binary[str_index] = '0' as u8,
            BitSetting::One => as_binary[str_index] = '1' as u8,
            BitSetting::Leave => (),
        }
    }
    usize::from_str_radix(
        &as_binary.iter().map(|&num| num as char).collect::<String>()[0..],
        2,
    )
    .unwrap()
}

fn apply_mask_new(num: usize, mask: [BitSetting; 36]) -> Vec<usize> {
    let mut as_binary = format!("{:036b}", num).into_bytes();
    let mut str_index = 36;
    let mut floating_indices = Vec::new();
    for setting in mask.iter() {
        str_index -= 1;
        match setting {
            BitSetting::Zero => (),
            BitSetting::One => {
                as_binary[str_index] = '1' as u8;
            }
            BitSetting::Leave => {
                floating_indices.push(str_index);
            }
        }
    }
    let mut result = vec![as_binary];
    for i in floating_indices {
        result = result
            .iter()
            .flat_map(|bin_str| {
                let mut with_zero = bin_str.clone();
                let mut with_one = bin_str.clone();
                with_zero[i] = '0' as u8;
                with_one[i] = '1' as u8;
                vec![with_zero, with_one]
            })
            .collect();
    }

    result
        .iter()
        .map(|bin_str| {
            usize::from_str_radix(
                &bin_str.iter().map(|&num| num as char).collect::<String>()[0..],
                2,
            )
            .unwrap()
        })
        .collect()
}

struct Memory {
    bitmask: [BitSetting; 36],
    content: HashMap<usize, usize>,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            bitmask: [BitSetting::Leave; 36],
            content: HashMap::new(),
        }
    }

    fn process_string(&mut self, s: String) -> () {
        if &s[0..7] == "mask = " {
            self.bitmask = BitSetting::from_string(&s[7..]);
        } else {
            // can assume line has the form mem[XXX] = YYY
            let parts: Vec<&str> = s.split("] = ").collect();
            let key = parts[0][4..].parse().unwrap();
            let value = apply_mask(parts[1].parse().unwrap(), self.bitmask);
            self.content.insert(key, value);
        }
    }

    fn process_all(&mut self, v: Vec<String>) -> () {
        for s in v {
            self.process_string(s);
        }
    }

    fn process_string_new(&mut self, s: String) -> () {
        if &s[0..7] == "mask = " {
            self.bitmask = BitSetting::from_string(&s[7..]);
        } else {
            // can assume line has the form mem[XXX] = YYY
            let parts: Vec<&str> = s.split("] = ").collect();
            let keys = apply_mask_new(parts[0][4..].parse().unwrap(), self.bitmask);
            let value = parts[1].parse().unwrap();
            for key in keys {
                self.content.insert(key, value);
            }
        }
    }

    fn process_all_new(&mut self, v: Vec<String>) -> () {
        for s in v {
            self.process_string_new(s);
        }
    }
}

fn read_file() -> Vec<String> {
    let mut file = File::open("./input/input14.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|l| l.to_owned()).collect()
}

fn solve_part_1(info: Vec<String>) -> usize {
    let mut memory = Memory::new();
    memory.process_all(info);
    memory.content.values().sum()
}

pub fn part_1() -> usize {
    let info = read_file();
    solve_part_1(info)
}

fn solve_part_2(info: Vec<String>) -> usize {
    let mut memory = Memory::new();
    memory.process_all_new(info);
    memory.content.values().sum()
}

pub fn part_2() -> usize {
    let info = read_file();
    solve_part_2(info)
}
