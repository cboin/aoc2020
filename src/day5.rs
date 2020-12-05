use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = match File::open("inputs/day5") {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("{}", e),
    };

    let mut seats = f
        .lines()
        .filter_map(|s| s.ok())
        .map(|s| {
            s.replace("F", "0")
                .replace("L", "0")
                .replace("B", "1")
                .replace("R", "1")
        })
        .map(|s| u64::from_str_radix(&s, 2))
        .filter_map(|s| s.ok())
        .collect::<Vec<u64>>();

    let seat_max_id = match seats.iter().map(|s| s).max() {
        Some(seat_max_id) => seat_max_id,
        None => panic!("Seat not found"),
    };

    println!("[part1] seat max id: {}", seat_max_id);

    seats.sort();
    for t in seats.windows(2) {
        if t[0] != t[1] - 1 {
            println!("[part2] my seat id: {}", t[0] + 1);
            break;
        }
    }
}
