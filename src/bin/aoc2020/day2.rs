use std::{num::ParseIntError, str::FromStr};

use aoc::{lines, PuzzleInput};

type Input = Passwords;
type Output = usize;

register!(
    "input/day2.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(input);
        part2(input);
    }
);

fn part1(items: (usize, usize)) -> Output {
    items.0
}

fn part2(items: (usize, usize)) -> Output {
    items.1
}

pub struct Passwords;

impl PuzzleInput for Passwords {
    type Out = (Output, Output);

    fn from_input(input: &str) -> Self::Out {
        lines(input).map(|line| line.parse::<Line>().unwrap()).fold(
            (0, 0),
            |(in_range_cnt, at_index_cnt), Line { in_range, at_index }| {
                (
                    in_range_cnt + usize::from(in_range),
                    at_index_cnt + usize::from(at_index),
                )
            },
        )
    }
}

struct Line {
    in_range: bool,
    at_index: bool,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_ascii_whitespace();
        let (from, to) = s.next().unwrap().split_once('-').unwrap();
        let (from, to) = (from.parse::<u8>()?, to.parse::<u8>()?);
        let needle = s.next().unwrap().as_bytes()[0];
        let haystack = s.next().unwrap().as_bytes();
        let count = bytecount::count(haystack, needle);
        let from = usize::from(from);
        let to = usize::from(to);
        let in_range = count >= from && count <= to;
        let at_index = (haystack[from - 1] == needle) ^ (haystack[to - 1] == needle);

        Ok(Self { in_range, at_index })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 2);
        assert_eq!(res2, 1);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 424);
        assert_eq!(res2, 747);
    }

    #[bench]
    fn bench_parsing(b: &mut Bencher) {
        let input = Solver::puzzle_input();
        b.bytes = input.len() as u64;
        b.iter(|| Solver::parse_input(input));
    }

    #[bench]
    fn bench_pt1(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part1(input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(input));
    }
}
