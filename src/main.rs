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
    answers_groups.iter().fold(0, |acc, group| {
        let mut positive_answers_group = 0;
        let mut positive_answers: HashMap<char, usize> = HashMap::new();
        let group_size = group.len();

        for ppl_answer in group {
            for answer in ppl_answer.chars() {
                let ans = positive_answers.entry(answer).or_insert(0);
                *ans += 1;
            }
        }

        for answer in positive_answers.values() {
            if *answer == group_size {
                positive_answers_group += 1
            }
        }

        acc + positive_answers_group
    })
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
