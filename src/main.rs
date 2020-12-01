use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = match File::open("inputs/day1") {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("{}", e),
    };

    let mut numbers: HashSet<i64> = HashSet::new();
    for line in f.lines() {
        match line {
            Ok(v) => {
                let x: i64 = match v.parse::<i64>() {
                    Ok(x) => x,
                    Err(e) => panic!("{}", e),
                };
                numbers.insert(x);
            }
            Err(e) => panic!("{}", e),
        }
    }

    part_one(&numbers);
    part_two(&numbers);
}

fn part_one(numbers: &HashSet<i64>) {
    for x in numbers.iter() {
        let y = 2020 - x;
        match numbers.get(&y) {
            Some(n) => {
                println!("[part1] {}", n);
                break;
            }
            None => continue,
        }
    }
}

fn part_two(numbers: &HashSet<i64>) {
    '_a: for x in numbers.iter() {
        '_b: for y in numbers.iter() {
            if x + y > 2020 {
                continue;
            }
            let z = 2020 - x - y;

            match numbers.get(&z) {
                Some(n) => {
                    println!("[part2] {}", x * y * n);
                    break '_a;
                }
                None => continue,
            }
        }
    }
}
