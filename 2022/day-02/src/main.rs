use core::panic;

use aoc_lib::cli;
use clap::Parser;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Choice {
    Rock,
    Paper,
    Scisor,
}

impl Choice {
    pub fn score(self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scisor => 3,
        }
    }

    pub fn res(self, other: Self) -> Res {
        use Choice::*;
        match (self, other) {
            (x, y) if x == y => Res::Draw,
            (Rock, Scisor) | (Scisor, Paper) | (Paper, Rock) => Res::Win,
            _ => Res::Lose,
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Res {
    Win,
    Lose,
    Draw,
}

impl Res {
    pub fn score(self) -> u32 {
        match self {
            Res::Win => 6,
            Res::Lose => 0,
            Res::Draw => 3,
        }
    }

    fn choice(self, other: Choice) -> Choice {
        match self {
            Res::Draw => other,
            Res::Lose => match other {
                Choice::Rock => Choice::Scisor,
                Choice::Paper => Choice::Rock,
                Choice::Scisor => Choice::Paper,
            },
            Res::Win => match other {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scisor,
                Choice::Scisor => Choice::Rock,
            },
        }
    }
}

fn parse_choice(i: &str) -> Choice {
    match i {
        "X" | "A" => Choice::Rock,
        "Y" | "B" => Choice::Paper,
        "Z" | "C" => Choice::Scisor,
        _ => panic!("invalid"),
    }
}
fn parse_res(i: &str) -> Res {
    match i {
        "X" => Res::Lose,
        "Y" => Res::Draw,
        "Z" => Res::Win,
        _ => panic!("invalid"),
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(' ');
            let a = parse_choice(it.next().unwrap());
            let b = parse_choice(it.next().unwrap());

            b.score() + b.res(a).score()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(' ');
            let a = parse_choice(it.next().unwrap());
            let b = parse_res(it.next().unwrap());

            b.score() + b.choice(a).score()
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let input = args.input.read()?;

    println!(
        "{}: {}",
        args.part,
        match args.part {
            cli::AocPart::Part1 => part1(&input).to_string(),
            cli::AocPart::Part2 => part2(&input).to_string(),
        }
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn p1_0() {
        let input = "A Y
B X
C Z
";
        let expected = 15;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "A Y
B X
C Z
";
        let expected = 12;
        assert_eq!(part2(input), expected);
    }
}
