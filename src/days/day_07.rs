use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 7).unwrap();
    let contents: Vec<u32> = contents[0].split(",").map(|n| n.parse().unwrap()).collect();

    part1(&mut writer, &contents);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, positions: &[u32]) {
    let (&min, &max) = (
        positions.iter().min().unwrap(),
        positions.iter().max().unwrap(),
    );
    let mut min_fuel = u32::MAX;
    for i in min..=max {
        min_fuel = min_fuel.min(compute_fuel_part_1(positions, i));
    }

    printwriteln!(writer, "part 1: {}", min_fuel).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, positions: &[u32]) {
    let (&min, &max) = (
        positions.iter().min().unwrap(),
        positions.iter().max().unwrap(),
    );
    let mut min_fuel = u32::MAX;
    for i in min..=max {
        min_fuel = min_fuel.min(compute_fuel_part_2(positions, i));
    }

    printwriteln!(writer, "part 2: {}", min_fuel).unwrap();
}

fn compute_fuel_part_1(positions: &[u32], align: u32) -> u32 {
    let mut total = 0;
    for i in positions {
        total += i.abs_diff(align);
    }

    total
}

fn compute_fuel_part_2(positions: &[u32], align: u32) -> u32 {
    let mut total = 0;
    for i in positions {
        total += triangular_number(i.abs_diff(align));
    }

    total
}

/// https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
fn triangular_number(n: u32) -> u32 {
    n * (n + 1) / 2
}
