use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use aoclib_rs::printwriteln;

pub fn run() {
    let write_file = File::create("outputs/02.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/02.txt").unwrap();
    let contents = contents.split('\n');

    part1(&mut writer, contents.clone());
    part2(&mut writer, contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: std::str::Split<'_, char>) {
    let mut horizontal = 0;
    let mut depth = 0;
    for line in contents {
        if line.is_empty() {
            continue;
        }

        let mut lsp = line.split(" ");
        let dir = lsp.next().unwrap();
        let magnitude = lsp.next().unwrap().parse::<i32>().unwrap();

        match dir {
            "forward" => horizontal += magnitude,
            "down" => depth += magnitude,
            "up" => depth -= magnitude,
            &_ => panic!("unknown direction: {}", dir),
        };
    }

    printwriteln!(writer, "part 1: {}", horizontal * depth).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: std::str::Split<'_, char>) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in contents {
        if line.is_empty() {
            continue;
        }

        let mut lsp = line.split(" ");
        let cmd = lsp.next().unwrap();
        let magnitude = lsp.next().unwrap().parse::<i32>().unwrap();

        match cmd {
            "forward" => {
                horizontal += magnitude;
                depth += aim * magnitude;
            }
            "down" => aim += magnitude,
            "up" => aim -= magnitude,
            &_ => panic!("unknown command: {}", cmd),
        };
    }

    printwriteln!(writer, "part 2: {}", horizontal * depth).unwrap();
}
