use aoc_lib::{cli, ToVec};
use clap::Parser;

fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .filter(|&x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}
fn part2(input: &str) -> u32 {
    let mut v = input
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .filter(|&x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .to_vec();

    v.sort();
    v.iter().rev().take(3).sum()
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
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        let expected = 24000;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        let expected = 45000;
        assert_eq!(part2(input), expected);
    }
}
