use atoi::FromRadix10;
use fxhash::FxHashSet;
use std::{cmp::Ordering, convert::Infallible, ops::Add, str::FromStr};

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
        .map(|p| p.surrounding().filter(|p| !set.contains(p)).count())
        .sum()
}

fn part2(points: &[Input]) -> Output {
    // Discover the surrounding space of the lava droplet using
    // a bounded depth-first-search. Then we can check which
    // cubes have a side facing the outside.
    let p_min = points.iter().min().unwrap() + &(-1, -1, -1);
    let p_max = points.iter().max().unwrap() + &(1, 1, 1);

    let point_set = points.iter().collect::<FxHashSet<_>>();
    let mut seen = FxHashSet::default();
    let mut stack = vec![p_min];

    loop {
        let Some(next) = stack.pop() else {
            break;
        };

        if next >= p_min && next <= p_max && !point_set.contains(&next) && !seen.contains(&next) {
            seen.insert(next);
            next.surrounding().for_each(|p| stack.push(p));
        }
    }

    points
        .iter()
        .map(|p| p.surrounding().filter(|p| seen.contains(p)).count())
        .sum()
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

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let d0 = self.x.pow(2) + self.y.pow(2) + self.z.pow(2);
        let d1 = other.x.pow(2) + other.y.pow(2) + other.z.pow(2);
        d0.cmp(&d1)
    }
}

impl Point {
    fn surrounding(&self) -> impl Iterator<Item = Self> + '_ {
        [
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
        ]
        .iter()
        .map(move |delta| self + delta)
    }
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        let (x, c_x) = isize::from_radix_10(s);
        let (y, c_y) = isize::from_radix_10(&s[c_x + 1..]);
        let (z, _) = isize::from_radix_10(&s[c_x + c_y + 2..]);
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
        assert_eq!(res2, 58);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 4418);
        assert_eq!(res2, 2486);
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
