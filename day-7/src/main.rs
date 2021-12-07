use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./day-7/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input: Vec<i32> = Vec::new();
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    for line in buf.split(",") {
        input.push(line.trim().parse::<i32>().unwrap());
    }

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &[i32]) -> i32 {
    let mut final_fuel = i32::MAX;

    for k in *input.iter().min().unwrap()..*input.iter().max().unwrap() {
        let mut fuel = 0;
        for val in input.iter() {
            fuel += i32::abs(*val - k);
        }

        // update final
        if fuel < final_fuel {
            final_fuel = fuel;
        }
    }

    return final_fuel;
}

fn part_2(input: &[i32]) -> i32 {
    let mut final_fuel = i32::MAX;

    for k in *input.iter().min().unwrap()..*input.iter().max().unwrap() {
        let mut fuel = 0;
        for val in input.iter() {
            // each step is +1x the previous
            let n = i32::abs(*val - k);
            fuel += n * (n + 1) / 2;
        }

        // update final
        if fuel < final_fuel {
            final_fuel = fuel;
        }
    }

    return final_fuel;
}
