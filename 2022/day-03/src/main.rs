#![feature(iter_array_chunks)]
use std::collections::HashSet;

use aoc_lib::cli;
use clap::Parser;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut it = l.chars();

            let seta = std::collections::HashSet::<char>::from_iter((&mut it).take(l.len() / 2));
            let setb = std::collections::HashSet::<char>::from_iter(it);

            seta.intersection(&setb)
                .map(|x| {
                    if x.is_lowercase() {
                        *x as u32 - 'a' as u32 + 1
                    } else {
                        *x as u32 - 'A' as u32 + 27
                    }
                })
                .sum::<u32>()
        })
        .sum()
}
fn part2(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let seta = std::collections::HashSet::<char>::from_iter(a.chars());
            let setb = std::collections::HashSet::<char>::from_iter(b.chars());
            let setc = std::collections::HashSet::<char>::from_iter(c.chars());

            let setab: HashSet<char> = seta.intersection(&setb).copied().collect();
            setab
                .intersection(&setc)
                .map(|x| {
                    if x.is_lowercase() {
                        *x as u32 - 'a' as u32 + 1
                    } else {
                        *x as u32 - 'A' as u32 + 27
                    }
                })
                .sum::<u32>()
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
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let expected = 157;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let expected = 70;
        assert_eq!(part2(input), expected);
    }
}
