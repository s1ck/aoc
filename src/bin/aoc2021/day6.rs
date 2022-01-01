use std::{convert::Infallible, str::FromStr};

type Input = Fishes;
type Output = usize;

register!(
    "input/day6.txt";
    (input: input!(first input!(parse Input))) -> Output {
        part1(&input.0);
        part2(&input.0);
    }
);

fn part1(items: &[usize]) -> Output {
    simulate(items, 80)
}

fn part2(items: &[usize]) -> Output {
    simulate(items, 256)
}

fn simulate(fish: &[usize], runs: usize) -> usize {
    let mut fish_per_day = [0_usize; 9];

    for day in fish {
        fish_per_day[*day] += 1;
    }

    for _ in 0..runs {
        // n fishies with day 0 spawn n new fishies with day 8
        // and re-enter the simulation as day 6 fishies ..
        fish_per_day.rotate_left(1);
        fish_per_day[6] += fish_per_day[8];
    }

    fish_per_day.iter().sum()
}

pub struct Fishes(Vec<usize>);

impl FromStr for Fishes {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(',').flat_map(str::parse::<usize>).collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"3,4,3,1,2"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 5934);
        assert_eq!(res2, 26984457539);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 377263);
        assert_eq!(res2, 1695929023803);
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
        b.iter(|| part1(&input.0));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input.0));
    }
}
