use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./day-9/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut line_input = Vec::new();
        for c in line.trim().split_terminator("").skip(1) {
            let i = c.trim().parse::<i32>().unwrap();
            line_input.push(i);
        }
        input.push(line_input);
    }

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_2(input: &Vec<Vec<i32>>) -> i128 {
    let mut total = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let mut counted: HashSet<(i32, i32)> = HashSet::new();
            total.push(find_basin(input, row as i32, col as i32, &mut counted) + 1);
        }
    }

    total.sort();
    return total[total.len() - 3..].into_iter().product();
}

// find_basin returns the size of the basin
// with (i,j) as their lowest point
fn find_basin(
    input: &Vec<Vec<i32>>,
    row: i32,
    col: i32,
    counted: &mut HashSet<(i32, i32)>,
) -> i128 {
    let mut total: i128 = 0;
    for (i, j) in combinations(row, col) {
        if counted.contains(&(i, j)) {
            continue;
        }

        if get_val(input, row, col) < get_val(input, i, j) && get_val(input, i, j) != 9 {
            total += find_basin(input, i, j, counted) + 1;
            counted.insert((i, j));
        }
    }

    return total;
}

fn combinations(i: i32, j: i32) -> Vec<(i32, i32)> {
    vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
}

fn part_1(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let row: i32 = row as i32;
            let col = col as i32;
            // Check if its around is greater than input[row][col]
            if get_val_with_max(input, row, col) < get_val_with_max(input, row, col + 1)
                && get_val_with_max(input, row, col) < get_val_with_max(input, row, col - 1)
                && get_val_with_max(input, row, col) < get_val_with_max(input, row + 1, col)
                && get_val_with_max(input, row, col) < get_val_with_max(input, row - 1, col)
            {
                sum += get_val_with_max(input, row, col) + 1;
            }
        }
    }
    return sum;
}

fn get_val_with_max(input: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    if i < 0 || j < 0 || i >= input.len() as i32 || j >= input[0].len() as i32 {
        return i32::MAX;
    } else {
        return input[i as usize][j as usize];
    }
}

fn get_val(input: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    if i < 0 || j < 0 || i >= input.len() as i32 || j >= input[0].len() as i32 {
        return -1;
    } else {
        return input[i as usize][j as usize];
    }
}
