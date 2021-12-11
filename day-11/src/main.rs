use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

#[derive(Debug, Clone)]
struct Input {
    data: Vec<Vec<i32>>,
    height: usize,
    width: usize,
}

impl Input {
    fn new(data: Vec<Vec<i32>>, height: usize, width: usize) -> Self {
        Input {
            data,
            height,
            width,
        }
    }

    // increments by 1, and counts flashes
    fn increment(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                self.data[row][col] += 1;
            }
        }
    }

    fn flash(&mut self) -> i32 {
        let mut total = 0;
        let mut set = HashSet::new();
        for i in 0..self.height {
            for j in 0..self.width {
                total += self.current_flash(i, j, &mut set);
            }
        }

        return total;
    }

    fn current_flash(&mut self, i: usize, j: usize, set: &mut HashSet<(usize, usize)>) -> i32 {
        if self.data[i][j] > 9 {
            // flash
            self.data[i][j] = 0;
            set.insert((i, j));

            // increment adjacent and calculate
            let mut flashes = 1;
            for (row, col) in self.adjacents(i as i32, j as i32) {
                if !set.contains(&(row, col)) {
                    self.data[row][col] += 1;
                    flashes += self.current_flash(row, col, set);
                }
            }

            return flashes;
        }

        return 0;
    }

    fn adjacents(&self, i: i32, j: i32) -> Vec<(usize, usize)> {
        let mut adjacents = Vec::new();
        for (x_diff, y_diff) in vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let current_pos = (i + x_diff, j + y_diff);
            if current_pos.0 < 0
                || current_pos.1 < 0
                || current_pos.0 >= self.height as i32
                || current_pos.1 >= self.width as i32
            {
                continue;
            }

            adjacents.push((current_pos.0 as usize, current_pos.1 as usize));
        }

        return adjacents;
    }
}

fn main() {
    let file = File::open("./day-11/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input = Input::new(Vec::new(), 0, 0);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut current_ine = Vec::new();
        for c in line.trim().split_terminator("").skip(1) {
            let i = c.trim().parse::<i32>().unwrap();
            current_ine.push(i);
        }

        input.width = current_ine.len();
        input.data.push(current_ine);
    }

    input.height = input.data.len();

    let mut new_input = input.clone();

    println!("{}", part_1(&mut new_input));
    println!("{}", part_2(&mut input));
}

fn part_1(input: &mut Input) -> i32 {
    // 100 steps
    let mut flashes = 0;
    for _ in 0..100 {
        input.increment();
        // Count flashes
        flashes += input.flash();
    }

    return flashes;
}

fn part_2(input: &mut Input) -> i32 {
    for i in 0..1000 {
        input.increment();

        if input.flash() == (input.height * input.width) as i32 {
            return i + 1;
        }
    }

    return 0;
}
