use std::fs::File;
use std::io::{BufRead, BufReader};

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn main() {
    let file = File::open("./day-2/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<Instruction> = Vec::new();
    for file_line in reader.lines() {
        let file_line = file_line.unwrap();
        let line = file_line.split(" ").collect::<Vec<&str>>();

        if line[0] == "forward" {
            input.push(Instruction::Forward(line[1].parse::<i32>().unwrap()));
        } else if line[0] == "up" {
            input.push(Instruction::Up(line[1].parse::<i32>().unwrap()));
        } else if line[0] == "down" {
            input.push(Instruction::Down(line[1].parse::<i32>().unwrap()));
        }
    }

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &[Instruction]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for instruction in input {
        match instruction {
            Instruction::Forward(x) => {
                horizontal += x;
            }
            Instruction::Down(x) => {
                depth += x;
            }
            Instruction::Up(x) => {
                depth -= x;
            }
        }
    }

    return horizontal * depth;
}

fn part_2(input: &[Instruction]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in input {
        match instruction {
            Instruction::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Instruction::Down(x) => {
                aim += x;
            }
            Instruction::Up(x) => {
                aim -= x;
            }
        }
    }

    return horizontal * depth;
}
