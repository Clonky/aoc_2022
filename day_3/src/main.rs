use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Backpack {
    _hash_set: HashSet<char>,
    nonunique_char: Option<char>,
}

impl Backpack {
    fn new(input: &str) -> Self {
        let (hash_set, nonunique_char) = Self::get_hashset(input);
        Self {
            _hash_set: hash_set,
            nonunique_char,
        }
    }

    fn get_hashset(input: &str) -> (HashSet<char>, Option<char>) {
        let mut char_set = HashSet::new();
        let mut first_set = HashSet::new();
        let mut second_set = HashSet::new();
        let (first, second) = input.split_at(input.len() / 2);
        for ichar in input.chars() {
            char_set.insert(ichar);
        }
        for ichar in first.chars() {
            first_set.insert(ichar);
        }
        for ichar in second.chars() {
            second_set.insert(ichar);
        }
        let nonunique_char = first_set.intersection(&second_set).next().copied();
        (char_set, nonunique_char)
    }
}

impl From<&str> for Backpack {
    fn from(input: &str) -> Self {
        Backpack::new(input)
    }
}

fn setup_score_hashmap() -> HashMap<char, i32> {
    let lower_case = "abcdefghijklmnopqrstuvwxyz";
    let upper_case = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let range = 1..(26 * 2 + 1);
    let mut char_map: HashMap<char, i32> = HashMap::new();
    for (ichar, idx) in format!("{}{}", lower_case, upper_case).chars().zip(range) {
        char_map.insert(ichar, idx);
    }
    char_map
}

fn main() {
    const SRC: &str = "input.txt";
    let char_map: HashMap<char, i32> = setup_score_hashmap();
    let content = fs::read_to_string(SRC).expect("Error reading file.");
    let backpacks: Vec<Backpack> = content.lines().map(Backpack::from).collect();
    let result_first = backpacks.iter().fold(0, |mut _accum, ipack| {
        if let Some(nonunique_char) = ipack.nonunique_char {
            _accum += *char_map
                .get(&nonunique_char)
                .expect("Couldn't find char in map.");
        }
        _accum
    });
    println!("The sum of nonunique items is: {}", result_first);
    let badges: Vec<char> = backpacks
        .chunks(3)
        .map(|igroup| {
            igroup
                .iter()
                .map(|item| item._hash_set.clone())
                .reduce(|acc, ibp| acc.intersection(&ibp).copied().collect::<HashSet<char>>())
                .map(|iset| iset.into_iter().next().unwrap())
                .unwrap()
        })
        .collect();
    let badges_score = badges.iter().fold(0, |mut accum, ichar| {
        accum += char_map.get(ichar).unwrap();
        accum
    });
    println!("Badge sum: {}", badges_score);
}
