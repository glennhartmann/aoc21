use std::{
    cmp::Ordering,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    str,
};

use crate::common::printwriteln;

pub fn run() {
    let write_file = File::create("outputs/03.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/03.txt").unwrap();
    let contents = contents.split('\n');
    let contents: Vec<&[u8]> = contents.map(|s| s.as_bytes()).collect();

    part1(&mut writer, &contents);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &Vec<&[u8]>) {
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..contents[0].len() {
        let (zeros, ones) = count_zeros_and_ones_at_position(contents, i);
        gamma.push(if zeros > ones { '0' } else { '1' });
        epsilon.push(if zeros > ones { '1' } else { '0' });
    }

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();
    printwriteln!(writer, "part 1: {}", gamma * epsilon).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &[&[u8]]) {
    let mut o2 = contents.to_owned();
    let mut co2 = contents.to_owned();
    for i in 0..o2[0].len() {
        let (o2_zeros, o2_ones) = count_zeros_and_ones_at_position(&o2, i);
        let o2_most_common = match o2_zeros.cmp(&o2_ones) {
            Ordering::Less => b'1',
            Ordering::Greater => b'0',
            Ordering::Equal => b'1',
        };
        if o2.len() > 1 {
            o2.retain(|n| !n.is_empty() && n[i] == o2_most_common);
        }

        let (co2_zeros, co2_ones) = count_zeros_and_ones_at_position(&co2, i);
        let co2_most_common = match co2_zeros.cmp(&co2_ones) {
            Ordering::Less => b'0',
            Ordering::Greater => b'1',
            Ordering::Equal => b'0',
        };
        if co2.len() > 1 {
            co2.retain(|d| !d.is_empty() && d[i] == co2_most_common);
        }
    }

    let o2 = u32::from_str_radix(str::from_utf8(o2[0]).unwrap(), 2).unwrap();
    let co2 = u32::from_str_radix(str::from_utf8(co2[0]).unwrap(), 2).unwrap();
    printwriteln!(writer, "part 2: {}", o2 * co2).unwrap();
}

fn count_zeros_and_ones_at_position(contents: &Vec<&[u8]>, i: usize) -> (u32, u32) {
    let mut zeros = 0;
    let mut ones = 0;
    for n in contents {
        if n.is_empty() {
            continue;
        }

        match n[i] {
            b'0' => zeros += 1,
            b'1' => ones += 1,
            _ => panic!("invalid value: {}", n[i]),
        };
    }

    (zeros, ones)
}
