use atoi::FromRadix10;
use fxhash::FxHashSet;
use std::{convert::Infallible, ops::Add, str::FromStr};

type Input = Point;
type Output = usize;

register!(
    "input/day18.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(points: &[Input]) -> Output {
    let set = points.iter().collect::<FxHashSet<_>>();
    points
        .iter()
        .map(|p| {
            [
                (1, 0, 0),
                (0, 1, 0),
                (0, 0, 1),
                (-1, 0, 0),
                (0, -1, 0),
                (0, 0, -1),
            ]
            .iter()
            .map(|delta| p + delta)
            .filter(|p| !set.contains(p))
            .count()
        })
        .sum()
}

fn part2(items: &[Input]) -> Output {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Add<&(isize, isize, isize)> for &Point {
    type Output = Point;

    fn add(self, rhs: &(isize, isize, isize)) -> Self::Output {
        Self::Output {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
        }
    }
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        let (x, c_x) = isize::from_radix_10(s);
        let (y, c_y) = isize::from_radix_10(&s[c_x + 1..]);
        let (z, _) = isize::from_radix_10(&s[c_x + c_y + 2..]);
        // Some points have 0 on some axis, avoid overflow.
        Ok(Self { x, y, z })
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
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 64);
        assert_eq!(res2, 0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 4418);
        assert_eq!(res2, 0);
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
