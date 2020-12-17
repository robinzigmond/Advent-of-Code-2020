use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Cube {
    Active,
    Inactive,
}

impl Cube {
    fn is_active(&self) -> bool {
        match self {
            Cube::Active => true,
            Cube::Inactive => false,
        }
    }
}

struct Grid {
    locations: HashMap<(isize, isize, isize), Cube>,
}

impl Grid {
    fn count_active(&self) -> usize {
        self.locations
            .values()
            .filter(|cube| cube.is_active())
            .count()
    }

    fn get_cube(&self, x: isize, y: isize, z: isize) -> &Cube {
        let lookup = self.locations.get(&(x, y, z));
        lookup.unwrap_or(&Cube::Inactive)
    }

    fn active_neighbour_count(&self, x: isize, y: isize, z: isize) -> usize {
        let mut neighbours = 0;
        for x0 in x - 1..x + 2 {
            for y0 in y - 1..y + 2 {
                for z0 in z - 1..z + 2 {
                    if (x0, y0, z0) != (x, y, z) {
                        if let Cube::Active = self.get_cube(x0, y0, z0) {
                            neighbours += 1;
                        }
                    }
                }
            }
        }
        neighbours
    }

    fn get_active_ranges(&self) -> [(isize, isize); 3] {
        let mut x_min = isize::MAX;
        let mut x_max = isize::MIN;
        let mut y_min = isize::MAX;
        let mut y_max = isize::MIN;
        let mut z_min = isize::MAX;
        let mut z_max = isize::MIN;
        self.locations.iter().for_each(|((x, y, z), cube)| {
            if let Cube::Active = cube {
                if x - 1 < x_min {
                    x_min = x - 1;
                }
                if x + 1 > x_max {
                    x_max = x + 1;
                }
                if y - 1 < y_min {
                    y_min = y - 1;
                }
                if y + 1 > y_max {
                    y_max = y + 1;
                }
                if z - 1 < z_min {
                    z_min = z - 1;
                }
                if z + 1 > z_max {
                    z_max = z + 1;
                }
            }
        });
        [(x_min, x_max), (y_min, y_max), (z_min, z_max)]
    }

    fn evolve(&mut self, generations: usize) -> () {
        for _ in 0..generations {
            let mut changes = HashMap::new();
            let [(x_min, x_max), (y_min, y_max), (z_min, z_max)] = self.get_active_ranges();
            for x in x_min..x_max + 1 {
                for y in y_min..y_max + 1 {
                    for z in z_min..z_max + 1 {
                        let active_neighbour_count = self.active_neighbour_count(x, y, z);
                        match self.get_cube(x, y, z) {
                            Cube::Active => {
                                if active_neighbour_count != 2 && active_neighbour_count != 3 {
                                    changes.insert((x, y, z), Cube::Inactive);
                                }
                            }
                            Cube::Inactive => {
                                if active_neighbour_count == 3 {
                                    changes.insert((x, y, z), Cube::Active);
                                }
                            }
                        }
                    }
                }
            }
            for (&k, &v) in changes.iter() {
                self.locations.insert(k, v);
            }
        }
    }
}

fn read_file() -> Grid {
    let mut file = File::open("./input/input17.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    parse_file(contents)
}

fn parse_file(s: String) -> Grid {
    let mut grid = Grid {
        locations: HashMap::new(),
    };
    s.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(col, c)| {
            grid.locations
                .insert((row as isize, col as isize, 0), parse_char(c));
        })
    });
    grid
}

fn parse_char(c: char) -> Cube {
    match c {
        '#' => Cube::Active,
        '.' => Cube::Inactive,
        _ => panic!("unexpected character: {}", c),
    }
}

fn solve_part_1(mut grid: Grid) -> usize {
    grid.evolve(6);
    grid.count_active()
}

pub fn part_1() -> usize {
    let grid = read_file();
    solve_part_1(grid)
}

// unfortunately, while Part 2 is an easy change from part 1 (barring potential performance
// issues, which I don't expect - in reality it took longer than I hoped but nothing dramatic),
// it's not easy to adapt the existing code (above), so I'm going to have to just copy-paste it
// all and make the necessary (small) changes
struct Grid4D {
    locations: HashMap<(isize, isize, isize, isize), Cube>,
}

impl Grid4D {
    fn count_active(&self) -> usize {
        self.locations
            .values()
            .filter(|cube| cube.is_active())
            .count()
    }

    fn get_cube(&self, x: isize, y: isize, z: isize, w: isize) -> &Cube {
        let lookup = self.locations.get(&(x, y, z, w));
        lookup.unwrap_or(&Cube::Inactive)
    }

    fn active_neighbour_count(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let mut neighbours = 0;
        for x0 in x - 1..x + 2 {
            for y0 in y - 1..y + 2 {
                for z0 in z - 1..z + 2 {
                    for w0 in w - 1..w + 2 {
                        if (x0, y0, z0, w0) != (x, y, z, w) {
                            if let Cube::Active = self.get_cube(x0, y0, z0, w0) {
                                neighbours += 1;
                            }
                        }
                    }
                }
            }
        }
        neighbours
    }

    fn get_active_ranges(&self) -> [(isize, isize); 4] {
        let mut x_min = isize::MAX;
        let mut x_max = isize::MIN;
        let mut y_min = isize::MAX;
        let mut y_max = isize::MIN;
        let mut z_min = isize::MAX;
        let mut z_max = isize::MIN;
        let mut w_min = isize::MAX;
        let mut w_max = isize::MIN;
        self.locations.iter().for_each(|((x, y, z, w), cube)| {
            if let Cube::Active = cube {
                if x - 1 < x_min {
                    x_min = x - 1;
                }
                if x + 1 > x_max {
                    x_max = x + 1;
                }
                if y - 1 < y_min {
                    y_min = y - 1;
                }
                if y + 1 > y_max {
                    y_max = y + 1;
                }
                if z - 1 < z_min {
                    z_min = z - 1;
                }
                if z + 1 > z_max {
                    z_max = z + 1;
                }
                if w - 1 < w_min {
                    w_min = w - 1;
                }
                if w + 1 > w_max {
                    w_max = w + 1;
                }
            }
        });
        [
            (x_min, x_max),
            (y_min, y_max),
            (z_min, z_max),
            (w_min, w_max),
        ]
    }

    fn evolve(&mut self, generations: usize) -> () {
        for _ in 0..generations {
            let mut changes = HashMap::new();
            let [(x_min, x_max), (y_min, y_max), (z_min, z_max), (w_min, w_max)] =
                self.get_active_ranges();
            for x in x_min..x_max + 1 {
                for y in y_min..y_max + 1 {
                    for z in z_min..z_max + 1 {
                        for w in w_min..w_max + 1 {
                            let active_neighbour_count = self.active_neighbour_count(x, y, z, w);
                            match self.get_cube(x, y, z, w) {
                                Cube::Active => {
                                    if active_neighbour_count != 2 && active_neighbour_count != 3 {
                                        changes.insert((x, y, z, w), Cube::Inactive);
                                    }
                                }
                                Cube::Inactive => {
                                    if active_neighbour_count == 3 {
                                        changes.insert((x, y, z, w), Cube::Active);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for (&k, &v) in changes.iter() {
                self.locations.insert(k, v);
            }
        }
    }
}

fn read_file_4d() -> Grid4D {
    let mut file = File::open("./input/input17.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    parse_file_4d(contents)
}

fn parse_file_4d(s: String) -> Grid4D {
    let mut grid = Grid4D {
        locations: HashMap::new(),
    };
    s.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(col, c)| {
            grid.locations
                .insert((row as isize, col as isize, 0, 0), parse_char(c));
        })
    });
    grid
}

fn solve_part_2(mut grid: Grid4D) -> usize {
    grid.evolve(6);
    grid.count_active()
}

pub fn part_2() -> usize {
    let grid = read_file_4d();
    solve_part_2(grid)
}
