use std::{convert::Infallible, str::FromStr};

type Input = Crabs;
type Output = i32;

register!(
    "input/day7.txt";
    (input: input!(first input!(parse Input))) -> Output {
        part1(&input.0);
        part2(&input.0);
    }
);

fn part1(positions: &[i32]) -> Output {
    let mut positions = positions.to_vec();
    positions.sort_unstable();
    let median = positions[positions.len() / 2];
    positions.into_iter().map(|pos| (pos - median).abs()).sum()
}

fn part2(positions: &[i32]) -> Output {
    let max_position = positions.iter().max().unwrap();

    (0..*max_position)
        .map(|x| {
            positions
                .iter()
                .map(|position| {
                    let diff = (x - position).abs();
                    diff * (diff + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

pub struct Crabs(Vec<i32>);

impl FromStr for Crabs {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(',').flat_map(str::parse::<i32>).collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"16,1,2,0,4,2,7,1,2,14"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 37);
        assert_eq!(res2, 168);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 326132);
        assert_eq!(res2, 88612508);
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
