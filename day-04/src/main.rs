use aoc_lib::cli;
use clap::Parser;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(':').unwrap();
            let (winning, got) = line.split_once('|').unwrap();

            let winning = std::collections::HashSet::<u32>::from_iter(
                winning.split(' ').flat_map(|x| x.parse()),
            );
            let got =
                std::collections::HashSet::<u32>::from_iter(got.split(' ').flat_map(|x| x.parse()));

            let inter = winning.intersection(&got).count();
            if inter == 0 {
                0
            } else {
                2u32.pow(inter as u32 - 1)
            }
        })
        .sum()
}
fn part2(input: &str) -> u32 {
    let mut scratchpad = std::collections::HashMap::<usize, u32>::new();
    let s = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (_, line) = line.split_once(':').unwrap();
            let (winning, got) = line.split_once('|').unwrap();

            let winning = std::collections::HashSet::<u32>::from_iter(
                winning.split(' ').flat_map(|x| x.parse()),
            );
            let got =
                std::collections::HashSet::<u32>::from_iter(got.split(' ').flat_map(|x| x.parse()));

            let inter = winning.intersection(&got).count();
            let n = *scratchpad.get(&i).unwrap_or(&0) + 1;
            for j in 0..inter {
                *scratchpad.entry(i + 1 + j).or_insert(0) += n;
            }
            n
        })
        .sum();

    println!("{:?}", scratchpad);
    s
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
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let expected = 13;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let expected = 30;
        assert_eq!(part2(input), expected);
    }
}
