use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    fn swap(&self) -> Self {
        match self {
            Instruction::Acc(num) => Instruction::Acc(*num),
            Instruction::Jmp(num) => Instruction::Nop(*num),
            Instruction::Nop(num) => Instruction::Jmp(*num),
        }
    }
}

struct ProgramState {
    program: Vec<Instruction>,
    position: usize,
    acc: isize,
}

impl ProgramState {
    fn new(program: Vec<Instruction>) -> ProgramState {
        ProgramState {
            program,
            position: 0,
            acc: 0,
        }
    }

    fn run_instruction(&mut self) -> () {
        let instruction = &self.program[self.position];
        match instruction {
            Instruction::Acc(num) => {
                self.acc += num;
                self.position += 1;
            }
            Instruction::Jmp(num) => self.position = (self.position as isize + num) as usize,
            Instruction::Nop(_) => {
                self.position += 1;
            }
        }
    }

    fn run_program(&mut self) -> Result<isize, isize> {
        let mut used: Vec<usize> = Vec::new();
        while !used.contains(&self.position) {
            if self.position == self.program.len() {
                return Ok(self.acc);
            }
            used.push(self.position);
            self.run_instruction();
        }
        Err(self.acc)
    }
}

fn read_file() -> Vec<Instruction> {
    let mut file = File::open("./input/input8.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(parse_line).collect()
}

fn parse_line(s: &str) -> Instruction {
    let instr_name = &s[0..3];
    let number: &isize = &s[4..].parse().unwrap();
    match instr_name {
        "acc" => Instruction::Acc(*number),
        "jmp" => Instruction::Jmp(*number),
        "nop" => Instruction::Nop(*number),
        s => panic!("unexpected instruction: {}", s),
    }
}

fn solve_part_1(program: Vec<Instruction>) -> isize {
    let mut state = ProgramState::new(program);
    match state.run_program() {
        Ok(_) => panic!("program terminated without repeating!"),
        Err(acc) => acc,
    }
}

pub fn part_1() -> isize {
    let program = read_file();
    solve_part_1(program)
}

fn solve_part_2(program: Vec<Instruction>) -> isize {
    for i in 0..program.len() {
        let mut new_program = program.clone();
        match &program[i] {
            Instruction::Acc(_) => continue,
            instr => new_program[i] = instr.swap(),
        }
        let mut state = ProgramState::new(new_program.to_vec());
        match state.run_program() {
            Ok(acc) => return acc,
            Err(_) => (),
        }
    }
    panic!("no swap worked!");
}

pub fn part_2() -> isize {
    let program = read_file();
    solve_part_2(program)
}
