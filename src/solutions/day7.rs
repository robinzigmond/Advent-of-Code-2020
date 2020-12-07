use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type BagDescription = String;

type BagRules = HashMap<BagDescription, Vec<(usize, BagDescription)>>;

fn read_file() -> BagRules {
    let mut file = File::open("./input/input7.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    parse_rules(contents)
}

fn parse_rules(s: String) -> BagRules {
    let mut rules = HashMap::new();
    for l in s.lines() {
        let (container, contained) = parse_line(l.to_string());
        rules.insert(container, contained);
    }
    rules
}

fn parse_line(s: String) -> (BagDescription, Vec<(usize, BagDescription)>) {
    let parts: Vec<&str> = s.split(" bags contain ").collect();
    let container = parts[0].to_string();
    let contained = parts[1].to_string();
    if contained == String::from("no other bags.") {
        return (container, Vec::new());
    }
    let all_parts = contained.split(", ");
    (
        container,
        all_parts.map(|s| parse_part(s.to_string())).collect(),
    )
}

fn parse_part(s: String) -> (usize, BagDescription) {
    let words: Vec<&str> = s.split(" ").collect();
    let num_words = words.len();
    let number = words[0].parse().unwrap();
    let bag_description = words[1..num_words - 1].join(" ");
    (number, bag_description)
}

fn is_valid(target: &BagDescription, container: BagDescription, rules: &BagRules) -> bool {
    let can_contain = rules.get(&container);
    match can_contain {
        Some(other_bags) => {
            let mut ans = false;
            for (_, next_bag) in other_bags.iter() {
                if next_bag == target {
                    ans = true;
                    break;
                }
                if is_valid(target, next_bag.to_string(), rules) {
                    ans = true;
                    break;
                }
            }
            ans
        }
        None => false,
    }
}

fn solve_part_1(rules: BagRules) -> usize {
    let want = String::from("shiny gold");
    let mut number = 0;
    for k in rules.keys() {
        if is_valid(&want, k.to_string(), &rules) {
            number += 1;
        }
    }
    number
}

pub fn part_1() -> usize {
    let rules = read_file();
    solve_part_1(rules)
}

fn get_bag_total(bag: BagDescription, rules: &BagRules) -> usize {
    let mut total = 0;
    let contained_in = rules.get(&bag);
    match contained_in {
        Some(bags) => {
            for (num, inner_bag) in bags {
                total += num * (get_bag_total(inner_bag.to_string(), rules) + 1);
            }
        }
        None => {}
    }
    total
}

fn solve_part_2(rules: BagRules) -> usize {
    get_bag_total(String::from("shiny gold"), &rules)
}

pub fn part_2() -> usize {
    let rules = read_file();
    solve_part_2(rules)
}
