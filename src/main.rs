use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::Read;

fn as_required_fields(p: &str) -> bool {
    p.contains("byr:")
        && p.contains("iyr:")
        && p.contains("eyr:")
        && p.contains("hgt:")
        && p.contains("hcl:")
        && p.contains("ecl:")
        && p.contains("pid:")
}

fn part_one(buffer: &Vec<String>) -> usize {
    buffer.iter().filter(|p| as_required_fields(p)).count()
}

fn validate_fields(s: &String) -> bool {
    lazy_static! {
        static ref BYR: Regex = Regex::new(r"byr:(\d{4})\b").unwrap();
        static ref IYR: Regex = Regex::new(r"iyr:(\d{4})\b").unwrap();
        static ref EYR: Regex = Regex::new(r"eyr:(\d{4})\b").unwrap();
        static ref HGT: Regex = Regex::new(r"hgt:(\d{2,3})(cm|in)\b").unwrap();
        static ref HCL: Regex = Regex::new(r"hcl:#(?:[0-9a-fA-F]{3}){1,2}\b").unwrap();
        static ref ECL: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
        static ref PID: Regex = Regex::new(r"pid:(\d{9})\b").unwrap();
    }

    if let Some(caps) = BYR.captures(&s) {
        let byr = match caps.get(1) {
            Some(byr) => match byr.as_str().parse::<u32>() {
                Ok(byr) => byr,
                Err(_) => return false,
            },
            None => return false,
        };

        if !(1920..=2002).contains(&byr) {
            return false;
        }
    }

    if let Some(caps) = IYR.captures(&s) {
        let iyr = match caps.get(1) {
            Some(iyr) => match iyr.as_str().parse::<u32>() {
                Ok(iyr) => iyr,
                Err(_) => return false,
            },
            None => return false,
        };

        if !(2010..=2020).contains(&iyr) {
            return false;
        }
    }

    if let Some(caps) = EYR.captures(&s) {
        let eyr = match caps.get(1) {
            Some(eyr) => match eyr.as_str().parse::<u32>() {
                Ok(eyr) => eyr,
                Err(_) => return false,
            },
            None => return false,
        };

        if !(2020..=2030).contains(&eyr) {
            return false;
        }
    }

    if let Some(caps) = HGT.captures(&s) {
        let hgt = match caps.get(1) {
            Some(hgt) => match hgt.as_str().parse::<u32>() {
                Ok(hgt) => hgt,
                Err(_) => return false,
            },
            None => return false,
        };

        let hgt_unit = match caps.get(2) {
            Some(hgt_unit) => hgt_unit.as_str(),
            None => "",
        };

        match hgt_unit {
            "cm" => {
                if !(150..=193).contains(&hgt) {
                    return false;
                }
            }
            "in" => {
                if !(59..=76).contains(&hgt) {
                    return false;
                }
            }
            _ => return false,
        }
    }

    HCL.is_match(s) && ECL.is_match(s) && PID.is_match(s)
}

fn part_two(buffer: &Vec<String>) -> usize {
    buffer
        .iter()
        .filter(|p| as_required_fields(p))
        .filter(|p| validate_fields(p))
        .count()
}

fn main() -> io::Result<()> {
    let mut f = File::open("inputs/day4")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let passports = buffer
        .split("\n\n")
        .map(|p| p.replace("\n", " "))
        .filter(|p| p != "")
        .collect::<Vec<String>>();

    println!("[part1] passports count: {}", part_one(&passports));
    println!("[part2] passports count: {}", part_two(&passports));

    Ok(())
}
