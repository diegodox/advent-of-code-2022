use std::{collections::LinkedList, io::BufRead};

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn input() -> std::io::BufReader<std::fs::File> {
    let mut p = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    p.push("src");
    p.push("bin");
    p.push(module_path!().split("::").last().unwrap());
    p.push("input.txt");
    std::io::BufReader::new(std::fs::File::open(p).unwrap())
}

#[cfg(test)]
fn example() -> std::io::BufReader<std::fs::File> {
    let mut p = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    p.push("src");
    p.push("bin");
    p.push(module_path!().split("::").last().unwrap());
    p.push("example.txt");
    std::io::BufReader::new(std::fs::File::open(p).unwrap())
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Ship(Vec<LinkedList<char>>);

impl std::fmt::Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, stack) in self.0.iter().enumerate() {
            writeln!(f, "{i}: {stack:?}")?
        }
        std::fmt::Result::Ok(())
    }
}

impl Ship {
    fn move_crates_each(&mut self, procedure: Procedure) {
        let (to, from) = {
            let pointer = &mut self.0 as *mut Vec<LinkedList<char>>;
            // Safety: both indeces `to` and `from` are not same and not out-of-bound.
            unsafe {
                (
                    (*pointer).get_unchecked_mut(procedure.to - 1),
                    (*pointer).get_unchecked_mut(procedure.from - 1),
                )
            }
        };
        for _ in 0..procedure.num_crate {
            to.push_back(from.pop_back().unwrap())
        }
    }

    fn move_crates_once(&mut self, procedure: Procedure) {
        let (to, from) = {
            let pointer = &mut self.0 as *mut Vec<LinkedList<char>>;
            // Safety: both indeces `to` and `from` are not same and not out-of-bound.
            unsafe {
                (
                    (*pointer).get_unchecked_mut(procedure.to - 1),
                    (*pointer).get_unchecked_mut(procedure.from - 1),
                )
            }
        };
        to.append(&mut from.split_off(from.len() - procedure.num_crate));
    }

    fn lasts(self) -> impl Iterator<Item = char> {
        self.0.into_iter().map(|stack| *stack.back().unwrap())
    }
}

impl FromIterator<String> for Ship {
    fn from_iter<T: IntoIterator<Item = String>>(lines: T) -> Self {
        #[allow(clippy::needless_collect)]
        let lines: Vec<_> = lines.into_iter().collect();
        let mut lines = lines.into_iter().rev();
        let n_stack = lines.next().unwrap().split_whitespace().count();
        let ship = lines
            .map(|mut line| {
                std::iter::repeat_with(move || {
                    let mut remain = line.split_off(4.min(line.len()));
                    std::mem::swap(&mut remain, &mut line);
                    remain
                })
                .take(n_stack)
                .map(|x| x.chars().nth(1).unwrap_or(' '))
                .map(|x| if x != ' ' { Some(x) } else { None })
            })
            .fold(vec![LinkedList::new(); n_stack], |mut cum, x| {
                cum.iter_mut()
                    .zip(x)
                    .filter_map(|(stack, w)| w.map(|w| (stack, w)))
                    .for_each(|(stack, i)| stack.push_back(i));
                cum
            });

        Self(ship)
    }
}

struct Procedure {
    num_crate: usize,
    from: usize,
    to: usize,
}

impl std::str::FromStr for Procedure {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut w = s.split_whitespace();
        Ok(Self {
            num_crate: w.nth(1).unwrap().parse().unwrap(),
            from: w.nth(1).unwrap().parse().unwrap(),
            to: w.nth(1).unwrap().parse().unwrap(),
        })
    }
}

#[test]
fn part1_example() {
    let mut lines = example().lines().map(|lines| lines.unwrap());
    let mut ship = (&mut lines).take_while(|x| !x.is_empty()).collect::<Ship>();
    println!("{}", &ship);
    lines
        .map(|line| line.parse::<Procedure>().unwrap())
        .for_each(|procedure| {
            ship.move_crates_each(procedure);
            println!("{}", &ship);
        });
    let ans = ship.lasts().collect::<String>();
    assert_eq!("CMZ", dbg!(ans));
}

pub fn part1() -> String {
    let mut lines = input().lines().map(|lines| lines.unwrap());
    let mut ship = (&mut lines).take_while(|x| !x.is_empty()).collect::<Ship>();
    lines
        .map(|line| line.parse::<Procedure>().unwrap())
        .for_each(|procedure| ship.move_crates_each(procedure));
    ship.lasts().collect()
}

#[test]
fn part2_example() {
    let mut lines = example().lines().map(|lines| lines.unwrap());
    let mut ship = (&mut lines).take_while(|x| !x.is_empty()).collect::<Ship>();
    println!("{}", &ship);
    lines
        .map(|line| line.parse::<Procedure>().unwrap())
        .for_each(|procedure| {
            ship.move_crates_once(procedure);
            println!("{}", &ship);
        });
    let ans = ship.lasts().collect::<String>();
    assert_eq!("MCD", dbg!(ans));
}

pub fn part2() -> String {
    let mut lines = input().lines().map(|lines| lines.unwrap());
    let mut ship = (&mut lines).take_while(|x| !x.is_empty()).collect::<Ship>();
    lines
        .map(|line| line.parse::<Procedure>().unwrap())
        .for_each(|procedure| ship.move_crates_once(procedure));
    ship.lasts().collect()
}
