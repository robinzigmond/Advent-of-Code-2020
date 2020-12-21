use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct AllergenInfo {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

struct InformationSummary {
    // associates each allergen with the set of ingredients which could potentially contain it
    summary: HashMap<String, HashSet<String>>,
}

impl InformationSummary {
    fn new() -> InformationSummary {
        InformationSummary {
            summary: HashMap::new(),
        }
    }

    fn add_allergen_information(&mut self, info: &AllergenInfo) -> () {
        for allergen in info.allergens.iter() {
            let possible_ingredients = self.summary.get(allergen);

            let new_ingredients = match possible_ingredients {
                Some(ingredient_set) => ingredient_set
                    .intersection(&info.ingredients)
                    .map(|s| s.to_owned())
                    .collect(),
                None => info.ingredients.iter().map(|s| s.to_owned()).collect(),
            };

            self.summary.insert(allergen.to_owned(), new_ingredients);
        }
    }

    fn can_contain_allergen(&self, ingredient: String) -> bool {
        let res = self
            .summary
            .iter()
            .any(|(_, set)| set.contains(&ingredient));
        res
    }
}

fn read_file() -> Vec<AllergenInfo> {
    let mut file = File::open("./input/input21.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.lines().map(parse_line).collect()
}

fn parse_line(s: &str) -> AllergenInfo {
    let parts: Vec<&str> = s.split(" (contains ").collect();
    let ingredients = parts[0].split(" ").map(|s| s.to_owned()).collect();

    let allergens = parts[1][..parts[1].len() - 1]
        .split(", ")
        .map(|s| s.to_owned())
        .collect();

    AllergenInfo {
        ingredients,
        allergens,
    }
}

fn all_ingredients(v: &Vec<AllergenInfo>) -> Vec<String> {
    let mut ingredients: HashSet<String> = HashSet::new();
    v.iter().for_each(|info| {
        ingredients = ingredients
            .union(&info.ingredients.iter().map(|s| s.to_owned()).collect())
            .map(|s| s.to_owned())
            .collect();
    });
    ingredients.iter().map(|s| s.to_owned()).collect()
}

fn solve_part_1(info: Vec<AllergenInfo>) -> usize {
    let mut summary = InformationSummary::new();
    for line in info.iter() {
        summary.add_allergen_information(line);
    }

    let no_allergens: Vec<String> = all_ingredients(&info)
        .iter()
        .filter(|&ing| !summary.can_contain_allergen(ing.to_owned()))
        .map(|s| s.to_owned())
        .collect();

    info.iter()
        .map(|line| {
            line.ingredients
                .iter()
                .filter(|i| no_allergens.iter().any(|ing| &ing == i))
                .count()
        })
        .sum()
}

pub fn part_1() -> usize {
    let info = read_file();
    solve_part_1(info)
}

fn solve_part_2(info: Vec<AllergenInfo>) -> String {
    // very messy way to compute the answer in code. Would have been much quicker just to work with the
    // summary printout and do by hand!
    let mut summary = InformationSummary::new();
    for line in info.iter() {
        summary.add_allergen_information(line);
    }
    let allergen_count = summary.summary.iter().count();

    let mut results: HashMap<String, String> = HashMap::new();

    while results.len() < allergen_count {
        let mut reduced_summary = InformationSummary {
            summary: HashMap::new(),
        };
        {
            let one_left = summary.summary.iter().find(|(_, s)| s.len() == 1).unwrap();
            let (allergen, set_of_one) = one_left;
            let ingredient = set_of_one.iter().nth(0).unwrap().to_owned();
            results.insert(allergen.to_owned(), ingredient);
            let ingredient_again = results.get(allergen).unwrap();
            for allergen in summary.summary.keys() {
                let reduced_set = summary.summary.get(allergen).unwrap();
                let mut new_set = HashSet::new();
                for ing in reduced_set {
                    if ing != ingredient_again {
                        new_set.insert(ing.to_owned());
                    }
                }
                reduced_summary.summary.insert(allergen.to_owned(), new_set);
            }
        }
        summary = reduced_summary;
    }

    let mut pairs = results
        .iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect::<Vec<(String, String)>>();
    pairs.sort_unstable_by(|(k1, _), (k2, _)| k1.partial_cmp(k2).unwrap());
    pairs
        .iter()
        .map(|(_, v)| v.to_owned())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part_2() -> String {
    let info = read_file();
    solve_part_2(info)
}
