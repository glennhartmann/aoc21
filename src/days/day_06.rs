use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 6).unwrap();
    let contents: Vec<u8> = contents[0].split(",").map(|n| n.parse().unwrap()).collect();

    let mut counts = [0u64; 9];
    for n in contents {
        counts[usize::from(n)] += 1;
    }

    part1(&mut writer, counts);
    part2(&mut writer, &mut counts);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, mut counts: [u64; 9]) {
    for _ in 0..80 {
        simulate_day(&mut counts);
    }

    printwriteln!(writer, "part 1: {}", counts.iter().sum::<u64>()).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, counts: &mut [u64; 9]) {
    for _ in 0..256 {
        simulate_day(counts);
    }

    printwriteln!(writer, "part 2: {}", counts.iter().sum::<u64>()).unwrap();
}

fn simulate_day(counts: &mut [u64; 9]) {
    let reproducers = counts[0];
    for i in 0..(counts.len() - 1) {
        counts[i] = counts[i + 1];
    }

    counts[8] = reproducers;
    counts[6] += reproducers;
}
