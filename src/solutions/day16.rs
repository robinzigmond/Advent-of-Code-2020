use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Rules {
    numbers: [usize; 4],
}

impl Rules {
    fn in_range(&self, num: usize) -> bool {
        let Rules {
            numbers: [a, b, c, d],
        } = self;
        (num >= *a && num <= *b) || (num >= *c && num <= *d)
    }
}

// it turns out that using a HashMap intead of a Struct would have avoided
// a bunch of repetitive boilerplate code later - but I couldn't be bothered changing.
// A Struct feels "correct" because all of these fields are included in a set of rules
struct AllRules {
    departure_location: Rules,
    departure_station: Rules,
    departure_platform: Rules,
    departure_track: Rules,
    departure_date: Rules,
    departure_time: Rules,
    arrival_location: Rules,
    arrival_station: Rules,
    arrival_platform: Rules,
    arrival_track: Rules,
    class: Rules,
    duration: Rules,
    price: Rules,
    route: Rules,
    row: Rules,
    seat: Rules,
    train: Rules,
    type_: Rules,
    wagon: Rules,
    zone: Rules,
}

impl AllRules {
    fn invalid_for_any(&self, num: usize) -> bool {
        !self.departure_location.in_range(num)
            && !self.departure_station.in_range(num)
            && !self.departure_platform.in_range(num)
            && !self.departure_track.in_range(num)
            && !self.departure_date.in_range(num)
            && !self.departure_time.in_range(num)
            && !self.arrival_location.in_range(num)
            && !self.arrival_station.in_range(num)
            && !self.arrival_platform.in_range(num)
            && !self.arrival_track.in_range(num)
            && !self.class.in_range(num)
            && !self.duration.in_range(num)
            && !self.price.in_range(num)
            && !self.route.in_range(num)
            && !self.row.in_range(num)
            && !self.seat.in_range(num)
            && !self.train.in_range(num)
            && !self.type_.in_range(num)
            && !self.wagon.in_range(num)
            && !self.zone.in_range(num)
    }
}

struct Ticket {
    values: [usize; 20],
}

impl Ticket {
    fn invalid_for_any(&self, rules: &AllRules) -> Vec<usize> {
        self.values
            .iter()
            .filter(|&n| rules.invalid_for_any(*n))
            .map(|&n| n)
            .collect()
    }
}

struct Info {
    rules: AllRules,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn read_file() -> Info {
    let mut file = File::open("./input/input16.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    parse_file(&contents)
}

fn parse_file(s: &str) -> Info {
    let v: Vec<String> = s
        .lines()
        .group_by(|s| s.is_empty())
        .into_iter()
        .filter(|(k, _g)| !k)
        .map(|(_k, g)| g.collect::<Vec<&str>>().join("\n"))
        .collect();

    let rules = parse_rules(&v[0]);
    let my_ticket = parse_ticket(&v[1].lines().collect::<Vec<&str>>()[1]);
    let nearby_tickets = v[2].lines().collect::<Vec<&str>>()[1..]
        .iter()
        .map(|&t| parse_ticket(t))
        .collect();

    Info {
        rules,
        my_ticket,
        nearby_tickets,
    }
}

fn parse_rules(s: &str) -> AllRules {
    let v: Vec<Rules> = s
        .lines()
        .map(|l| parse_range(l.split(": ").collect::<Vec<&str>>()[1]))
        .collect();
    let [
        departure_location,
        departure_station,
        departure_platform,
        departure_track,
        departure_date,
        departure_time,
        arrival_location,
        arrival_station,
        arrival_platform,
        arrival_track,
        class,
        duration,
        price,
        route,
        row,
        seat,
        train,
        type_,
        wagon,
        zone
        ]: [Rules; 20] =
        v.try_into().unwrap();
    AllRules {
        departure_location,
        departure_station,
        departure_platform,
        departure_track,
        departure_date,
        departure_time,
        arrival_location,
        arrival_station,
        arrival_platform,
        arrival_track,
        class,
        duration,
        price,
        route,
        row,
        seat,
        train,
        type_,
        wagon,
        zone,
    }
}

fn parse_range(s: &str) -> Rules {
    let nums: Vec<usize> = s
        .split(" or ")
        .flat_map(|s| s.split("-").map(|n| n.parse().unwrap()))
        .collect();
    let numbers: [usize; 4] = nums.try_into().unwrap();
    Rules { numbers }
}

fn parse_ticket(s: &str) -> Ticket {
    let values = s
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();
    Ticket { values }
}

fn solve_part_1(info: &Info) -> usize {
    info.nearby_tickets
        .iter()
        .flat_map(|t| t.invalid_for_any(&info.rules))
        .collect::<Vec<usize>>()
        .iter()
        .sum()
}

pub fn part_1() -> usize {
    let info = read_file();
    solve_part_1(&info)
}

fn solve_ticket_parts(rules: &AllRules, tickets: Vec<&Ticket>) -> [String; 20] {
    let mut possibilities = Vec::new();
    for i in 0..20 {
        let mut fields = vec![
            "departure_location",
            "departure_station",
            "departure_platform",
            "departure_track",
            "departure_date",
            "departure_time",
            "arrival_location",
            "arrival_station",
            "arrival_platform",
            "arrival_track",
            "class",
            "duration",
            "price",
            "route",
            "row",
            "seat",
            "train",
            "type_",
            "wagon",
            "zone",
        ];
        for ticket in &tickets {
            if !rules.departure_location.in_range(ticket.values[i]) {
                fields = remove(fields, "departure_location");
            }
            if !rules.departure_station.in_range(ticket.values[i]) {
                fields = remove(fields, "departure_station");
            }
            if !rules.departure_platform.in_range(ticket.values[i]) {
                fields = remove(fields, "departure_platform");
            }
            if !rules.departure_track.in_range(ticket.values[i]) {
                fields = remove(fields, "departure_track");
            }
            if !rules.departure_date.in_range(ticket.values[i]) {
                fields = remove(fields, "departure_date");
            }
            if !rules.departure_time.in_range(ticket.values[i]) {
                fields = remove(fields, "departure_time");
            }
            if !rules.arrival_location.in_range(ticket.values[i]) {
                fields = remove(fields, "arrival_location");
            }
            if !rules.arrival_station.in_range(ticket.values[i]) {
                fields = remove(fields, "arrival_station");
            }
            if !rules.arrival_platform.in_range(ticket.values[i]) {
                fields = remove(fields, "arrival_platform");
            }
            if !rules.arrival_track.in_range(ticket.values[i]) {
                fields = remove(fields, "arrival_track");
            }
            if !rules.class.in_range(ticket.values[i]) {
                fields = remove(fields, "class");
            }
            if !rules.duration.in_range(ticket.values[i]) {
                fields = remove(fields, "duration");
            }
            if !rules.price.in_range(ticket.values[i]) {
                fields = remove(fields, "price");
            }
            if !rules.route.in_range(ticket.values[i]) {
                fields = remove(fields, "route");
            }
            if !rules.row.in_range(ticket.values[i]) {
                fields = remove(fields, "row");
            }
            if !rules.seat.in_range(ticket.values[i]) {
                fields = remove(fields, "seat");
            }
            if !rules.train.in_range(ticket.values[i]) {
                fields = remove(fields, "train");
            }
            if !rules.type_.in_range(ticket.values[i]) {
                fields = remove(fields, "type_");
            }
            if !rules.wagon.in_range(ticket.values[i]) {
                fields = remove(fields, "wagon");
            }
            if !rules.zone.in_range(ticket.values[i]) {
                fields = remove(fields, "zone");
            }
        }
        possibilities.push(fields);
    }
    // we'll assume that we can solve this now by "elimination" - find an index with just one
    // choice, eliminate that from the others, and keep on going this way until we've solved for
    // all. This indeed works!
    let mut solutions_hm = HashMap::new();
    let mut solutions_vec = Vec::new();
    for _ in 0..20 {
        let one_choice_index = possibilities.iter().position(|v| v.len() == 1).unwrap();
        let result = possibilities[one_choice_index][0];
        solutions_hm.insert(one_choice_index, result);
        possibilities = possibilities
            .iter()
            .map(|v| remove(v.to_vec(), result))
            .collect();
    }
    for i in 0..20 {
        solutions_vec.push(solutions_hm.get(&i).unwrap().to_string());
    }
    solutions_vec.try_into().unwrap()
}

fn remove<'a>(v: Vec<&'a str>, s: &str) -> Vec<&'a str> {
    v.into_iter().filter(|&field| field != s).collect()
}

fn solve_part_2(info: &Info) -> usize {
    let valid_tickets = info
        .nearby_tickets
        .iter()
        .filter(|t| t.invalid_for_any(&info.rules).len() == 0)
        .collect();
    let correct_parts = solve_ticket_parts(&info.rules, valid_tickets);
    let departure_positions = correct_parts
        .iter()
        .enumerate()
        .filter(|(_, s)| s.len() > 9 && &s[0..9] == "departure")
        .map(|(i, _)| i);
    departure_positions
        .map(|i| info.my_ticket.values[i])
        .product()
}

pub fn part_2() -> usize {
    let info = read_file();
    solve_part_2(&info)
}
