use aoc_lib::cli;
use clap::Parser;
use itertools::Itertools;

#[derive(Debug)]
struct Number {
    value: u32,
    start: usize,
    end: usize,
    used: bool,
}

fn part1(input: &str) -> u32 {
    let mut numbers = vec![];
    let mut gears = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut v = vec![];
        for (is_number, group) in line
            .char_indices()
            .group_by(|(_, ch)| ch.is_ascii_digit())
            .into_iter()
        {
            if is_number {
                let (idx, num_str): (Vec<usize>, String) = group.unzip();

                let num = num_str.parse::<u32>().unwrap();
                v.push(Number {
                    value: num,
                    start: idx[0],
                    end: idx[idx.len() - 1],
                    used: false,
                })
            } else {
                for (j, ch) in group {
                    if ch != '.' {
                        gears.push((i, j))
                    }
                }
            }
        }

        numbers.push(v);
    }

    let mut sum = 0;

    for (i, j) in gears {
        for dy in [-1isize, 0, 1] {
            let y = i as isize + dy;

            if y < 0 || y >= numbers.len() as isize {
                continue;
            }
            for number in &mut numbers[y as usize] {
                for dx in [-1isize, 0, 1] {
                    let x = j as isize + dx;
                    if !number.used
                        && ((number.start as isize)..=(number.end as isize)).contains(&x)
                    {
                        sum += number.value;
                        number.used = true;
                    }
                }
            }
        }
    }

    sum
}
fn part2(input: &str) -> u32 {
    let mut numbers = vec![];
    let mut gears = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut v = vec![];
        for (is_number, group) in line
            .char_indices()
            .group_by(|(_, ch)| ch.is_ascii_digit())
            .into_iter()
        {
            if is_number {
                let (idx, num_str): (Vec<usize>, String) = group.unzip();

                let num = num_str.parse::<u32>().unwrap();
                v.push(Number {
                    value: num,
                    start: idx[0],
                    end: idx[idx.len() - 1],
                    used: false,
                })
            } else {
                for (j, ch) in group {
                    if ch != '.' {
                        gears.push((i, j))
                    }
                }
            }
        }

        numbers.push(v);
    }

    let mut sum = 0;

    for (i, j) in gears {
        let mut adjacent = vec![];
        for dy in [-1isize, 0, 1] {
            let y = i as isize + dy;

            if y < 0 || y >= numbers.len() as isize {
                continue;
            }
            for number in &mut numbers[y as usize] {
                for dx in [-1isize, 0, 1] {
                    let x = j as isize + dx;
                    if !number.used
                        && ((number.start as isize)..=(number.end as isize)).contains(&x)
                    {
                        adjacent.push(number.value);
                        number.used = true;
                    }
                }
            }
        }
        if adjacent.len() == 2 {
            sum += adjacent[0] * adjacent[1];
        }
    }

    sum
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
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected = 4361;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected = 467835;
        assert_eq!(part2(input), expected);
    }
}
