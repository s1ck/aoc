use std::{convert::Infallible, str::FromStr};

use fxhash::FxHashSet;

type Input = Command;
type Output = usize;

register!(
    "input/day9.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(commands: &[Input]) -> Output {
    fn next_t(h: (i16, i16), t: (i16, i16), step: (i16, i16)) -> (i16, i16) {
        let dx = i16::abs(h.0 - t.0);
        let dy = i16::abs(h.1 - t.1);

        if dx <= 1 && dy <= 1 {
            t
        } else if dx == 2 && dy == 0 {
            // left and right
            (t.0 + step.0, t.1 + step.1)
        } else if dx == 0 && dy == 2 {
            // up and down
            (t.0 + step.0, t.1 + step.1)
        } else {
            // diagonal, move to prev h position
            (h.0 + step.0 * -1, h.1 + step.1 * -1)
        }
    }
    let mut visits = FxHashSet::<(i16, i16)>::default();
    let mut h = (0, 0);
    let mut t = (0, 0);

    for cmd in commands {
        let (dx, dy) = cmd.step;
        for _ in 0..cmd.times {
            h = (h.0 + dx, h.1 + dy);
            t = next_t(h, t, cmd.step);
            visits.insert(t);
        }
    }

    visits.len()
}

fn part2(items: &[Input]) -> Output {
    0
}

#[derive(Debug)]
pub struct Command {
    step: (i16, i16),
    times: u8,
}

impl FromStr for Command {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (step, times) = s.split_once(' ').unwrap();

        let step = match step {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        let times = times.parse::<u8>().unwrap();

        Ok(Self { step, times })
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
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 13);
        assert_eq!(res2, 1);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 5883);
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
