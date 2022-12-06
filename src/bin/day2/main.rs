use std::io::BufRead;

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn play(self, other: Self) -> WinDrawLose {
        match (self, other) {
            (Hand::Rock, Hand::Rock)
            | (Hand::Paper, Hand::Paper)
            | (Hand::Scissors, Hand::Scissors) => WinDrawLose::Draw,

            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => WinDrawLose::Win,

            (Hand::Rock, Hand::Paper)
            | (Hand::Paper, Hand::Scissors)
            | (Hand::Scissors, Hand::Rock) => WinDrawLose::Lose,
        }
    }

    fn from_result_and_op_hand(result: WinDrawLose, op_hand: Hand) -> Self {
        match (result, op_hand) {
            (WinDrawLose::Draw, x) => x,

            (WinDrawLose::Win, Hand::Rock) => Hand::Paper,
            (WinDrawLose::Win, Hand::Paper) => Hand::Scissors,
            (WinDrawLose::Win, Hand::Scissors) => Hand::Rock,

            (WinDrawLose::Lose, Hand::Rock) => Hand::Scissors,
            (WinDrawLose::Lose, Hand::Paper) => Hand::Rock,
            (WinDrawLose::Lose, Hand::Scissors) => Hand::Paper,
        }
    }

    fn point(self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl std::str::FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum WinDrawLose {
    Win,
    Draw,
    Lose,
}

impl WinDrawLose {
    fn point(self) -> usize {
        match self {
            WinDrawLose::Lose => 0,
            WinDrawLose::Draw => 3,
            WinDrawLose::Win => 6,
        }
    }
}

impl std::str::FromStr for WinDrawLose {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StrategyPart1 {
    my_hand: Hand,
    op_hand: Hand,
}

impl StrategyPart1 {
    fn point(self) -> usize {
        self.my_hand.point() + self.my_hand.play(self.op_hand).point()
    }
}

impl std::str::FromStr for StrategyPart1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        Ok(Self {
            op_hand: words.next().ok_or(())?.parse()?,
            my_hand: words.next().ok_or(())?.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StrategyPart2 {
    op_hand: Hand,
    result: WinDrawLose,
}

impl StrategyPart2 {
    pub fn point(self) -> usize {
        let my_hand = Hand::from_result_and_op_hand(self.result, self.op_hand);
        my_hand.point() + self.result.point()
    }
}

impl std::str::FromStr for StrategyPart2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        Ok(Self {
            op_hand: words.next().ok_or(())?.parse()?,
            result: words.next().ok_or(())?.parse()?,
        })
    }
}

pub fn part1() -> usize {
    input()
        .lines()
        .map(|l| l.unwrap().parse::<StrategyPart1>().unwrap())
        .map(|s| s.point())
        .sum()
}

pub fn part2() -> usize {
    input()
        .lines()
        .map(|l| l.unwrap().parse::<StrategyPart2>().unwrap())
        .map(|s| s.point())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT2: &str = "\
A Y
B X
C Z
";
    #[test]
    fn test_2() {
        let s: Vec<_> = INPUT2
            .lines()
            .map(|l| l.parse::<StrategyPart2>().unwrap())
            .collect();

        assert_eq!(
            s,
            vec![
                StrategyPart2 {
                    result: WinDrawLose::Draw,
                    op_hand: Hand::Rock
                },
                StrategyPart2 {
                    result: WinDrawLose::Lose,
                    op_hand: Hand::Paper
                },
                StrategyPart2 {
                    result: WinDrawLose::Win,
                    op_hand: Hand::Scissors
                }
            ]
        );

        let h: Vec<_> = s
            .into_iter()
            .map(|x| Hand::from_result_and_op_hand(x.result, x.op_hand))
            .collect();
        assert_eq!(h, vec![Hand::Rock, Hand::Rock, Hand::Rock]);
    }
}
