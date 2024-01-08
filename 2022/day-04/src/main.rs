use aoc_lib::cli;

use clap::Parser;

fn parse_range(i: &str) -> std::ops::RangeInclusive<u32> {
    let mut it = i.split('-').map(|x| x.parse::<u32>().unwrap());
    it.next().unwrap()..=it.next().unwrap()
}

fn intersect(
    ra: &std::ops::RangeInclusive<u32>,
    rb: &std::ops::RangeInclusive<u32>,
) -> std::ops::RangeInclusive<u32> {
    u32::max(*ra.start(), *rb.start())..=u32::min(*ra.end(), *rb.end())
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let [ra, rb] = [a, b].map(parse_range);
            let i = intersect(&ra, &rb);

            if i == ra || i == rb {
                1
            } else {
                0
            }
        })
        .sum()
}
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let [ra, rb] = [a, b].map(parse_range);
            let i = intersect(&ra, &rb);

            if i.is_empty() {
                0
            } else {
                1
            }
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
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let expected = 2;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let expected = 4;
        assert_eq!(part2(input), expected);
    }
}
