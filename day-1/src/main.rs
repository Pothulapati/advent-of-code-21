use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./day-1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<i32> = Vec::new();
    for line in reader.lines() {
        input.push(line.unwrap().parse::<i32>().unwrap());
    }

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &[i32]) -> i32 {
    let mut current = 0;
    // Setting -1, as first element is counted to be big
    let mut count = -1;
    for next in input {
        if *next > current {
            count += 1;
        }
        current = *next;
    }
    return count;
}

fn part_2(input: &[i32]) -> i32 {
    let mut prev_window = 0;
    let mut count = -1;

    for i in 0..input.len() - 2 {
        let current_window = input[i] + input[i + 1] + input[i + 2];

        if current_window > prev_window {
            count += 1;
        }
        prev_window = current_window;
    }

    // TODO: Try to use rust iterators, maps, etc next time as much as possible.
    return count;
}
