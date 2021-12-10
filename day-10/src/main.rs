use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./day-10/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<String> = Vec::new();
    for line in reader.lines() {
        input.push(line.unwrap());
    }

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &[String]) -> i32 {
    let matching = HashMap::from([
        ('}', ('{', 1197)),
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('>', ('<', 25137)),
    ]);

    let mut error_score = 0;

    for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            if is_open(c) {
                stack.push(c)
            } else {
                // check if the opposite brace is at peak
                if stack[stack.len() - 1] == matching[&c].0 {
                    stack.pop();
                } else {
                    error_score += matching[&c].1;
                    // first illegal character
                    break;
                }
            }
        }
    }

    return error_score;
}

fn part_2(input: &[String]) -> i128 {
    let matching = HashMap::from([('}', '{'), (')', '('), (']', '['), ('>', '<')]);
    let rev_matching = HashMap::from([('{', 3), ('(', 1), ('[', 2), ('<', 4)]);

    let mut ans = Vec::new();
    for line in input {
        let mut stack = Vec::new();
        let mut corrupt = false;
        for c in line.chars() {
            if is_open(c) {
                stack.push(c)
            } else {
                // check if the opposite brace is at peak
                if stack[stack.len() - 1] == matching[&c] {
                    stack.pop();
                } else {
                    corrupt = true;
                    break;
                }
            }
        }

        if !corrupt {
            stack.reverse();
            let mut score = 0;
            if !stack.is_empty() {
                for i in stack {
                    score = 5 * score + rev_matching[&i]
                }
            }

            ans.push(score);
        }
    }

    ans.sort();
    return ans[ans.len() / 2];
}

fn is_open(input: char) -> bool {
    return input == '{' || input == '(' || input == '[' || input == '<';
}
