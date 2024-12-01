use std::{
    fmt,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use crate::common::printwriteln;

use {once_cell::sync::Lazy, regex::Regex};

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[derive(Copy, Clone)]
struct BingoCell {
    val: u8,
    marked: bool,
}

impl fmt::Display for BingoCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}", self.val)
    }
}

#[derive(Clone)]
struct BingoBoard {
    grid: [[BingoCell; WIDTH]; HEIGHT],
    row_marked: [u8; HEIGHT],
    col_marked: [u8; WIDTH],
    last_marked: u8,
}

impl BingoBoard {
    fn new(spec: &[&str]) -> BingoBoard {
        if spec.len() != WIDTH {
            panic!("spec.len() = {}", spec.len());
        }

        let mut board = BingoBoard {
            grid: [[BingoCell {
                val: 0,
                marked: false,
            }; WIDTH]; HEIGHT],
            row_marked: [0; HEIGHT],
            col_marked: [0; WIDTH],
            last_marked: 0,
        };

        for (y, item) in spec.iter().enumerate() {
            static RE: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"(\d+) +(\d+) +(\d+) +(\d+) +(\d+)").unwrap());
            let caps = RE.captures(item).unwrap();
            for x in 0..board.grid[y].len() {
                board.grid[y][x].val = caps[x + 1].parse::<u8>().unwrap();
            }
        }

        board
    }

    fn has_won(&self) -> bool {
        for m in self.row_marked {
            if usize::from(m) == WIDTH {
                return true;
            }
        }

        for m in self.col_marked {
            if usize::from(m) == HEIGHT {
                return true;
            }
        }

        false
    }

    fn mark(&mut self, n: u8) {
        self.last_marked = n;

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x].val == n {
                    self.grid[y][x].marked = true;
                    self.row_marked[y] += 1;
                    self.col_marked[x] += 1;
                }
            }
        }
    }

    fn score(&self) -> u32 {
        let mut unmarked_sum: u32 = 0;
        for r in self.grid {
            for c in r {
                if !c.marked {
                    unmarked_sum += <u8 as Into<u32>>::into(c.val);
                }
            }
        }

        unmarked_sum * <u8 as Into<u32>>::into(self.last_marked)
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..HEIGHT {
            writeln!(
                f,
                "{} {} {} {} {}",
                self.grid[y][0], self.grid[y][1], self.grid[y][2], self.grid[y][3], self.grid[y][4]
            )?;
        }
        Ok(())
    }
}

pub fn run() {
    let write_file = File::create("outputs/04.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/04.txt").unwrap();
    let contents = contents.split('\n');
    let contents: Vec<&str> = contents.collect();

    let numbers: Vec<u8> = contents[0]
        .split(',')
        .map(|v| v.parse::<u8>().unwrap())
        .collect();

    let mut contents = &contents[2..];
    let mut bingo_boards = Vec::new();
    loop {
        if contents.len() < HEIGHT {
            break;
        }

        bingo_boards.push(BingoBoard::new(&contents[..HEIGHT]));
        println!("{}", bingo_boards[bingo_boards.len() - 1]);
        contents = &contents[HEIGHT + 1..];
    }

    part1(&mut writer, bingo_boards.clone(), &numbers);
    part2(&mut writer, bingo_boards, &numbers);
}

fn part1<W: Write>(
    writer: &mut BufWriter<W>,
    mut bingo_boards: Vec<BingoBoard>,
    numbers: &Vec<u8>,
) {
    'outer: for n in numbers {
        for (b, board) in bingo_boards.iter_mut().enumerate() {
            board.mark(*n);
            if board.has_won() {
                println!("board {} won!", b);
                printwriteln!(writer, "part 1: {}", board.score()).unwrap();
                break 'outer;
            }
        }
    }
}

fn part2<W: Write>(
    writer: &mut BufWriter<W>,
    mut bingo_boards: Vec<BingoBoard>,
    numbers: &Vec<u8>,
) {
    let mut last_board = 0;
    for n in numbers {
        let mut num_won = 0;
        for (b, board) in bingo_boards.iter_mut().enumerate() {
            board.mark(*n);
            if board.has_won() {
                num_won += 1;
            } else {
                last_board = b;
            }
        }

        if num_won == bingo_boards.len() - 1 {
            println!("board {} is the last one", last_board);
        }

        if num_won == bingo_boards.len() {
            printwriteln!(writer, "part 2: {}", bingo_boards[last_board].score()).unwrap();
            break;
        }
    }
}
