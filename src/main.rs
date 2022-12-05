mod day1;
mod day2;
mod day3;
mod day4;

pub static CARGO_MANIFEST_DIR: &str = std::env!("CARGO_MANIFEST_DIR");

fn main() {
    println!("advent of code!");
    dbg!(day1::part1());
    dbg!(day1::part2());
    dbg!(day2::part1());
    dbg!(day2::part2());
    dbg!(day3::part1());
    dbg!(day3::part2());
    dbg!(day4::part1());
    dbg!(day4::part2());
}
