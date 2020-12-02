use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

struct Entry {
    policy: Policy,
    password: String,
}

impl Entry {
    fn new(attributes: Vec<&str>) -> Self {
        let indexes: Vec<&str> = attributes[0].split("-").collect();
        let min = match indexes[0].parse::<usize>() {
            Ok(min) => min,
            Err(e) => panic!("{}", e),
        };
        let max = match indexes[1].parse::<usize>() {
            Ok(max) => max,
            Err(e) => panic!("{}", e),
        };

        let letter = match attributes[1].chars().nth(0) {
            Some(letter) => letter,
            None => panic!("Letter not found"),
        };

        let policy = Policy { min, max, letter };

        Entry {
            policy,
            password: String::from(attributes[2]),
        }
    }
}

fn load_entries<R: BufRead>(buf: &mut R) -> Vec<Entry> {
    let mut entries = Vec::<Entry>::new();
    for line in buf.lines() {
        match line {
            Ok(line) => {
                let split: Vec<&str> = line.split(" ").collect();
                entries.push(Entry::new(split));
            }
            Err(e) => panic!("{}", e),
        }
    }

    entries
}

fn main() {
    let mut f = match File::open("inputs/day2") {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("{}", e),
    };

    let entries = load_entries(&mut f);
    part_one(&entries);
    part_two(&entries);
}

fn part_one(entries: &Vec<Entry>) {
    let mut count_valid_entries = 0;
    for entry in entries.iter() {
        let mut count = 0;
        for c in entry.password.chars() {
            if c == entry.policy.letter {
                count += 1;
            }

            if count > entry.policy.max {
                count = 0;
                break;
            }
        }

        if count < entry.policy.min {
            continue;
        } else {
            count_valid_entries += 1;
        }
    }
    println!("[part1] valid passwords count: {}", count_valid_entries);
}

fn part_two(entries: &Vec<Entry>) {
    let mut count_valid_entries = 0;
    for entry in entries.iter() {
        let p1 = match entry.password.chars().nth(entry.policy.min - 1) {
            Some(p1) => p1,
            None => continue,
        };
        let p2 = match entry.password.chars().nth(entry.policy.max - 1) {
            Some(p2) => p2,
            None => continue,
        };

        if (p1 == entry.policy.letter || p2 == entry.policy.letter) && p1 != p2 {
            count_valid_entries += 1;
        }
    }
    println!("[part2] valid passwords count: {}", count_valid_entries);
}
