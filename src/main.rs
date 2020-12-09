use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = match File::open("inputs/day9") {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("{}", e),
    };

    let inputs = f
        .lines()
        .map(|line| {
            let line = match line {
                Ok(line) => line,
                Err(e) => panic!("{}", e),
            };

            match line.parse::<u64>() {
                Ok(line) => line,
                Err(e) => panic!("{}", e),
            }
        })
        .collect::<Vec<u64>>();

    let preamble_size = 26;
    let windows = inputs
        .windows(preamble_size)
        .map(|w| {
            let last = w.last().expect("invalid last");
            let w = &w[..(preamble_size - 1)];

            (w, *last)
        })
        .collect::<Vec<(&[u64], u64)>>();

    let target = part_one(&windows);
    println!(
        "[part2] {:?}",
        part_two(&inputs, target).expect("part2 error")
    );
}

fn part_one(windows: &Vec<(&[u64], u64)>) -> u64 {
    for window in windows {
        if !is_valid(&window) {
            println!("[part1] {:?}", window.1);
            return window.1;
        }
    }

    0
}

fn part_two(inputs: &Vec<u64>, target: u64) -> Option<u64> {
    let t = inputs.to_vec();
    let mut curr_sum = *t.first().expect("invalid first");
    let mut start = 0;
    let mut i = 1;

    loop {
        if i >= inputs.len() {
            break;
        }

        while curr_sum > target && start < i - 1 {
            curr_sum = curr_sum - t[start];
            start += 1;
        }

        if curr_sum == target {
            let mut result = t[start..(i - 1)].to_vec();
            result.sort();
            let a = *result.first().expect("invalid first");
            let b = *result.last().expect("invalid last");

            return Some(a + b);
        }

        if i < inputs.len() {
            curr_sum = curr_sum + t[i];
        }

        i += 1;
    }

    None
}

fn is_valid(window: &(&[u64], u64)) -> bool {
    window
        .0
        .iter()
        .any(|&x| window.0.contains(&(window.1.checked_sub(x).unwrap_or(0))))
}
