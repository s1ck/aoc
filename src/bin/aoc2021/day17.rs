use std::{convert::Infallible, str::FromStr};

type Input = Target;
type Output = i32;

register!(
    "input/day17.txt";
    (input: input!(first input!(parse Input))) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(target: &Input) -> Output {
    simulate(target).0
}

fn part2(target: &Input) -> Output {
    simulate(target).1
}

fn simulate(target: &Target) -> (Output, Output) {
    let (x_min, x_max) = target.x_range;
    let (y_min, y_max) = target.y_range;

    let mut peak = 0;
    let mut vels = 0;

    (1..=x_max).for_each(|d_x| {
        (y_min..=y_min.abs()).for_each(|d_y| {
            let (hit, y_max) = fire(d_x, d_y, target);
            if hit {
                vels += 1;
                peak = Output::max(peak, y_max);
            }
        });
    });

    (peak, vels)
}

fn fire(mut x_v: i32, mut y_v: i32, target: &Target) -> (bool, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut peak = y;

    let (x_min, x_max) = target.x_range;
    let (y_min, y_max) = target.y_range;

    loop {
        x += x_v;
        y += y_v;

        x_v -= x_v.signum();
        y_v -= 1;

        peak = peak.max(y);

        if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
            return (true, peak);
        }
        if x > x_max || y < y_min {
            return (false, peak);
        }
    }
}

pub struct Target {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

impl FromStr for Target {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (x, y) = line[13..].split_once(", ").unwrap();
        let x_range = x[2..]
            .split_once("..")
            .map(|(l, h)| (l.parse::<i32>().unwrap(), h.parse::<i32>().unwrap()))
            .unwrap();
        let y_range = y[2..]
            .split_once("..")
            .map(|(l, h)| (l.parse::<i32>().unwrap(), h.parse::<i32>().unwrap()))
            .unwrap();

        Ok(Self { x_range, y_range })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"target area: x=20..30, y=-10..-5"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 45);
        assert_eq!(res2, 112);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 5995);
        assert_eq!(res2, 3202);
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
