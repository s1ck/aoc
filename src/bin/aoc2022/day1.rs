use aoc::{lines, PuzzleInput};

type Input = Calories;
type Output = u32;

register!(
    "input/day1.txt";
    (input: input!(blocks Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items.iter().map(|c| c.0).max().unwrap_or_default()
}

fn part2(items: &[Input]) -> Output {
    let (a, b, c) = items.iter().map(|c| c.0).fold((0, 0, 0), |acc, n| {
        if n >= acc.0 {
            (n, acc.0, acc.1)
        } else if n >= acc.1 {
            (acc.0, n, acc.1)
        } else if n >= acc.2 {
            (acc.0, acc.1, n)
        } else {
            acc
        }
    });

    a + b + c
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Calories(u32);

impl PuzzleInput for Calories {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let lines = lines(input);
        Self(lines.map(|c| c.parse::<u32>().unwrap_or_default()).sum())
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
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 24000);
        assert_eq!(res2, 45000);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 71023);
        assert_eq!(res2, 206289);
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
