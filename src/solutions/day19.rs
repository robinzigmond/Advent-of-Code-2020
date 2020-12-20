use itertools::Itertools;
use nom::{
    branch::alt, character::complete::char, combinator::map, multi::many1, sequence::tuple, IResult,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
enum Parser {
    ParseA,
    ParseB,
    Single(usize),
    Pair(usize, usize),
    Triple(usize, usize, usize),
    Choice(Box<Parser>, Box<Parser>),
    Many(Box<Parser>),
}

impl Parser {
    fn get_parser<'a>(
        &self,
        parsers: &'a HashMap<usize, Parser>,
    ) -> Box<dyn FnMut(&'a str) -> IResult<&'a str, ()> + 'a> {
        match self {
            Parser::ParseA => Box::new(map(char('a'), |_| ())),
            Parser::ParseB => Box::new(map(char('b'), |_| ())),
            Parser::Single(num) => Box::new(parsers.get(num).unwrap().get_parser(parsers)),
            Parser::Pair(num1, num2) => Box::new(map(
                tuple((
                    parsers.get(num1).unwrap().get_parser(parsers),
                    parsers.get(num2).unwrap().get_parser(parsers),
                )),
                |_| (),
            )),
            Parser::Triple(num1, num2, num3) => Box::new(map(
                tuple((
                    parsers.get(num1).unwrap().get_parser(parsers),
                    parsers.get(num2).unwrap().get_parser(parsers),
                    parsers.get(num3).unwrap().get_parser(parsers),
                )),
                |_| (),
            )),
            Parser::Choice(first, second) => {
                Box::new(alt((first.get_parser(parsers), second.get_parser(parsers))))
            }
            Parser::Many(repeated) => Box::new(map(many1(repeated.get_parser(parsers)), |_| ())),
        }
    }

    fn can_parse(&self, parsers: &HashMap<usize, Parser>, input: String) -> bool {
        let mut parser = self.get_parser(parsers);

        if let Ok(("", ())) = parser(&input) {
            return true;
        }
        false
    }
}

struct PuzzleInfo {
    parsers: HashMap<usize, Parser>,
    strings: Vec<String>,
}

fn read_file() -> PuzzleInfo {
    let mut file = File::open("./input/input19.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parts: Vec<String> = contents
        .lines()
        .group_by(|s| s.is_empty())
        .into_iter()
        .filter(|(k, _g)| !k)
        .map(|(_k, g)| g.collect::<Vec<&str>>().join("\n"))
        .collect();

    PuzzleInfo {
        parsers: build_parsers(parts[0].lines().map(|s| s.to_owned()).collect()),
        strings: parts[1].lines().map(|s| s.to_owned()).collect(),
    }
}

fn build_parsers(v: Vec<String>) -> HashMap<usize, Parser> {
    let mut parsers = HashMap::new();
    for s in v {
        let parts: Vec<&str> = s.split(": ").collect();
        let number = parts[0].parse().unwrap();
        let expr = parts[1];
        parsers.insert(number, build_parser(expr));
    }
    parsers
}

fn build_parser(s: &str) -> Parser {
    if s == "\"a\"" {
        Parser::ParseA
    } else if s == "\"b\"" {
        Parser::ParseB
    } else {
        let options: Vec<&str> = s.split(" | ").collect();
        if options.len() == 2 {
            Parser::Choice(
                Box::new(build_parser(options[0])),
                Box::new(build_parser(options[1])),
            )
        } else {
            let parts: Vec<&str> = s.split(" ").collect();
            if parts.len() > 2 {
                Parser::Triple(
                    parts[0].parse().unwrap(),
                    parts[1].parse().unwrap(),
                    parts[2].parse().unwrap(),
                )
            } else if parts.len() == 2 {
                Parser::Pair(parts[0].parse().unwrap(), parts[1].parse().unwrap())
            } else {
                Parser::Single(parts[0].parse().unwrap())
            }
        }
    }
}

fn solve_part_1(info: PuzzleInfo) -> usize {
    let PuzzleInfo { parsers, strings } = info;
    let zero_parser = parsers.get(&0).unwrap();
    strings
        .iter()
        .filter(|&s| zero_parser.can_parse(&parsers, s.to_string()))
        .count()
}

pub fn part_1() -> usize {
    let info = read_file();
    solve_part_1(info)
}

/*
8: 42 | 42 8
corresponds to "any number (at least 1) of 42s", while
11: 42 31 | 42 11 31
corresponds to "any numbers (at least 1) of 42s, followed by the same number of 31s".
We can easily encode these directly with the Parser::Many variant, which I've added to the enum
*/
#[allow(dead_code)]
fn solve_part_2(info: PuzzleInfo) -> usize {
    let PuzzleInfo {
        mut parsers,
        strings,
    } = info;

    // this doesn't work, because the many1 parser combinator is "greedy" - it takes as
    // many copies of parser 42 as immediately work, and won't backtrack due to a later failure
    // need to figure out how to get this backtracking!
    // (perhaps many_till would work - but that needs to know what the "next" parser is to try to apply,
    // and it's not clear how I could do this with the code as it stands)
    parsers.insert(8, Parser::Many(Box::new(Parser::Single(42))));
    // parsers.insert(11, Parser::Many(Box::new(Parser::Pair(42, 31)))); this isn't right, but won't fix until
    // I figure out why we get a *lower* answer than part 1!
    let zero_parser = parsers.get(&0).unwrap();
    strings
        .iter()
        .filter(|&s| zero_parser.can_parse(&parsers, s.to_string()))
        .count()
}

#[allow(dead_code)]
pub fn part_2() -> usize {
    let info = read_file();
    solve_part_2(info)
}
