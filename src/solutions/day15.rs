use std::collections::HashMap;

/*
The key observation, for efficiently solving the second part of the puzzle (part 1 is
easily done with any sensible algorithm), is that a vector/array of all numbers - which
is the most obvious data structure to use - is unnecessary to keep. All we ever need to know
is the most recent index of each number (if any). This suggests using a hashmap, whose keys
are the numbers, and values are the most recent index. Actually, the easiest way is for the
value to be *pairs* of the *two* most recent indices. The algorithm then becomes very
easy (provided one remembers to not update the hashmap until after the difference has been
calculated!), as well as efficient even up to 30 million iterations (and beyond)!

(Actually this solution still ran for a few minutes (although down to just 15-20 seconds when compiled
in release mode) - must have some inefficiencies that can be
improved. But still reasonable, and vastly better than using vectors!)
*/

#[derive(Debug)]
struct IndexPair(Option<usize>, Option<usize>);

impl IndexPair {
    fn new() -> IndexPair {
        IndexPair(None, None)
    }

    fn has_both(&self) -> bool {
        if let IndexPair(Some(_), Some(_)) = self {
            true
        } else {
            false
        }
    }

    fn new_index(&self, index: usize) -> IndexPair {
        let IndexPair(_, new) = self;
        IndexPair(*new, Some(index))
    }

    fn index_difference(&self) -> usize {
        let IndexPair(old, new) = self;
        new.unwrap() - old.unwrap()
    }
}

#[derive(Debug)]
struct Numbers {
    used: HashMap<usize, IndexPair>,
    index: usize,
    last: usize,
}

impl Numbers {
    fn new() -> Numbers {
        Numbers {
            used: HashMap::new(),
            index: 0,
            last: 0,
        }
    }

    fn insert(&mut self, num: usize) -> () {
        self.last = num;
        let old_pair = self.used.get(&num);
        let new = &IndexPair::new();
        let old_pair = match old_pair {
            None => new,
            Some(p) => p,
        };
        let new_pair = old_pair.new_index(self.index);
        self.used.insert(num, new_pair);
        self.index += 1;
    }

    fn from_vec(nums: Vec<usize>) -> Numbers {
        let mut new = Numbers::new();
        for num in nums {
            new.insert(num);
        }
        new
    }

    fn next(&mut self) -> () {
        let new = &IndexPair::new();
        let last_index_pair = self.used.get(&self.last).unwrap_or(new);
        let new_val = if last_index_pair.has_both() {
            last_index_pair.index_difference()
        } else {
            0
        };
        let new_index_pair = self.used.get(&new_val).unwrap_or(new).new_index(self.index);
        self.used.insert(new_val, new_index_pair);
        self.last = new_val;
        self.index += 1;
    }

    fn get_nth(&mut self, n: usize) -> usize {
        for _ in (self.index + 1)..(n + 1) {
            self.next();
        }
        self.last
    }
}

fn solve_part_1(nums: &mut Numbers) -> usize {
    nums.get_nth(2020)
}

pub fn part_1() -> usize {
    let mut nums = Numbers::from_vec(vec![12, 1, 16, 3, 11, 0]);
    solve_part_1(&mut nums)
}

fn solve_part_2(nums: &mut Numbers) -> usize {
    nums.get_nth(30000000)
}

pub fn part_2() -> usize {
    let mut nums = Numbers::from_vec(vec![12, 1, 16, 3, 11, 0]);
    solve_part_2(&mut nums)
}
