use std::fs::File;
use std::io::prelude::*;

enum Move {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn move_forward(&self, amount: usize) -> Move {
        match self {
            Direction::North => Move::North(amount),
            Direction::South => Move::South(amount),
            Direction::East => Move::East(amount),
            Direction::West => Move::West(amount),
        }
    }

    fn rotate_right(&self, turns: usize) -> Self {
        let one_turn = |dir| match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        let mut current = *self;
        for _ in 0..turns {
            current = one_turn(current);
        }
        current
    }

    fn rotate_left(&self, turns: usize) -> Self {
        let one_turn = |dir| match dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        let mut current = *self;
        for _ in 0..turns {
            current = one_turn(current);
        }
        current
    }
}

struct ShipState {
    x_pos: isize,
    y_pos: isize,
    direction: Direction,
    waypoint_offset_x: isize,
    waypoint_offset_y: isize,
}

impl ShipState {
    fn new() -> Self {
        ShipState {
            x_pos: 0,
            y_pos: 0,
            direction: Direction::East,
            waypoint_offset_x: 10,
            waypoint_offset_y: 1,
        }
    }

    fn movement(&mut self, m: Move) -> () {
        match m {
            Move::North(amount) => self.y_pos += amount as isize,
            Move::South(amount) => self.y_pos -= amount as isize,
            Move::East(amount) => self.x_pos += amount as isize,
            Move::West(amount) => self.x_pos -= amount as isize,
            Move::Left(angle) => self.direction = self.direction.rotate_left(angle / 90),
            Move::Right(angle) => self.direction = self.direction.rotate_right(angle / 90),
            Move::Forward(amount) => self.movement(self.direction.move_forward(amount)),
        }
    }

    fn bulk_move(&mut self, v: Vec<Move>) -> () {
        for m in v {
            self.movement(m);
        }
    }

    fn manhattan(&self) -> isize {
        self.x_pos.abs() + self.y_pos.abs()
    }

    fn waypoint_rotate_left(&mut self, turns: usize) -> () {
        for _ in 0..turns {
            let old_xpos = self.waypoint_offset_x;
            self.waypoint_offset_x = -self.waypoint_offset_y;
            self.waypoint_offset_y = old_xpos;
        }
    }

    fn waypoint_rotate_right(&mut self, turns: usize) {
        for _ in 0..turns {
            let old_xpos = self.waypoint_offset_x;
            self.waypoint_offset_x = self.waypoint_offset_y;
            self.waypoint_offset_y = -old_xpos;
        }
    }

    fn movement_with_waypoint(&mut self, m: Move) -> () {
        match m {
            Move::North(amount) => self.waypoint_offset_y += amount as isize,
            Move::South(amount) => self.waypoint_offset_y -= amount as isize,
            Move::East(amount) => self.waypoint_offset_x += amount as isize,
            Move::West(amount) => self.waypoint_offset_x -= amount as isize,
            Move::Left(angle) => self.waypoint_rotate_left(angle / 90),
            Move::Right(angle) => self.waypoint_rotate_right(angle / 90),
            Move::Forward(amount) => {
                self.x_pos += amount as isize * self.waypoint_offset_x;
                self.y_pos += amount as isize * self.waypoint_offset_y;
            }
        }
    }

    fn bulk_move_with_waypoint(&mut self, v: Vec<Move>) -> () {
        for m in v {
            self.movement_with_waypoint(m);
        }
    }
}

fn read_file() -> Vec<Move> {
    let mut file = File::open("./input/input12.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(parse_line).collect()
}

fn parse_line(l: &str) -> Move {
    let dir = l.as_bytes()[0] as char;
    let amount: usize = l.as_bytes()[1..]
        .iter()
        .map(|&n| n as char)
        .collect::<String>()
        .parse()
        .unwrap();
    match dir {
        'N' => Move::North(amount),
        'S' => Move::South(amount),
        'E' => Move::East(amount),
        'W' => Move::West(amount),
        'L' => Move::Left(amount),
        'R' => Move::Right(amount),
        'F' => Move::Forward(amount),
        c => panic!("unexpected move character: {}", c),
    }
}

fn solve_part_1(moves: Vec<Move>) -> isize {
    let mut ship = ShipState::new();
    ship.bulk_move(moves);
    ship.manhattan()
}

pub fn part_1() -> isize {
    let moves = read_file();
    solve_part_1(moves)
}

fn solve_part_2(moves: Vec<Move>) -> isize {
    let mut ship = ShipState::new();
    ship.bulk_move_with_waypoint(moves);
    ship.manhattan()
}

pub fn part_2() -> isize {
    let moves = read_file();
    solve_part_2(moves)
}
