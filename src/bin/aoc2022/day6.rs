use std::{collections::HashSet, convert::Infallible, str::FromStr};

type Input = Buffer;
type Output = usize;

register!(
    "input/day6.txt";
    (input: input!(first input!(parse Input))) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(buffer: &Input) -> Output {
    find_marker::<4>(buffer)
}

fn part2(buffer: &Input) -> Output {
    find_marker::<14>(buffer)
}

fn find_marker<const N: usize>(buffer: &Input) -> Output {
    buffer
        .0
        .as_bytes()
        .array_windows::<N>()
        .enumerate()
        .take_while(|(i, arr)| HashSet::<&u8>::from_iter(arr.iter()).len() != N)
        .map(|(i, _)| i + N + 1)
        .last()
        .unwrap()
}

pub struct Buffer(String);

impl FromStr for Buffer {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
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
        mjqjpqmgbljsphdztnvjfqwrcgsmlb
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 7);
        assert_eq!(res2, 19);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1912);
        assert_eq!(res2, 2122);
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
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input));
    }
}
