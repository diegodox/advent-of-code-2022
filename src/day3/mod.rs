use itertools::{self, Itertools};
use std::io::BufRead;

fn input() -> std::io::BufReader<std::fs::File> {
    let mut p = std::path::PathBuf::from(crate::CARGO_MANIFEST_DIR);
    p.push("src/day3/input.txt");
    std::io::BufReader::new(std::fs::File::open(p).unwrap())
}

pub fn part1() -> usize {
    input()
        .lines()
        .map(|lines| lines.unwrap())
        .map(|l| {
            let len = l.len();
            let (first, last) = l.split_at(len / 2);
            ('a'..='z')
                .into_iter()
                .chain('A'..='Z')
                // .inspect(|x| println!("{x}"))
                .filter(|c| first.contains(*c) && last.contains(*c))
                // .inspect(|c| println!("{c}"))
                .map(|c| match c {
                    c @ 'a'..='z' => c as u8 - 'a' as u8 + 1,
                    c @ 'A'..='Z' => c as u8 - 'A' as u8 + 1 + 26,
                    _ => unreachable!(),
                })
                .map(|x| x as usize)
                .sum::<usize>()
        })
        // .take(1)
        .sum()
}

pub fn part2() -> usize {
    input()
        .lines()
        .map(|lines| lines.unwrap())
        .chunks(3)
        .into_iter()
        .map(|l| {
            let l = l.collect_vec();
            ('a'..='z')
                .into_iter()
                .chain('A'..='Z')
                // .inspect(|x| println!("{x}"))
                .filter(|c| l.iter().all(|l| l.contains(*c)))
                // .inspect(|c| println!("{c}"))
                .map(|c| match c {
                    c @ 'a'..='z' => c as u8 - 'a' as u8 + 1,
                    c @ 'A'..='Z' => c as u8 - 'A' as u8 + 1 + 26,
                    _ => unreachable!(),
                })
                .map(|x| x as usize)
                .sum::<usize>()
        })
        // .take(1)
        .sum()
}

pub struct Chunk3<I> {
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
