use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn main() {
    let file = File::open("./day-3/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap().clone();
        input.push(line);
    }

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_2(input: &[String]) -> i32 {
    // o2_val
    let mut oxygen_rating = "";
    let mut current_list = input.to_vec();
    for col in 0..input[0].chars().count() {
        let mut count_0 = 0;
        let mut count_1 = 0;

        current_list.iter().for_each(|x| {
            if x.chars().nth(col).unwrap() == '1' {
                count_1 += 1;
            } else {
                count_0 += 1;
            }
        });

        // 1 is when both are equal
        let mut mc = '1';
        if count_1 > count_0 {
            mc = '1';
        } else if count_0 > count_1 {
            mc = '0';
        }
        current_list = current_list
            .iter()
            .filter(|x| x.chars().nth(col).unwrap() == mc)
            .cloned()
            .collect::<Vec<String>>();

        if current_list.len() == 1 {
            oxygen_rating = current_list[0].as_str();
            break;
        }
    }

    // co2 rating
    let mut co2_rating = "";
    let mut current_list = input.to_vec();
    for col in 0..input[0].chars().count() {
        let mut count_0 = 0;
        let mut count_1 = 0;

        current_list.iter().for_each(|x| {
            if x.chars().nth(col).unwrap() == '1' {
                count_1 += 1;
            } else {
                count_0 += 1;
            }
        });

        // 0 is when both are equal
        let mut lc = '0';
        if count_1 > count_0 {
            lc = '0';
        } else if count_0 > count_1 {
            lc = '1';
        }

        // update inputs
        current_list = current_list
            .iter()
            .filter(|x| x.chars().nth(col).unwrap() == lc)
            .cloned()
            .collect::<Vec<String>>();

        if current_list.len() == 1 {
            co2_rating = current_list[0].as_str();
            break;
        }
    }

    return bytes_to_i32(oxygen_rating) * bytes_to_i32(co2_rating);
}

fn part_1(input: &[String]) -> i32 {
    // (0, 1)
    let mut count = vec![(0, 0); input[0].chars().count()];
    // count mcb and lcb
    input.iter().for_each(|x| {
        for (i, col) in x.chars().into_iter().enumerate() {
            if col == '1' {
                count[i].1 += 1;
            } else {
                count[i].0 += 1;
            }
        }
    });

    let mcb = count
        .iter()
        .map(|(zero_count, one_count)| -> &str {
            if *zero_count > *one_count {
                "0"
            } else {
                "1"
            }
        })
        .collect::<String>()
        .to_string();

    let lcb = count
        .iter()
        .map(|(zero_count, one_count)| -> &str {
            if *zero_count > *one_count {
                "1"
            } else {
                "0"
            }
        })
        .collect::<String>()
        .to_string();

    return bytes_to_i32(&mcb) * bytes_to_i32(&lcb);
}

fn bytes_to_i32(input: &str) -> i32 {
    let mut sum = 0;
    for (i, c) in input.chars().into_iter().rev().enumerate() {
        if c == '1' {
            sum += i32::pow(2, i as u32);
        }
    }
    return sum;
}
