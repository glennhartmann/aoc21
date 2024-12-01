use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use crate::common::printwriteln;

pub fn run() {
    let write_file = File::create("outputs/01.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/01.txt").unwrap();
    let contents = contents.split('\n');
    let contents: Vec<&str> = contents.collect();

    part1(&mut writer, &contents);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &Vec<&str>) {
    let mut first = true;
    let mut prev = -1;
    let mut total = 0;
    for currs in contents {
        if currs.is_empty() {
            continue;
        }

        let curr = currs.parse::<i32>().unwrap();
        if !first && curr > prev {
            total += 1;
        }

        prev = curr;
        first = false;
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &[&str]) {
    let mut first = true;
    let mut total = 0;
    let mut windows = vec![0; contents.len()];
    for i in 2..windows.len() {
        if contents[i].is_empty() {
            continue;
        }

        windows[i] = contents[i].parse::<i32>().unwrap()
            + contents[i - 1].parse::<i32>().unwrap()
            + contents[i - 2].parse::<i32>().unwrap();

        if !first && windows[i] > windows[i - 1] {
            total += 1;
        }

        first = false;
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}
