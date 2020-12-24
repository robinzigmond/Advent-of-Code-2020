use nom::{branch::alt, bytes::complete::tag, multi::fold_many1, IResult};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, PartialEq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

// represents co-ordinates on a hex grid. The "x-axis" is a horizontal line,
// while the "y-axis" runs NW to SE
#[derive(Clone, Copy)]
struct HexPosition {
    x: isize,
    y: isize,
}

struct Floor {
    flipped: HashMap<(isize, isize), bool>,
}

impl Floor {
    fn new() -> Floor {
        Floor {
            flipped: HashMap::new(),
        }
    }

    fn toggle_position(&mut self, pos: HexPosition) -> () {
        let already_flipped = self.flipped.get(&(pos.x, pos.y)).unwrap_or(&false);
        let toggled = !already_flipped;
        self.flipped.insert((pos.x, pos.y), toggled);
    }

    fn from_vector(v: Vec<HexPosition>) -> Floor {
        let mut floor = Floor::new();
        for hex in v {
            floor.toggle_position(hex);
        }
        floor
    }

    fn count_flipped(&self) -> usize {
        self.flipped.values().filter(|&&flipped| flipped).count()
    }

    fn flipped_neighbours(&self, pos: HexPosition) -> usize {
        let HexPosition { x, y } = pos;
        let neighbours = [
            (x - 1, y),
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x, y + 1),
            (x - 1, y + 1),
        ];
        neighbours
            .iter()
            .filter(|&(xpos, ypos)| *self.flipped.get(&(*xpos, *ypos)).unwrap_or(&false))
            .count()
    }

    fn evolve_once(&mut self) -> () {
        let mut to_flip = vec![];

        // the following tiles should be flipped
        // 1) any currently-flipped tiles which have any number of flipped neighbours other than 1 or 2
        // 2) any tile that is currently not flipped, which has exactly 2 flipped neighbours
        // 3) as in 2), but remember to take into account any tiles which are not yet in the hashmap!
        // So first we build up a vector of all keys, making sure to include all neighbouring hexes of
        // all those we have
        let all_keys: Vec<(isize, isize)> =
            self.flipped.keys().map(|pair| pair.to_owned()).collect();
        let mut new_keys = all_keys.to_vec();
        for &(x, y) in all_keys.iter() {
            let neighbours = [
                (x - 1, y),
                (x, y - 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x, y + 1),
                (x - 1, y + 1),
            ];
            for neighbour in &neighbours {
                if !new_keys.contains(&neighbour) {
                    new_keys.push(*neighbour);
                }
            }
        }

        for key in new_keys {
            let is_flipped = *self.flipped.get(&key).unwrap_or(&false);
            let flipped_neighbours = self.flipped_neighbours(HexPosition { x: key.0, y: key.1 });
            if (is_flipped && flipped_neighbours != 1 && flipped_neighbours != 2)
                || (!is_flipped && flipped_neighbours == 2)
            {
                to_flip.push(key);
            }
        }

        to_flip
            .iter()
            .for_each(|&pos| self.toggle_position(HexPosition { x: pos.0, y: pos.1 }));
    }

    fn evolve(&mut self, times: usize) -> () {
        for _ in 0..times {
            self.evolve_once();
        }
    }
}

fn movement(pos: HexPosition, dir: Direction) -> HexPosition {
    match dir {
        Direction::East => HexPosition {
            x: pos.x + 1,
            y: pos.y,
        },
        Direction::SouthEast => HexPosition {
            x: pos.x,
            y: pos.y + 1,
        },
        Direction::SouthWest => HexPosition {
            x: pos.x - 1,
            y: pos.y + 1,
        },
        Direction::West => HexPosition {
            x: pos.x - 1,
            y: pos.y,
        },
        Direction::NorthWest => HexPosition {
            x: pos.x,
            y: pos.y - 1,
        },
        Direction::NorthEast => HexPosition {
            x: pos.x + 1,
            y: pos.y - 1,
        },
    }
}

fn read_file() -> Vec<HexPosition> {
    let mut file = File::open("./input/input24.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.lines().map(parse_line).collect()
}

fn parse_line(s: &str) -> HexPosition {
    let mut parser = fold_many1(
        alt((
            tag("e"),
            tag("se"),
            tag("sw"),
            tag("w"),
            tag("nw"),
            tag("ne"),
        )),
        HexPosition { x: 0, y: 0 },
        |current, newdir| {
            if newdir == "e" {
                movement(current, Direction::East)
            } else if newdir == "se" {
                movement(current, Direction::SouthEast)
            } else if newdir == "sw" {
                movement(current, Direction::SouthWest)
            } else if newdir == "w" {
                movement(current, Direction::West)
            } else if newdir == "nw" {
                movement(current, Direction::NorthWest)
            } else if newdir == "ne" {
                movement(current, Direction::NorthEast)
            } else {
                panic!("this can't happen!")
            }
        },
    );
    let result: IResult<&str, HexPosition> = parser(s);
    result.unwrap().1
}

fn solve_part_1(hexes_flipped: Vec<HexPosition>) -> usize {
    let floor = Floor::from_vector(hexes_flipped);
    floor.count_flipped()
}

pub fn part_1() -> usize {
    let hexes_flipped = read_file();
    solve_part_1(hexes_flipped)
}

// ran for around 10 minutes via cargo run - but after compilation in release mode,
// went down to around 10 seconds!
fn solve_part_2(hexes_flipped: Vec<HexPosition>) -> usize {
    let mut floor = Floor::from_vector(hexes_flipped);
    floor.evolve(100);
    floor.count_flipped()
}

pub fn part_2() -> usize {
    let hexes_flipped = read_file();
    solve_part_2(hexes_flipped)
}
