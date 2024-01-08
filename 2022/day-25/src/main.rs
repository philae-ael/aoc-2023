use core::panic;

use aoc_lib::cli;
use clap::Parser;

fn part1(input: &str) -> String {
    let mut i: i64 = input
        .lines()
        .map(|l| {
            l.chars().fold(0, |be, c| {
                be * 5
                    + match c {
                        '1' => 1,
                        '2' => 2,
                        '0' => 0,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!(),
                    }
            })
        })
        .sum();

    println!("{}", i);

    let mut s = String::new();
    while i > 0 {
        s.push(match i % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!(),
        });

        if i % 5 >= 3 {
            i += 5;
        }
        i /= 5;
    }

    s.chars().rev().collect()
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
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        let expected = "2=-1=0";
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "";
        let expected = "";
        assert_eq!(part2(input), expected);
    }
}
