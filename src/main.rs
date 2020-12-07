use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let inputs = parse_inputs();
    let target = "shiny gold";
    let mut matches = HashSet::new();

    for (key, values) in inputs.iter() {
        if contains(values, &target) {
            matches.insert(key);
        }
    }

    println!("{:?}", matches.len());
}

fn contains(values: &Option<Vec<(u32, String)>>, target: &str) -> bool {
    match values {
        Some(values) => {
            let values = values
                .iter()
                .map(|(_, r)| r.to_string())
                .collect::<Vec<String>>();

            values.iter().any(|x| x == target)
        }
        None => false,
    }
}

fn parse_inputs() -> HashMap<String, Option<Vec<(u32, String)>>> {
    let f = match File::open("inputs/day7") {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("{}", e),
    };

    let inputs = f
        .lines()
        .map(|rules| match rules {
            Ok(rules) => {
                let rules = rules
                    .replace(".", "")
                    .replace("bags", "")
                    .replace("bag", "");
                let s = rules.split("contain").collect::<Vec<&str>>();
                let luggage = s[0].trim().to_string();
                let rules = s[1].trim().to_string();

                if rules.contains("no other") {
                    return (luggage, None);
                }

                let rules = rules.split(",").collect::<Vec<&str>>();
                let rules = rules
                    .iter()
                    .map(|rule| {
                        let rule = rule.trim().to_string();
                        let position = match rule.find(' ') {
                            Some(position) => position,
                            None => panic!("Whitespace not found"),
                        };

                        let rule = rule.split_at(position);
                        let number = match rule.0.to_string().parse::<u32>() {
                            Ok(number) => number,
                            Err(e) => panic!("{}", e),
                        };

                        (number, rule.1.trim().to_string())
                    })
                    .collect::<Vec<(u32, String)>>();

                (luggage, Some(rules))
            }
            Err(e) => panic!("{}", e),
        })
        .collect::<Vec<(String, Option<Vec<(u32, String)>>)>>();

    inputs.into_iter().collect::<HashMap<String, Option<_>>>()
}
