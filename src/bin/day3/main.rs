use std::io::BufRead;

fn main() {
    dbg!(part1());
    dbg!(part2());
}

pub fn part1() -> usize {
    input()
        .lines()
        .map(|line| line.unwrap())
        .map(part1_line)
        .sum()
}

pub fn part2() -> usize {
    input()
        .lines()
        .map(|lines| lines.unwrap())
        .chunk3()
        .map(|(l1, l2, l3)| part2_chunk(l1, l2, l3))
        .sum()
}

trait Iterator3ChunkExt {
    type Iterator;
    fn chunk3(self) -> Chunk3<Self::Iterator>;
}

impl<I: Iterator> Iterator3ChunkExt for I {
    type Iterator = Self;

    fn chunk3(self) -> Chunk3<Self::Iterator> {
        Chunk3 { iter: self }
    }
}

struct Chunk3<I> {
    iter: I,
}

impl<I: Iterator> Iterator for Chunk3<I> {
    type Item = (
        <I as Iterator>::Item,
        <I as Iterator>::Item,
        <I as Iterator>::Item,
    );

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.iter.next()?, self.iter.next()?, self.iter.next()?))
    }
}

fn input() -> std::io::BufReader<std::fs::File> {
    let mut p = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    p.push("src");
    p.push(module_path!().split("::").last().unwrap());
    p.push("input.txt");
    std::io::BufReader::new(std::fs::File::open(p).unwrap())
}

fn char_point(c: char) -> usize {
    let p = match c {
        #[allow(clippy::char_lit_as_u8)]
        c @ 'a'..='z' => c as u8 - 'a' as u8 + 1,
        #[allow(clippy::char_lit_as_u8)]
        c @ 'A'..='Z' => c as u8 - 'A' as u8 + 1 + 26,
        _ => unreachable!(),
    };
    p as usize
}

#[test]
fn test_char_point() {
    assert_eq!(char_point('p'), 16);
    assert_eq!(char_point('L'), 38);
    assert_eq!(char_point('P'), 42);
    assert_eq!(char_point('v'), 22);
    assert_eq!(char_point('t'), 20);
}

fn part1_line<S: AsRef<str>>(line: S) -> usize {
    let line = line.as_ref();
    let len = line.len();
    let (first, last) = line.split_at(len / 2);
    ('a'..='z')
        .into_iter()
        .chain('A'..='Z')
        .filter(|c| first.contains(*c) && last.contains(*c))
        .map(char_point)
        .sum()
}

fn part2_chunk<S: AsRef<str>>(l1: S, l2: S, l3: S) -> usize {
    let lines = [l1.as_ref(), l2.as_ref(), l3.as_ref()];
    ('a'..='z')
        .into_iter()
        .chain('A'..='Z')
        .filter(|c| lines.iter().all(|l| l.contains(*c)))
        .map(char_point)
        .sum()
}
