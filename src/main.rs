use counter::Counter;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

fn part_one(answers_groups: &Vec<Vec<String>>) -> usize {
    let mut acc = 0;
    for answer_group in answers_groups {
        let joined_answer_group = answer_group.join("");
        let mut answer_group_dedup = joined_answer_group.clone().chars().collect::<Vec<char>>();
        answer_group_dedup.sort();
        answer_group_dedup.dedup();

        acc = acc + answer_group_dedup.len();
    }

    acc
}

fn part_two(answers_groups: &Vec<Vec<String>>) -> usize {
    let mut positive_answers = 0;
    for answers_group in answers_groups {
        let group_size = answers_group.len();
        let mut positive_group_answers: HashMap<char, usize> = HashMap::new();

        for answers in answers_group {
            for answer in answers.chars() {
                let counter = positive_group_answers.entry(answer).or_insert(0);
                *counter += 1;
            }
        }

        for positive_answer in positive_group_answers.values() {
            if group_size == *positive_answer {
                positive_answers += 1;
            }
        }
    }

    positive_answers
}

fn main() -> io::Result<()> {
    let mut f = File::open("inputs/day6")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let answers = buffer
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut answers_groups: Vec<Vec<String>> = Vec::new();
    for answer in answers.iter() {
        answers_groups.push(answer.split("\n").map(|s| s.to_string()).collect());
    }

    println!("[part1] {}", part_one(&answers_groups));
    println!("[part2] {}", part_two(&answers_groups));
    Ok(())
}
