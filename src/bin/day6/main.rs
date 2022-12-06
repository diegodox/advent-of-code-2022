use std::{fmt::Write, io::BufRead};

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn part1() -> usize {
    let line = input().lines().next().unwrap().unwrap();
    Solver::<_, 4>::default().solve(line.chars())
}

fn part2() -> usize {
    let line = input().lines().next().unwrap().unwrap();
    Solver::<_, 14>::default().solve(line.chars())
}

fn input() -> std::io::BufReader<std::fs::File> {
    let mut p = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    p.push("src");
    p.push("bin");
    p.push(module_path!().split("::").last().unwrap());
    p.push("input.txt");
    std::io::BufReader::new(std::fs::File::open(p).unwrap())
}

#[derive(Clone, PartialEq, Eq)]
struct Solver<T, const N: usize> {
    window: std::collections::VecDeque<T>,
    guard: usize,
}

impl<const N: usize, T> Default for Solver<T, N> {
    fn default() -> Self {
        Self {
            window: Default::default(),
            guard: N,
        }
    }
}

impl<T: std::fmt::Display, const N: usize> std::fmt::Display for Solver<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.write_str("( [")?;
        for q in self.window.iter() {
            s.write_str(&format!("{q}, "))?;
        }
        s.write_str(&format!("] {} )", self.guard))?;
        f.write_str(&s)?;
        Ok(())
    }
}

impl<T: PartialEq, const N: usize> Solver<T, N> {
    fn solve(mut self, iter: impl Iterator<Item = T>) -> usize {
        iter.enumerate()
            .find_map(|q| self.process_char(q.1).then_some(q.0 + 1))
            .unwrap()
    }

    fn process_char(&mut self, v: T) -> bool {
        if let Some(x) = self.guard.checked_sub(1) {
            self.guard = x;
        }

        let last_match = self
            .window
            .iter()
            .enumerate()
            .filter_map(|c| (c.1 == &v).then_some(c.0))
            .last();

        match last_match {
            None if self.guard == 0 => {
                return true;
            }

            Some(x) => {
                let guard = if self.window.len() >= N - 1 {
                    x + 1
                } else {
                    N + x - self.window.len()
                };
                self.guard = self.guard.max(guard);
            }

            _ => { /* do nothing */ }
        }

        if self.window.len() >= N - 1 {
            self.window.pop_front();
        }
        self.window.push_back(v);

        false
    }
}

#[test]
fn part1_example() {
    let line = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    let ans = [7, 5, 6, 10, 11];
    std::iter::zip(line, ans).for_each(|(line, ans)| {
        let x = Solver::<_, 4>::default().solve(line.chars());
        assert_eq!(x, ans);
    });
}

#[test]
fn part2_example() {
    let line = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    let ans = [19, 23, 23, 29, 26];
    std::iter::zip(line, ans).for_each(|(line, ans)| {
        let x = Solver::<_, 14>::default().solve(line.chars());
        assert_eq!(x, ans);
    });
}
