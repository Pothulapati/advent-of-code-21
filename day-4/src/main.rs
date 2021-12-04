use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str;

#[derive(Debug)]
struct Game(Vec<Board>);

impl Game {
    fn new(boards: Vec<Board>) -> Self {
        Game(boards)
    }

    fn mark(&mut self, input: i32) {
        for board in &mut self.0 {
            board.mark_input(input);
        }
    }

    fn check(&self) -> Option<i32> {
        for board in &self.0 {
            if let Some(x) = board.check() {
                return Some(x);
            }
        }

        return None;
    }
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<Option<i32>>>,
    width: usize,
    height: usize,
}

impl Board {
    fn from_string(input: &str) -> Self {
        let mut board = Vec::new();

        for line in input.lines() {
            let mut row: Vec<Option<i32>> = Vec::new();
            for col in line.trim().split(" ") {
                if col != "" {
                    row.push(Some(col.trim().parse::<i32>().unwrap()));
                }
            }
            board.push(row);
        }

        let width = board[0].len();
        let height = board.len();

        return Board {
            board: board,
            width: width,
            height: height,
        };
    }

    fn mark_input(&mut self, input: i32) {
        for i in 0..self.height {
            for j in 0..self.width {
                if let Some(x) = self.board[i][j] {
                    if input == x {
                        self.board[i][j] = None;
                    }
                }
            }
        }
    }

    fn check(&self) -> Option<i32> {
        // Check all rows
        for j in 0..self.width {
            let mut is_done = true;
            for i in 0..self.height {
                if self.board[i][j] != None {
                    is_done = false;
                    break;
                }
            }
            if is_done {
                return Some(self.sum());
            }
        }

        // Check all columns
        for i in 0..self.height {
            let mut is_done = true;
            for j in 0..self.width {
                if self.board[i][j] != None {
                    is_done = false;
                    break;
                }
            }
            if is_done {
                return Some(self.sum());
            }
        }

        return None;
    }

    fn sum(&self) -> i32 {
        let mut sum = 0;
        for row in &self.board {
            for col in row {
                if let Some(x) = col {
                    sum += x;
                }
            }
        }
        return sum;
    }
}

fn main() {
    let file = File::open("./day-4/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut game = Game::new(Vec::new());
    // read inputs
    let mut inputs = Vec::new();
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    for input in buf.trim().split(",") {
        inputs.push(input.parse::<i32>().unwrap());
    }

    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    for board in buf.trim().split("\n\n") {
        game.0.push(Board::from_string(board));
    }

    println!("{}", part_1(&inputs, &mut game));
    println!("{}", part_2(&inputs, &mut game));
}

fn part_1(inputs: &Vec<i32>, game: &mut Game) -> i32 {
    // Handle all inputs
    for input in inputs {
        game.mark(*input);

        // Check if any board is solved
        if let Some(x) = game.check() {
            return input * x;
        }
    }

    return 1;
}

fn part_2(inputs: &Vec<i32>, game: &mut Game) -> i32 {
    // Handle all inputs
    for input in inputs {
        game.mark(*input);

        if game.0.len() == 1 {
            // Check if any board is solved
            if let Some(x) = game.check() {
                return input * x;
            }
        }

        // Update Game Boards
        game.0 = game
            .0
            .iter()
            .filter(|board| board.check() == None)
            .cloned()
            .collect();
    }

    return 1;
}
