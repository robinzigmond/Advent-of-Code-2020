use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug)]
enum Transformation {
    None,
    RotateOnce,
    RotateTwice,
    RotateThrice,
    Flip,
    FlipAndRotateOnce,
    FlipAndRotateTwice,
    FlipAndRotateThrice,
}

impl Transformation {
    fn all() -> [Transformation; 8] {
        [
            Transformation::None,
            Transformation::RotateOnce,
            Transformation::RotateTwice,
            Transformation::RotateThrice,
            Transformation::Flip,
            Transformation::FlipAndRotateOnce,
            Transformation::FlipAndRotateTwice,
            Transformation::FlipAndRotateThrice,
        ]
    }
}

#[derive(Copy, Clone)]
struct Tile {
    content: [[char; 10]; 10],
}

impl Tile {
    fn top_edge(&self) -> [char; 10] {
        self.content[0]
    }

    fn bottom_edge(&self) -> [char; 10] {
        self.content[9]
    }

    fn left_edge(&self) -> [char; 10] {
        self.content
            .iter()
            .map(|row| row[0])
            .collect::<Vec<char>>()
            .try_into()
            .unwrap()
    }

    fn right_edge(&self) -> [char; 10] {
        self.content
            .iter()
            .map(|row| row[9])
            .collect::<Vec<char>>()
            .try_into()
            .unwrap()
    }

    fn flip(&self) -> Tile {
        Tile {
            content: self
                .content
                .iter()
                .map(|&row| row)
                .rev()
                .collect::<Vec<[char; 10]>>()
                .try_into()
                .unwrap(),
        }
    }

    fn rotate_right(&self) -> Tile {
        Tile {
            content: (0..10)
                .map(|n| {
                    self.content
                        .iter()
                        .map(|row| row[n])
                        .collect::<Vec<char>>()
                        .try_into()
                        .unwrap()
                })
                .rev()
                .collect::<Vec<[char; 10]>>()
                .try_into()
                .unwrap(),
        }
    }

    fn apply_transformation(&self, t: &Transformation) -> Tile {
        match t {
            Transformation::None => Tile {
                content: self.content,
            },
            Transformation::RotateOnce => self.rotate_right(),
            Transformation::RotateTwice => self.rotate_right().rotate_right(),
            Transformation::RotateThrice => self.rotate_right().rotate_right().rotate_right(),
            Transformation::Flip => self.flip(),
            Transformation::FlipAndRotateOnce => self.flip().rotate_right(),
            Transformation::FlipAndRotateTwice => self.flip().rotate_right().rotate_right(),
            Transformation::FlipAndRotateThrice => {
                self.flip().rotate_right().rotate_right().rotate_right()
            }
        }
    }

    fn top_matches(&self, t: Tile) -> Vec<Transformation> {
        let top = self.top_edge();

        Transformation::all()
            .iter()
            .filter(|&trans| t.apply_transformation(trans).bottom_edge() == top)
            .map(|&trans| trans)
            .collect()
    }

    fn bottom_matches(&self, t: Tile) -> Vec<Transformation> {
        let bottom = self.bottom_edge();

        Transformation::all()
            .iter()
            .filter(|&trans| t.apply_transformation(trans).top_edge() == bottom)
            .map(|&trans| trans)
            .collect()
    }

    fn left_matches(&self, t: Tile) -> Vec<Transformation> {
        let left = self.left_edge();

        Transformation::all()
            .iter()
            .filter(|&trans| t.apply_transformation(trans).right_edge() == left)
            .map(|&trans| trans)
            .collect()
    }

    fn right_matches(&self, t: Tile) -> Vec<Transformation> {
        let right = self.right_edge();

        Transformation::all()
            .iter()
            .filter(|&trans| t.apply_transformation(trans).left_edge() == right)
            .map(|&trans| trans)
            .collect()
    }
}

struct AllTiles {
    tiles: HashMap<usize, Tile>,
}

impl AllTiles {
    fn match_info(&self, i: usize) -> [Vec<(usize, Transformation)>; 4] {
        let tile_to_test = self.tiles.get(&i).unwrap();

        let top_matches = self
            .tiles
            .iter()
            .filter(|(&idx, _)| idx != i)
            .flat_map(|(&idx, &tile)| {
                tile_to_test
                    .top_matches(tile)
                    .into_iter()
                    .map(move |trans| (idx, trans))
            })
            .collect();

        let bottom_matches = self
            .tiles
            .iter()
            .filter(|(&idx, _)| idx != i)
            .flat_map(|(&idx, &tile)| {
                tile_to_test
                    .bottom_matches(tile)
                    .into_iter()
                    .map(move |trans| (idx, trans))
            })
            .collect();

        let left_matches = self
            .tiles
            .iter()
            .filter(|(&idx, _)| idx != i)
            .flat_map(|(&idx, &tile)| {
                tile_to_test
                    .left_matches(tile)
                    .into_iter()
                    .map(move |trans| (idx, trans))
            })
            .collect();

        let right_matches = self
            .tiles
            .iter()
            .filter(|(&idx, _)| idx != i)
            .flat_map(|(&idx, &tile)| {
                tile_to_test
                    .right_matches(tile)
                    .into_iter()
                    .map(move |trans| (idx, trans))
            })
            .collect();

        [top_matches, bottom_matches, left_matches, right_matches]
    }
}

fn read_file() -> AllTiles {
    let mut file = File::open("./input/input20.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parts: Vec<String> = contents
        .lines()
        .group_by(|s| s.is_empty())
        .into_iter()
        .filter(|(k, _g)| !k)
        .map(|(_k, g)| g.collect::<Vec<&str>>().join("\n"))
        .collect();

    let mut tiles = HashMap::new();

    for part in parts {
        let (id, tile) = parse_tile(part);
        tiles.insert(id, tile);
    }

    AllTiles { tiles }
}

fn parse_tile(s: String) -> (usize, Tile) {
    let lines: Vec<&str> = s.lines().collect();
    let id = lines[0][5..9].parse().unwrap();
    let tile = Tile {
        content: lines[1..]
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>().try_into().unwrap())
            .collect::<Vec<[char; 10]>>()
            .try_into()
            .unwrap(),
    };

    (id, tile)
}

fn solve_part_1(tiles: AllTiles) -> usize {
    // we can try to get a printout of all possible matches:
    // this works and gives a very pleasing result - there are exactly 48 cases where
    // no match has been found for a side, 12 for each side. This instantly tells us what the
    // corner tiles are - they are the ones with exactly 2 empty sets in their match info!

    // (In fact, there is only one possibility for each side, which makes reconstructing the
    // full pattern simple. Will do, and thereby attempt to solve part 2, when I have more time!)
    let mut corners = vec![];
    for &idx in tiles.tiles.keys() {
        let info = tiles.match_info(idx);
        // println!("match information about tile #{}: {:?}", idx, info);
        let is_corner = info.iter().filter(|v| v.len() == 0).count() == 2;
        if is_corner {
            corners.push(idx);
        }
    }
    if corners.len() == 4 {
        corners.iter().product()
    } else {
        panic!("not 4 corners!");
    }
}

pub fn part_1() -> usize {
    let tiles = read_file();
    solve_part_1(tiles)
}
