use std::{convert::Infallible, str::FromStr};

type Input = Command;
type Output = u32;

register!(
    "input/day2.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Command(Direction, u32);

impl FromStr for Command {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let input = line
            .split_once(' ')
            .map(|(dir, dist)| {
                let dist = dist.parse::<u32>().unwrap();

                let dir = match dir {
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    _ /* up */ => Direction::Up,
                };

                Self(dir, dist)
            })
            .unwrap();

        Ok(input)
    }
}

fn part1(commands: &[Input]) -> Output {
    let (x, y) = commands
        .iter()
        .fold((0, 0), |(x, y), Command(dir, dist)| match *dir {
            Direction::Forward => (x + dist, y),
            Direction::Down => (x, y + dist),
            Direction::Up => (x, y - dist),
        });
    x * y
}

fn part2(commands: &[Input]) -> Output {
    let (x, y, _) = commands
        .iter()
        .fold((0, 0, 0), |(x, y, aim), Command(dir, dist)| match *dir {
            Direction::Forward => (x + dist, y + (dist * aim), aim),
            Direction::Down => (x, y, aim + dist),
            Direction::Up => (x, y, aim - dist),
        });
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"forward 5
                       down 5
                       forward 8
                       up 3
                       down 8
                       forward 2"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 150);
        assert_eq!(res2, 900);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 2322630);
        assert_eq!(res2, 2105273490);
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
