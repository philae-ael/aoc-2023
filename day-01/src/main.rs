use aoc_lib::{cli, ToVec};
use clap::Parser;
use nom::character::streaming::{anychar, satisfy};
use nom::{branch, bytes::streaming::tag, combinator, IResult};

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut it = l.chars().filter(|x| x.is_ascii_digit());
            let fst = it.next().unwrap();
            let last = it.last().unwrap_or(fst);

            format!("{fst}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

fn parse_once(input: &str) -> IResult<&str, Option<u32>> {
    branch::alt((
        combinator::map(tag("zero"), |_| Some(1)),
        combinator::map(tag("one"), |_| Some(1)),
        combinator::map(tag("two"), |_| Some(2)),
        combinator::map(tag("three"), |_| Some(3)),
        combinator::map(tag("four"), |_| Some(4)),
        combinator::map(tag("five"), |_| Some(5)),
        combinator::map(tag("six"), |_| Some(6)),
        combinator::map(tag("seven"), |_| Some(7)),
        combinator::map(tag("eight"), |_| Some(8)),
        combinator::map(tag("nine"), |_| Some(9)),
        combinator::map(satisfy(|c| c.is_ascii_digit()), |c| {
            c.to_string().parse().ok()
        }),
        combinator::map(anychar, |_| None),
    ))(input)
}

fn parser(input: &str) -> Vec<u32> {
    nom::combinator::iterator(input, parse_once)
        .flatten()
        .to_vec()
}
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let v = parser(l);
            let fst = v[0];
            let last = v[v.len() - 1];

            format!("{fst}{last}").parse::<u32>().unwrap()
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let expected = 142;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let expected = 281;
        assert_eq!(part2(input), expected);
    }
}
