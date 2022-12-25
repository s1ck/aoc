use std::{convert::Infallible, iter::Sum, str::FromStr};

type Input = Snafu;
type Output = String;

register!(
    "input/day25.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items.iter().copied().sum::<Snafu>().to_string()
}

fn part2(items: &[Input]) -> Output {
    String::new()
}

#[derive(Debug, Copy, Clone)]
pub struct Snafu(i64);

impl ToString for Snafu {
    fn to_string(&self) -> String {
        fn to_str<'a>(n: i64) -> &'a str {
            match n {
                -2 => "=",
                -1 => "-",
                0 => "0",
                1 => "1",
                2 => "2",
                _ => unreachable!(),
            }
        }

        fn to_snafu(n: i64, res: &mut String) {
            if n == 0 {
                return;
            }
            match n % 5 {
                0 | 1 | 2 => {
                    res.push_str(to_str(n % 5));
                    to_snafu(n / 5, res);
                }
                3 | 4 => {
                    res.push_str(to_str(n % 5 - 5));
                    to_snafu(n / 5 + 1, res);
                }
                _ => unreachable!(),
            }
        }

        let mut res = String::new();
        to_snafu(self.0, &mut res);
        res.chars().rev().collect::<_>()
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|s| s.0).sum())
    }
}

impl FromStr for Snafu {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let mut num = 0;
        for i in 0..bytes.len() {
            let pos = bytes.len() - 1 - i;
            let n = match bytes[pos] {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => unreachable!(),
            };
            num += i64::pow(5, i as u32) * n;
        }

        Ok(Self(num))
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
        1=-0-2
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
        122
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, "2=-1=0");
        assert_eq!(res2, "");
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, "2-2=21=0021=-02-1=-0");
        assert_eq!(res2, "");
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
