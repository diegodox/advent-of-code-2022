use std::io::BufRead;

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn input() -> std::io::BufReader<std::fs::File> {
    let mut p = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    p.push("src");
    p.push(module_path!().split("::").last().unwrap());
    p.push("input.txt");
    std::io::BufReader::new(std::fs::File::open(p).unwrap())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(self, other: Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn is_cross(self, other: Self) -> bool {
        self.end < other.start || self.start > other.end
    }
}

impl std::str::FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut w = s.split('-');
        Ok(Self {
            start: w.next().unwrap().parse().unwrap(),
            end: w.next().unwrap().parse().unwrap(),
        })
    }
}

pub fn part1() -> usize {
    input()
        .lines()
        .map(|lines| lines.unwrap())
        .map(|l| {
            let mut words = l.split(',');
            (
                words.next().unwrap().parse::<Range>().unwrap(),
                words.next().unwrap().parse::<Range>().unwrap(),
            )
        })
        .filter(|(r1, r2)| r1.contains(*r2) || r2.contains(*r1))
        .count()
}

#[test]
fn t() {
    let x = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    let ans = x
        .lines()
        .map(|l| {
            let mut words = l.split(',');
            (
                words.next().unwrap().parse::<Range>().unwrap(),
                words.next().unwrap().parse::<Range>().unwrap(),
            )
        })
        .filter(|(r1, r2)| r1.contains(*r2) || r2.contains(*r1))
        .count();
    assert_eq!(ans, 2);
}

#[test]
fn t2() {
    use itertools::Itertools;
    let x = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    let v = x
        .lines()
        .map(|l| {
            let mut words = l.split(',');
            (
                words.next().unwrap().parse::<Range>().unwrap(),
                words.next().unwrap().parse::<Range>().unwrap(),
            )
        })
        .map(|(r1, r2)| r1.is_cross(r2))
        .collect_vec();
    let ans = vec![true, true, false, false, false, false];
    assert_eq!(ans, v);
}

pub fn part2() -> usize {
    input()
        .lines()
        .map(|lines| lines.unwrap())
        .map(|l| {
            let mut words = l.split(',');
            (
                words.next().unwrap().parse::<Range>().unwrap(),
                words.next().unwrap().parse::<Range>().unwrap(),
            )
        })
        .filter(|(r1, r2)| !r1.is_cross(*r2))
        .count()
}
