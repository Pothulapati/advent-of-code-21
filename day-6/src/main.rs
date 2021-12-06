use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./day-6/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input: Vec<i128> = Vec::new();
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    for line in buf.split(",") {
        input.push(line.trim().parse::<i128>().unwrap());
    }

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &[i128]) -> i128 {
    let mut current_state = input.to_vec();

    for _ in 0..80 {
        // Decrease the counter by 1
        for i in 0..current_state.len() {
            current_state[i] -= 1;
            if current_state[i] < 0 {
                current_state[i] = 6;
                // add a new element
                current_state.push(8);
            }
        }
    }

    return current_state.len() as i128;
}

fn part_2(input: &[i128]) -> i128 {
    // part-1 is way too slow, instead
    // maintain a vector of states
    let mut states = vec![0; 9];

    for i in input.iter() {
        states[*i as usize] += 1;
    }

    for _ in 0..256 {
        // Decrease the counter by 1
        let x = states[0];
        states[0] = states[1];
        states[1] = states[2];
        states[2] = states[3];
        states[3] = states[4];
        states[4] = states[5];
        states[5] = states[6];
        // all `0` state gets moved to 6
        states[6] = states[7] + x;
        states[7] = states[8];
        // create a x new lanternfishes with 8
        states[8] = x;
    }

    return states.iter().sum::<i128>();
}
