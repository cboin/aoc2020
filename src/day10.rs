use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = match File::open("inputs/day10") {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("{}", e),
    };

    let mut output_joltages = f
        .lines()
        .map(|line| {
            line.expect("invalid line")
                .parse::<u64>()
                .expect("invalid number")
        })
        .collect::<Vec<u64>>();

    output_joltages.sort();

    let differences = part_one(&output_joltages);
    println!("[part1] {}", differences.0 * differences.1);

    let ways = part_two(&output_joltages);
    println!("[part2] {}", ways);
}

fn count_ways(
    joltages: &[u64],
    start: u64,
    target: u64,
    memory: &mut HashMap<(usize, u64), u64>,
) -> u64 {
    let mut ways = 0;
    let key = (joltages.len(), start);

    match memory.get(&key) {
        Some(v) => return *v,
        None => (),
    }

    if target - start <= 3 {
        ways += 1;
    }

    if joltages.is_empty() {
        return ways;
    }

    if joltages[0] - start <= 3 {
        ways += count_ways(&joltages[1..], joltages[0], target, memory);
    }

    ways += count_ways(&joltages[1..], start, target, memory);
    memory.insert(key, ways);

    ways
}

fn part_two(joltages: &Vec<u64>) -> u64 {
    let target = 3 + joltages.iter().max().expect("invalid max");
    let mut memory = HashMap::new();

    count_ways(&joltages[..], 0, target, &mut memory)
}

fn part_one(joltages: &Vec<u64>) -> (u64, u64) {
    let mut differences = joltages
        .windows(2)
        .map(|x| x[1] - x[0])
        .collect::<Vec<u64>>();

    // Build-in adapter always 3
    differences.push(3);

    (
        differences.iter().filter(|&x| *x == 1).count() as u64,
        differences.iter().filter(|&x| *x == 3).count() as u64,
    )
}
