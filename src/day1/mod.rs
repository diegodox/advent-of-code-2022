#[allow(unused)]
pub fn part1() {
    let mut lines = std::io::stdin()
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

    let ans: usize = elf_sum_iter.max().unwrap();

    println!("{ans}");
}

#[allow(unused)]
pub fn part2() {
    let mut lines = std::io::stdin()
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

    let ans: usize = elf_sum_iter
        .fold(([0; 3]), |mut cur, v| {
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
        .sum();

    println!("{ans}");
}
