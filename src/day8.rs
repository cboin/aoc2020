use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]

enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn lex(source: &Vec<String>) -> Vec<Instruction> {
    source
        .iter()
        .map(|s| {
            let s = s.split(' ').collect::<Vec<&str>>();
            let arg = match s[1].parse::<i64>() {
                Ok(arg) => arg,
                Err(e) => panic!("{}", e),
            };

            match s[0] {
                "acc" => Instruction::Acc(arg),
                "jmp" => Instruction::Jmp(arg),
                "nop" => Instruction::Nop(arg),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Instruction>>()
}

fn run(instructions: &Vec<Instruction>) -> (bool, i64) {
    let mut acc = 0;
    let mut ip: i64 = 0;
    let mut visited_ip: Vec<i64> = Vec::new();

    loop {
        if ip as usize > instructions.len() - 1 {
            return (true, acc);
        }

        match instructions[ip as usize] {
            Instruction::Acc(v) => {
                acc += v;
                ip += 1
            }
            Instruction::Jmp(v) => ip += v,
            Instruction::Nop(_) => ip += 1,
        }

        if visited_ip.contains(&ip) {
            return (false, acc);
        }
        visited_ip.push(ip);
    }
}

fn part_one(instructions: &Vec<Instruction>) -> i64 {
    let (_, result) = run(&instructions);

    result
}

fn part_two(instructions: &Vec<Instruction>) -> i64 {
    for program in instructions.iter().enumerate().map(|(idx, inst)| {
        let mut patched = instructions.clone();
        match patched[idx] {
            Instruction::Jmp(v) => patched[idx] = Instruction::Nop(v),
            Instruction::Nop(v) => patched[idx] = Instruction::Jmp(v),
            _ => (),
        }

        patched
    }) {
        let result = run(&program);
        if result.0 {
            return result.1;
        }
    }

    -1
}

fn main() {
    let f = match File::open("inputs/day8") {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("{}", e),
    };

    let inputs = f
        .lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(e) => panic!("{}", e),
        })
        .collect::<Vec<String>>();

    let instructions = lex(&inputs);

    println!("[part1] {}", part_one(&instructions));
    println!("[part2] {}", part_two(&instructions));
}
