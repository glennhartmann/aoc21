use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 2).unwrap();

    part1(&mut writer, contents.clone());
    part2(&mut writer, contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: Vec<&str>) {
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

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: Vec<&str>) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in contents {
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
