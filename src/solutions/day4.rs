use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn read_file() -> Vec<Passport> {
    let mut file = File::open("./input/input4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\r\n\r\n").map(parse_group).collect()
}

fn parse_group(g: &str) -> Passport {
    let parts = g.split_whitespace().map(parse_part);
    let mut byr = None;
    let mut iyr = None;
    let mut eyr = None;
    let mut hgt = None;
    let mut hcl = None;
    let mut ecl = None;
    let mut pid = None;
    let mut cid = None;

    for (name, val) in parts {
        match name {
            "byr" => {
                byr = Some(val.parse().unwrap());
            }
            "iyr" => {
                iyr = Some(val.parse().unwrap());
            }
            "eyr" => {
                eyr = Some(val.parse().unwrap());
            }
            "hgt" => {
                hgt = Some(val.to_owned());
            }
            "hcl" => {
                hcl = Some(val.to_owned());
            }
            "ecl" => {
                ecl = Some(val.to_owned());
            }
            "pid" => {
                pid = Some(val.to_owned());
            }
            "cid" => {
                cid = Some(val.to_owned());
            }
            s => {
                panic!(format!("unknown password part: {}", s))
            }
        }
    }

    Passport {
        byr,
        iyr,
        eyr,
        hgt,
        hcl,
        ecl,
        pid,
        cid,
    }
}

fn parse_part(p: &str) -> (&str, &str) {
    let parts: Vec<&str> = p.split(':').take(2).collect();
    (parts[0], parts[1])
}

fn has_all_required_fields(p: &Passport) -> bool {
    p.byr != None
        && p.iyr != None
        && p.eyr != None
        && p.hgt != None
        && p.hcl != None
        && p.ecl != None
        && p.pid != None
}

fn solve_part_1(v: Vec<Passport>) -> usize {
    v.into_iter().filter(has_all_required_fields).count()
}

pub fn part_1() -> usize {
    let passports = read_file();
    solve_part_1(passports)
}

fn byr_valid(year: Option<u16>) -> bool {
    match year {
        Some(y) => y >= 1920 && y <= 2002,
        None => false,
    }
}

fn iyr_valid(year: Option<u16>) -> bool {
    match year {
        Some(y) => y >= 2010 && y <= 2020,
        None => false,
    }
}

fn eyr_valid(year: Option<u16>) -> bool {
    match year {
        Some(y) => y >= 2020 && y <= 2030,
        None => false,
    }
}

fn is_valid_height_str(height_str: &String) -> bool {
    let mut numeric_part = height_str.chars().take_while(|c| c.is_digit(10));
    let number: u16 = numeric_part.by_ref().collect::<String>().parse().unwrap();
    let suffix = height_str[format!("{}", number).len()..].to_owned();
    match suffix.as_str() {
        "cm" => number >= 150 && number <= 193,
        "in" => number >= 59 && number <= 76,
        _ => false,
    }
}

fn hgt_valid(height: &Option<String>) -> bool {
    match height {
        Some(height_str) => is_valid_height_str(height_str),
        None => false,
    }
}

fn is_valid_colour_str(colour_str: &String) -> bool {
    let first_char = &colour_str[0..1];
    let after_hash = &colour_str[1..];
    first_char == "#" && after_hash.len() == 6 && after_hash.chars().all(|c| c.is_digit(16))
}

fn hcl_valid(colour: &Option<String>) -> bool {
    match colour {
        Some(colour_str) => is_valid_colour_str(colour_str),
        None => false,
    }
}

fn ecl_valid(colour: &Option<String>) -> bool {
    match colour {
        Some(str) => {
            let allowed = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            allowed.contains(&str.as_str())
        }
        None => false,
    }
}

fn pid_valid(pid: &Option<String>) -> bool {
    match pid {
        Some(id) => id.len() == 9 && id.chars().all(|c| c.is_digit(10)),
        None => false,
    }
}

fn is_valid(p: &Passport) -> bool {
    byr_valid(p.byr)
        && iyr_valid(p.iyr)
        && eyr_valid(p.eyr)
        && hgt_valid(&p.hgt)
        && hcl_valid(&p.hcl)
        && ecl_valid(&p.ecl)
        && pid_valid(&p.pid)
}

fn solve_part_2(v: Vec<Passport>) -> usize {
    v.into_iter().filter(is_valid).count()
}

pub fn part_2() -> usize {
    let passports = read_file();
    solve_part_2(passports)
}
