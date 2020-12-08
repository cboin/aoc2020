use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let rules = parse_inputs();
    let target = "shiny gold";
    let mut luggages = HashSet::new();

    for key in rules.keys() {
        if contains(target, &rules, &key) {
            luggages.insert(key);
        }
    }

    println!("[part1] {}", luggages.len());

    let mut hm_count: HashMap<String, usize> = HashMap::new();
    let count = count(target, &mut hm_count, &rules);
    println!("[part2] {}", count - 1);
}

fn count(
    target: &str,
    hm: &mut HashMap<String, usize>,
    rules: &HashMap<String, Option<Vec<(usize, String)>>>,
) -> usize {
    match hm.get(target) {
        Some(v) => *v,
        None => {
            let values = match rules.get(target) {
                Some(values) => values,
                None => panic!("no values"),
            };

            let values = match values {
                Some(values) => values,
                None => return 1,
            };

            let count = values
                .iter()
                .map(|t| t.0 * count(&t.1, hm, rules))
                .sum::<usize>()
                + 1;

            hm.insert(target.to_string(), count);

            count
        }
    }
}

fn contains(
    target: &str,
    hm: &HashMap<String, Option<Vec<(usize, String)>>>,
    key: &String,
) -> bool {
    let values = match hm.get(key) {
        Some(values) => values,
        None => panic!("luggage rules not found"),
    };

    let values = match values {
        Some(values) => values,
        None => return false,
    };

    values
        .iter()
        .any(|x| x.1 == target || contains(&target, &hm, &x.1))
}

fn parse_inputs() -> HashMap<String, Option<Vec<(usize, String)>>> {
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
                        let number = match rule.0.to_string().parse::<usize>() {
                            Ok(number) => number,
                            Err(e) => panic!("{}", e),
                        };

                        (number, rule.1.trim().to_string())
                    })
                    .collect::<Vec<(usize, String)>>();

                (luggage, Some(rules))
            }
            Err(e) => panic!("{}", e),
        })
        .collect::<Vec<(String, Option<Vec<(usize, String)>>)>>();

    inputs
        .into_iter()
        .collect::<HashMap<String, Option<Vec<(usize, String)>>>>()
}
