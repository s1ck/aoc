use std::{cmp::Ordering, str::FromStr};

type Input = Game;
type Output = u32;

register!(
    "input/day2.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items.iter().map(Game::play).sum()
}

fn part2(items: &[Input]) -> Output {
    items.iter().map(|game| game.transform().play()).sum()
}

pub struct Game(Move, Move, Outcome);

impl Game {
    fn play(&self) -> Output {
        let Self(first_move, second_move, _) = self;
        second_move.score()
            + second_move
                .partial_cmp(first_move)
                .map(|o| match o {
                    Ordering::Less => 0,
                    Ordering::Equal => 3,
                    Ordering::Greater => 6,
                })
                .unwrap_or_default()
    }

    fn transform(&self) -> Self {
        let Self(first_move, _, expected_outcome) = self;
        let expected_move = match (first_move, expected_outcome) {
            (Move::Rock, Outcome::Lose) | (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Lose) | (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Lose) | (Move::Rock, Outcome::Win) => Move::Paper,
            (_, Outcome::Draw) => self.0,
        };
        Self(self.0, expected_move, self.2)
    }
}

impl FromStr for Game {
    type Err = std::str::Utf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let left = chars.next().unwrap();
        let _ = chars.next(); // whitespace
        let right = chars.next().unwrap();

        Ok(Self(left.into(), right.into(), right.into()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn score(self) -> Output {
        self as Output
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, other) {
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => Ordering::Less,
            (Self::Scissors, Self::Paper)
            | (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock) => Ordering::Greater,
            (Self::Scissors, Self::Scissors)
            | (Self::Paper, Self::Paper)
            | (Self::Rock, Self::Rock) => Ordering::Equal,
        })
    }
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!("Unexpected char: {value}"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!("Unexpected char: {value}"),
        }
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
        A Y
        B X
        C Z
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 15);
        assert_eq!(res2, 12);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 11603);
        assert_eq!(res2, 12725);
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
