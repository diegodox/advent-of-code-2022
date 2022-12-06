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

pub fn part1() -> usize {
    let mut lines = input()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<usize>());

    let elf_sum = || {
        let mut it = lines
            .by_ref()
            .take_while(|line| line.is_ok())
            .map(|line| line.unwrap());
        it.next()
            .map(|first| std::iter::once(first).chain(it).sum())
    };

    let elf_sum_iter = std::iter::repeat_with(elf_sum)
        .take_while(|x| x.is_some())
        .flatten();

    elf_sum_iter.max().unwrap()
}

pub fn part2() -> usize {
    let mut lines = input()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<usize>());

    let elf_sum = || {
        let mut it = lines
            .by_ref()
            .take_while(|line| line.is_ok())
            .map(|line| line.unwrap());
        it.next()
            .map(|first| std::iter::once(first).chain(it).sum())
    };

    let elf_sum_iter = std::iter::repeat_with(elf_sum)
        .take_while(|x| x.is_some())
        .flatten();

    elf_sum_iter
        .fold([0; 3], |mut cur, v| {
            if v > cur[0] {
                cur.swap(1, 2);
                cur.swap(0, 1);
                cur[0] = v;
            } else if v > cur[1] {
                cur.swap(1, 2);
                cur[1] = v;
            } else if v > cur[2] {
                cur[2] = v;
            }
            cur
        })
        .into_iter()
        .sum()
}
