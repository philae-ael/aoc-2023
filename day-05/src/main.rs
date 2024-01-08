use std::{collections::HashMap, ops::Range};

use aoc_lib::{cli, ToVec};
use clap::Parser;

#[derive(Debug)]
struct RangeConverter {
    dst_start: u64,
    src_range: Range<u64>,
}

impl RangeConverter {
    fn map(&self, x: u64) -> u64 {
        (x + self.dst_start) - self.src_range.start
    }
}

pub fn simplify_rangelist(l: &mut Vec<Range<u64>>) {
    if l.is_empty() {
        return;
    }

    l.sort_by_key(|x| x.start);
    let mut v = vec![];

    let mut it = l.iter().cloned();
    let mut current_range = it.next().unwrap();
    for i in it {
        if current_range.end > i.start {
            current_range.end = i.end.max(current_range.end);
        } else {
            v.push(current_range);
            current_range = i;
        }
    }
    v.push(current_range);

    *l = v;
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, (String, Vec<RangeConverter>)>,
}

fn parse(input: &str) -> Almanac {
    let mut it = input.split("\n\n");

    let seeds = it
        .next()
        .unwrap()
        .split(' ')
        .flat_map(|x| x.parse::<u64>())
        .to_vec();

    let maps = it
        .map(|map| {
            let mut lines = map.lines();
            let mut metadata = lines.next().unwrap().split(' ').next().unwrap().split('-');
            let from = metadata.next().unwrap();
            let to = metadata.last().unwrap();

            let rangemaps = lines
                .map(|l| {
                    let mut ss = l.split(' ').flat_map(|x| x.parse::<u64>());
                    let dst_start = ss.next().unwrap();
                    let src_start = ss.next().unwrap();
                    let src_length = ss.next().unwrap();
                    RangeConverter {
                        dst_start,
                        src_range: (src_start..(src_start + src_length)),
                    }
                })
                .to_vec();

            (from.to_owned(), (to.to_owned(), rangemaps))
        })
        .collect();

    Almanac { seeds, maps }
}

fn part1(input: &str) -> u64 {
    let almanac = parse(input);

    almanac
        .seeds
        .into_iter()
        .map(|mut item| {
            let mut next = "seed".to_owned();

            while let Some((to, range_maps)) = almanac.maps.get(&next) {
                next = to.to_owned();
                for rc in range_maps {
                    if rc.src_range.contains(&item) {
                        item = rc.map(item);
                        break;
                    }
                }
            }
            item
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let almanac = parse(input);

    let mut next = "seed".to_owned();
    let mut ranges = almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
        .to_vec();
    while let Some((to, range_maps)) = almanac.maps.get(&next) {
        next = to.to_owned();

        ranges = ranges
            .into_iter()
            .flat_map(|item| {
                let mut v = vec![item];
                let mut w = vec![];
                for rm in range_maps {
                    let mut new_v = vec![];
                    for i in v {
                        match (
                            rm.src_range.contains(&i.start),
                            rm.src_range.contains(&(i.end - 1)),
                            i.contains(&rm.src_range.start),
                        ) {
                            (true, true, _) => w.push(rm.map(i.start)..rm.map(i.end)),
                            (true, false, _) => {
                                w.push(rm.map(i.start)..rm.map(rm.src_range.end));
                                new_v.push(rm.src_range.end..i.end);
                            }
                            (false, true, _) => {
                                w.push(rm.map(rm.src_range.start)..rm.map(i.end));
                                new_v.push(i.start..rm.src_range.start);
                            }
                            (false, false, true) => {
                                w.push(rm.map(rm.src_range.start)..rm.map(rm.src_range.end));
                                new_v.push(i.start..rm.src_range.start);
                                new_v.push(rm.src_range.end..i.end);
                            }
                            (false, false, false) => new_v.push(i),
                        };
                    }
                    v = new_v;
                }

                v.into_iter().chain(w)
            })
            .to_vec();

        simplify_rangelist(&mut ranges);
    }

    ranges[0].start
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
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let expected = 35;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn p2_0() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let expected = 46;
        assert_eq!(part2(input), expected);
    }
}
