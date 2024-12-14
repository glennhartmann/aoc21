use std::{
    fmt,
    io::{BufWriter, Write},
};

use aoclib_rs::{fwd_rev_incl_range, prep_io, printwriteln};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(s: &str) -> Point {
        let mut ssp = s.split(',');
        Point {
            x: ssp.next().unwrap().parse::<usize>().unwrap(),
            y: ssp.next().unwrap().parse::<usize>().unwrap(),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn new(s: &str) -> Segment {
        let mut ssp = s.split(" -> ");
        Segment {
            start: Point::new(ssp.next().unwrap()),
            end: Point::new(ssp.next().unwrap()),
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn mark_on_grid(&self, grid: &mut Grid) {
        if self.is_horizontal() || self.is_vertical() {
            for y in fwd_rev_incl_range(self.start.y, self.end.y) {
                for x in fwd_rev_incl_range(self.start.x, self.end.x) {
                    grid[y][x] += 1;
                }
            }
            return;
        }

        // diagonal
        let v = fwd_rev_incl_range(self.start.y, self.end.y).collect::<Vec<_>>();
        let h = fwd_rev_incl_range(self.start.x, self.end.x).collect::<Vec<_>>();

        if h.len() != v.len() {
            panic!(
                "diagonal segment is not 45 degrees: h={}, v={}",
                h.len(),
                v.len()
            );
        }

        for i in 0..h.len() {
            grid[v[i]][h[i]] += 1;
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

type Grid = Vec<Vec<usize>>;

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 5).unwrap();
    let segments: Vec<_> = contents.iter().map(|line| Segment::new(line)).collect();

    part1(&mut writer, &segments);
    part2(&mut writer, &segments);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, segments: &Vec<Segment>) {
    let mut grid = vec![vec![0; WIDTH]; HEIGHT];

    for seg in segments {
        if seg.is_horizontal() || seg.is_vertical() {
            seg.mark_on_grid(&mut grid);
        }
    }

    printwriteln!(writer, "part 1: {}", count_overlaps(&grid)).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, segments: &Vec<Segment>) {
    let mut grid = vec![vec![0; WIDTH]; HEIGHT];

    for seg in segments {
        seg.mark_on_grid(&mut grid);
    }

    printwriteln!(writer, "part 2: {}", count_overlaps(&grid)).unwrap();
}

fn count_overlaps(grid: &Grid) -> usize {
    let mut total = 0;
    for row in grid {
        for cell in row {
            if *cell >= 2 {
                total += 1;
            }
        }
    }

    total
}
