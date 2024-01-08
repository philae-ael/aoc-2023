use aoc_lib::cli;
use clap::Parser;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, combinator::value,
    multi::separated_list0, IResult,
};

struct Color {
    index: usize,
    count: u32,
}

struct Line {
    game_id: u32,
    l: Vec<Vec<Color>>,
}

fn parser(line: &str) -> IResult<&str, Line> {
    let (input, _) = tag("Game ")(line)?;
    let (input, game_id) = u32(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, l) = separated_list0(
        tag("; "),
        separated_list0(tag(", "), |line| {
            let (input, count) = u32(line)?;
            let (input, _) = tag(" ")(input)?;
            let (input, color) = alt((
                value(0, tag("red")),
                value(1, tag("blue")),
                value(2, tag("green")),
            ))(input)?;

            Ok((
                input,
                Color {
                    index: color,
                    count,
                },
            ))
        }),
    )(input)?;

    Ok((input, Line { game_id, l }))
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (_, line) = parser(l).unwrap();

            let s = line.l.iter().flatten().fold([0, 0, 0], |mut acc, item| {
                acc[item.index] = acc[item.index].max(item.count);
                acc
            });

            if s[0] > 12 || s[1] > 13 || s[2] > 14 {
                0
            } else {
                line.game_id
            }
        })
        .sum()
}
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (_, line) = parser(l).unwrap();

            let s = line.l.iter().flatten().fold([0, 0, 0], |mut acc, item| {
                acc[item.index] = acc[item.index].max(item.count);
                acc
            });

            s.into_iter().reduce(|a, b| a * b).unwrap()
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 8;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 2286;
        assert_eq!(part2(input), expected);
    }
}
