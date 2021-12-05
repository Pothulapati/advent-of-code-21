use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

fn main() {
    let file = File::open("./day-5/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<Line> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let points = line.splitn(2, "->").collect::<Vec<&str>>();
        let start = points[0].split(",").collect::<Vec<&str>>();
        let end = points[1].split(",").collect::<Vec<&str>>();
        input.push(Line {
            start: (
                start[0].trim().parse::<i32>().unwrap(),
                start[1].trim().parse::<i32>().unwrap(),
            ),
            end: (
                end[0].trim().parse::<i32>().unwrap(),
                end[1].trim().parse::<i32>().unwrap(),
            ),
        });
    }

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(inputs: &Vec<Line>) -> i32 {
    let mut points_marked = HashMap::new();
    for line in inputs {
        // horizontal line
        if line.start.0 == line.end.0 {
            // start from lowest y
            let mut y = line.start.1.min(line.end.1);
            // mark all points in the line
            while y <= line.start.1.max(line.end.1) {
                *points_marked.entry((line.start.0, y)).or_insert(0) += 1;
                y += 1;
            }
        }

        // vertical line
        if line.start.1 == line.end.1 {
            // start from lowest x
            let mut x = line.start.0.min(line.end.0);
            // mark all points in the line
            while x <= line.start.0.max(line.end.0) {
                *points_marked.entry((x, line.start.1)).or_insert(0) += 1;
                x += 1;
            }
        }
    }

    // count all points >=2
    return points_marked.values().filter(|&v| *v >= 2).count() as i32;
}

fn part_2(inputs: &Vec<Line>) -> i32 {
    let mut points_marked = HashMap::new();
    for line in inputs {
        // horizontal line
        if line.start.0 == line.end.0 {
            // start from lowest y
            let mut y = line.start.1.min(line.end.1);
            // mark all points in the line
            while y <= line.start.1.max(line.end.1) {
                *points_marked.entry((line.start.0, y)).or_insert(0) += 1;
                y += 1;
            }
        } else if line.start.1 == line.end.1 {
            // start from lowest x
            let mut x = line.start.0.min(line.end.0);
            // mark all points in the line
            while x <= line.start.0.max(line.end.0) {
                *points_marked.entry((x, line.start.1)).or_insert(0) += 1;
                x += 1;
            }
        } else {
            let xdiff = if line.start.0 < line.end.0 {
                1
            } else if line.start.0 > line.end.0 {
                -1
            } else {
                0
            };
            let ydiff = if line.start.1 < line.end.1 {
                1
            } else if line.start.1 > line.end.1 {
                -1
            } else {
                0
            };

            let mut x = line.start.0;
            let mut y = line.start.1;
            loop {
                if x == line.end.0 && y == line.end.1 {
                    *points_marked.entry((x, y)).or_insert(0) += 1;
                    break;
                }
                *points_marked.entry((x, y)).or_insert(0) += 1;

                x += xdiff;
                y += ydiff;
            }
        }
    }

    // count all points >=2
    return points_marked.values().filter(|&v| *v >= 2).count() as i32;
}
