use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Clone, Debug)]
enum CellState {
    Floor,
    Empty,
    Occupied,
}

struct FloorState {
    content: Vec<Vec<CellState>>,
}

impl FloorState {
    fn width(&self) -> usize {
        self.content[0].len()
    }

    fn height(&self) -> usize {
        self.content.len()
    }

    fn get_cell(&self, row: usize, col: usize) -> &CellState {
        &self.content[row][col]
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Vec<&CellState> {
        let mut neighbour_positions = vec![];
        let has_left_neighbour = col > 0;
        let has_right_neighbour = col < self.width() - 1;
        let has_top_neighbour = row > 0;
        let has_bottom_neighbour = row < self.height() - 1;
        if has_top_neighbour && has_left_neighbour {
            neighbour_positions.push((row - 1, col - 1))
        }
        if has_top_neighbour {
            neighbour_positions.push((row - 1, col))
        }
        if has_top_neighbour && has_right_neighbour {
            neighbour_positions.push((row - 1, col + 1))
        }
        if has_left_neighbour {
            neighbour_positions.push((row, col - 1))
        }
        if has_right_neighbour {
            neighbour_positions.push((row, col + 1))
        }
        if has_bottom_neighbour && has_left_neighbour {
            neighbour_positions.push((row + 1, col - 1))
        }
        if has_bottom_neighbour {
            neighbour_positions.push((row + 1, col))
        }
        if has_bottom_neighbour && has_right_neighbour {
            neighbour_positions.push((row + 1, col + 1))
        }
        neighbour_positions
            .iter()
            .map(|(y, x)| self.get_cell(*y, *x))
            .collect()
    }

    fn evolve_cell(&self, row: usize, col: usize) -> CellState {
        let current = self.get_cell(row, col);
        let neighbours = self.get_neighbours(row, col);
        match current {
            CellState::Floor => CellState::Floor,
            CellState::Empty => {
                if neighbours
                    .into_iter()
                    .filter(|&c| *c == CellState::Occupied)
                    .count()
                    == 0
                {
                    CellState::Occupied
                } else {
                    CellState::Empty
                }
            }
            CellState::Occupied => {
                if neighbours
                    .into_iter()
                    .filter(|&c| *c == CellState::Occupied)
                    .count()
                    > 3
                {
                    CellState::Empty
                } else {
                    CellState::Occupied
                }
            }
        }
    }

    // internally updates the state, and returns a boolean indicating
    // whether any change was made or not
    fn evolve(&mut self) -> bool {
        let mut any_change = false;
        let old_grid = FloorState {
            content: self.content.clone(),
        };
        for row in 0..self.height() {
            for col in 0..self.width() {
                let old = old_grid.get_cell(row, col);
                let new = old_grid.evolve_cell(row, col);
                if old != &new {
                    any_change = true;
                    self.content[row][col] = new;
                }
            }
        }
        any_change
    }

    fn count_occupied(&self) -> usize {
        self.content
            .iter()
            .map(|row| {
                row.into_iter()
                    .filter(|&c| *c == CellState::Occupied)
                    .count()
            })
            .sum()
    }

    // determines if the first seat seen in a given direction (specified by horizontal and vertical
    // offsets) is occupied or not. Returns None if no seat can be seen
    fn sees_occupied(
        &self,
        row: usize,
        col: usize,
        horizontal_change: isize,
        vertical_change: isize,
    ) -> Option<bool> {
        let mut current = (
            row as isize + vertical_change,
            col as isize + horizontal_change,
        );
        let mut in_grid = current.0 >= 0
            && current.0 < self.height() as isize
            && current.1 >= 0
            && current.1 < self.width() as isize;
        while in_grid {
            let cell = self.get_cell(current.0 as usize, current.1 as usize);
            match cell {
                CellState::Floor => (),
                CellState::Empty => return Some(false),
                CellState::Occupied => return Some(true),
            };
            current.0 += vertical_change;
            current.1 += horizontal_change;
            in_grid = current.0 >= 0
                && current.0 < self.height() as isize
                && current.1 >= 0
                && current.1 < self.width() as isize;
        }
        None
    }

    fn total_occupied_seen(&self, row: usize, col: usize) -> usize {
        let results = vec![
            self.sees_occupied(row, col, -1, -1),
            self.sees_occupied(row, col, -1, 0),
            self.sees_occupied(row, col, -1, 1),
            self.sees_occupied(row, col, 0, -1),
            self.sees_occupied(row, col, 0, 1),
            self.sees_occupied(row, col, 1, -1),
            self.sees_occupied(row, col, 1, 0),
            self.sees_occupied(row, col, 1, 1),
        ];
        let mut count = 0;
        for res in results {
            if let Some(true) = res {
                count += 1;
            }
        }
        count
    }

    fn evolve_cell_new(&self, row: usize, col: usize) -> CellState {
        let current = self.get_cell(row, col);
        let occupied_seen = self.total_occupied_seen(row, col);
        match current {
            CellState::Floor => CellState::Floor,
            CellState::Empty => {
                if occupied_seen == 0 {
                    CellState::Occupied
                } else {
                    CellState::Empty
                }
            }
            CellState::Occupied => {
                if occupied_seen > 4 {
                    CellState::Empty
                } else {
                    CellState::Occupied
                }
            }
        }
    }

    fn evolve_new(&mut self) -> bool {
        let mut any_change = false;
        let old_grid = FloorState {
            content: self.content.clone(),
        };
        for row in 0..self.height() {
            for col in 0..self.width() {
                let old = old_grid.get_cell(row, col);
                let new = old_grid.evolve_cell_new(row, col);
                if old != &new {
                    any_change = true;
                    self.content[row][col] = new;
                }
            }
        }
        any_change
    }
}

fn read_file() -> FloorState {
    let mut file = File::open("./input/input11.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    parse_file(contents)
}

fn parse_file(s: String) -> FloorState {
    FloorState {
        content: s.lines().map(|l| parse_line(l.to_owned())).collect(),
    }
}

fn parse_line(s: String) -> Vec<CellState> {
    s.chars().map(parse_char).collect()
}

fn parse_char(c: char) -> CellState {
    match c {
        '.' => CellState::Floor,
        'L' => CellState::Empty,
        '#' => CellState::Occupied,
        c => panic!("unexpected character: {}", c),
    }
}

fn solve_part_1(mut start: FloorState) -> usize {
    let mut finished = false;
    while !finished {
        finished = !start.evolve();
    }
    start.count_occupied()
}

pub fn part_1() -> usize {
    let start = read_file();
    solve_part_1(start)
}

fn solve_part_2(mut start: FloorState) -> usize {
    let mut finished = false;
    while !finished {
        finished = !start.evolve_new();
    }
    start.count_occupied()
}

pub fn part_2() -> usize {
    let start = read_file();
    solve_part_2(start)
}
