use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 1).unwrap();
    let contents = contents.iter().map(|c| c.parse().unwrap()).collect();

    part1(&mut writer, &contents);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &Vec<i32>) {
    let mut first = true;
    let mut prev = -1;
    let mut total = 0;
    for curr in contents {
        if !first && *curr > prev {
            total += 1;
        }

        prev = *curr;
        first = false;
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &[i32]) {
    let mut first = true;
    let mut total = 0;
    let mut windows = vec![0; contents.len()];
    for i in 2..windows.len() {
        windows[i] = contents[i] + contents[i - 1] + contents[i - 2];

        if !first && windows[i] > windows[i - 1] {
            total += 1;
        }

        first = false;
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}
