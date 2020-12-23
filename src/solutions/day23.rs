struct CupCycle {
    cups: Vec<usize>,
    current_index: usize,
}

impl CupCycle {
    fn new(start: &[usize]) -> CupCycle {
        CupCycle {
            cups: start.to_vec(),
            current_index: 0,
        }
    }

    fn read_from_position(&self, i: usize) -> usize {
        let number_of_cups = self.cups.len();
        self.cups[i % number_of_cups]
    }

    fn single_cycle(&mut self) -> () {
        let number_of_cups = self.cups.len();

        let next_three = [
            self.read_from_position(self.current_index + 1),
            self.read_from_position(self.current_index + 2),
            self.read_from_position(self.current_index + 3),
        ];
        let current = self.read_from_position(self.current_index);

        let mut target = current;
        let mut found_target = false;
        while !found_target {
            target -= 1;
            if target == 0 {
                target = 9;
            }
            found_target = !next_three.contains(&target);
        }

        for i in 0..3 {
            let position_to_remove = self.cups.iter().position(|&n| n == next_three[i]).unwrap();
            self.cups.remove(position_to_remove);
        }

        let insert_position = self.cups.iter().position(|&n| n == target).unwrap();
        self.cups.insert(insert_position + 1, next_three[0]);
        self.cups.insert(insert_position + 2, next_three[1]);
        self.cups.insert(insert_position + 3, next_three[2]);

        self.current_index =
            (self.cups.iter().position(|&n| n == current).unwrap() + 1) % number_of_cups;
    }

    fn cycle(&mut self, n: usize) -> () {
        for _ in 0..n {
            self.single_cycle();
        }
    }

    fn output(&self) -> String {
        let mut result = String::new();
        let index_of_one = self.cups.iter().position(|&n| n == 1).unwrap();
        let number_of_cups = self.cups.len();
        for i in index_of_one + 1..number_of_cups {
            result.push_str(&self.cups[i].to_string());
        }
        for i in 0..index_of_one {
            result.push_str(&self.cups[i].to_string());
        }
        result
    }
}

fn solve_part_1(cups: &mut CupCycle) -> String {
    cups.cycle(100);
    cups.output()
}

pub fn part_1() -> String {
    let mut cups = CupCycle::new(&[2, 1, 9, 7, 4, 8, 3, 6, 5]);
    solve_part_1(&mut cups)
}

#[allow(dead_code)]
fn solve_part_2(cups: &mut CupCycle) -> usize {
    cups.cycle(10000000);
    let one_index = cups.cups.iter().position(|&n| n == 1).unwrap();
    cups.read_from_position(one_index + 1) * cups.read_from_position(one_index + 2)
}

// takes several seconds per thousand cycles, obviously impractical to run for 10 million
// (which would take many hours). Need a different approach!
#[allow(dead_code)]
pub fn part_2() -> usize {
    // note: an additinal change would be needed, when checking for the "destination cup",
    // now need to count down from a million, rather than just 9!
    let mut starting_cups: Vec<usize> = (1..100001).collect();
    starting_cups[0] = 2;
    starting_cups[1] = 1;
    starting_cups[2] = 9;
    starting_cups[3] = 7;
    starting_cups[4] = 4;
    starting_cups[5] = 8;
    starting_cups[6] = 3;
    starting_cups[7] = 6;
    starting_cups[8] = 5;
    let mut cups = CupCycle::new(&starting_cups);
    solve_part_2(&mut cups)
}
