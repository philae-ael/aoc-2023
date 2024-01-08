use aoc_lib::{cli, ToVec};
use clap::Parser;

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut it = input.lines().map(|x| {
        let (_, b) = x.split_once(':').unwrap();

        b.split(' ').flat_map(|x| x.parse())
    });

    (it.next().unwrap().to_vec(), it.next().unwrap().to_vec())
}

fn part1(input: &str) -> u64 {
    let (v1, v2) = dbg!(parse(input));

    v1.into_iter()
        .zip(v2)
        .map(|(t, d)| {
            let delta = ((t as f32) * (t as f32)) - 4. * d as f32;
            let t_min = f32::floor(((t as f32) - delta.sqrt()) / 2.0) as u64 + 1;
            let t_max = f32::ceil(((t as f32) + delta.sqrt()) / 2.0) as u64 - 1;

            dbg!(t_min, t_max);
            t_max - t_min + 1
        })
        .product()
}
fn parse2(input: &str) -> (u64, u64) {
    let mut it = input.lines().map(|x| {
        let (_, b) = x.split_once(':').unwrap();

        let s = b.chars().filter(|x| x.is_ascii_digit()).collect::<String>();
        s.parse().unwrap()
    });

    (it.next().unwrap(), it.next().unwrap())
}
fn part2(input: &str) -> u64 {
    let (t, d) = dbg!(parse2(input));

    let delta = ((t as f64) * (t as f64)) - 4. * d as f64;
    let t_min = f64::floor(((t as f64) - delta.sqrt()) / 2.0) as u64 + 1;
    let t_max = f64::ceil(((t as f64) + delta.sqrt()) / 2.0) as u64 - 1;

    dbg!(t_min, t_max);
    t_max - t_min + 1
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
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let expected = 288;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let expected = 71503;
        assert_eq!(part2(input), expected);
    }
}
