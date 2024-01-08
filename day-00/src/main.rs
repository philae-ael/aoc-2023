use clap::Parser;
use aoc_lib::cli;

fn part1(input: &str) -> String {
    input.into()
}
fn part2(input: &str) -> String {
    input.into()
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
        let input = "";
        let expected = "";
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "";
        let expected = "";
        assert_eq!(part2(input), expected);
    }
}
