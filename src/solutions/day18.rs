use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum Expression {
    Lit(usize),
    Plus(Box<Expression>, Box<Expression>),
    Times(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn eval(&self) -> usize {
        match self {
            Expression::Lit(n) => *n,
            Expression::Plus(e1, e2) => e1.eval() + e2.eval(),
            Expression::Times(e1, e2) => e1.eval() * e2.eval(),
        }
    }
}

fn read_file() -> Vec<Expression> {
    let mut file = File::open("./input/input18.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // because the natural way to write the parser is "right-biased",
    // we reverse the string before parsing. Provided we remember to switch
    // opening and closing parentheses in the relevant parser, this works fine
    // as both operations are commutative
    contents
        .lines()
        .map(|l| parse_all(&l.chars().rev().collect::<String>()[..]))
        .collect()
}

// after failing (generally with stack overflows from infinite recursion) with
// several attempts to write this parser, I eventually found the example at
// https://github.com/Geal/nom/blob/master/tests/arithmetic.rs and adapted it

fn parse_expr(input: &str) -> IResult<&str, Expression> {
    let (rest, start) = alt((parse_brackets, parse_number))(input)?;

    fold_many0(
        pair(alt((tag(" + "), tag(" * "))), parse_expr),
        start,
        |acc, (op, val)| {
            if op == " + " {
                Expression::Plus(Box::new(acc), Box::new(val))
            } else {
                Expression::Times(Box::new(acc), Box::new(val))
            }
        },
    )(rest)
}

fn parse_brackets(input: &str) -> IResult<&str, Expression> {
    // see comment above as to why the parentheses are in the "wrong" order
    delimited(char(')'), parse_expr, char('('))(input)
}

fn parse_number(input: &str) -> IResult<&str, Expression> {
    map(digit1, |n: &str| Expression::Lit(n.parse().unwrap()))(input)
}

fn parse_all(s: &str) -> Expression {
    let parse_result = parse_expr(s);
    if let Ok(("", exp)) = parse_result {
        exp
    } else {
        panic!("parsing failed - got {:?}", parse_result);
    }
}

fn solve(v: Vec<Expression>) -> usize {
    v.iter().map(|e| e.eval()).sum()
}

pub fn part_1() -> usize {
    let expressions = read_file();
    solve(expressions)
}

fn read_file_2() -> Vec<Expression> {
    let mut file = File::open("./input/input18.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // reversed, as before
    contents
        .lines()
        .map(|l| parse_all_2(&l.chars().rev().collect::<String>()[..]))
        .collect()
}

fn parse_all_2(s: &str) -> Expression {
    let parse_result = parse_expr_2(s);
    if let Ok(("", exp)) = parse_result {
        exp
    } else {
        panic!("parsing failed - got {:?}", parse_result);
    }
}

fn parse_expr_2(input: &str) -> IResult<&str, Expression> {
    let (rest, start) = parse_add(input)?;

    fold_many0(pair(tag(" * "), parse_add), start, |acc, (_, val)| {
        Expression::Times(Box::new(acc), Box::new(val))
    })(rest)
}

fn parse_add(input: &str) -> IResult<&str, Expression> {
    let mut base_parser = alt((parse_brackets_2, parse_number));
    let (rest, start) = base_parser(input)?;

    fold_many0(pair(tag(" + "), base_parser), start, |acc, (_, val)| {
        Expression::Plus(Box::new(acc), Box::new(val))
    })(rest)
}

fn parse_brackets_2(input: &str) -> IResult<&str, Expression> {
    // see comment above as to why the parentheses are in the "wrong" order
    delimited(char(')'), parse_expr_2, char('('))(input)
}

pub fn part_2() -> usize {
    let expressions = read_file_2();
    solve(expressions)
}
