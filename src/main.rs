mod solutions;

use crate::solutions::day1;
use crate::solutions::day2;
use crate::solutions::day3;

fn main() {
    println!("The answer to day 1, part 1 is {}", day1::part_1());
    println!("The answer to day 1, part 2 is {}", day1::part_2());
    println!("The answer to day 2, part 1 is {}", day2::part_1());
    println!("The answer to day 2, part 2 is {}", day2::part_2());
    println!("The answer to day 3, part 1 is {}", day3::part_1());
    println!("The answer to day 3, part 2 is {}", day3::part_2());
}
