use std::str::FromStr;

use fxhash::FxHashSet;

type Input = Card;
type Output = i32;

register!(
    "input/day4.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(cards: &[Input]) -> Output {
    cards.iter().map(Card::points).sum()
}

fn part2(items: &[Input]) -> Output {
    0
}

pub struct Card {
    win: FxHashSet<i32>,
    own: FxHashSet<i32>,
}

impl Card {
    fn points(&self) -> i32 {
        match self.win.intersection(&self.own).count() as u32 {
            0 => 0,
            n => 2_i32.pow(n - 1),
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (_, nums) = line.split_once(':').ok_or("no colon")?;
        let (win, own) = nums.split_once('|').ok_or("no pipe")?;
        let win = win
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().ok())
            .try_collect::<FxHashSet<_>>()
            .ok_or("parse error")?;
        let own = own
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().ok())
            .try_collect::<FxHashSet<_>>()
            .ok_or("parse_error")?;

        Ok(Self { win, own })
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
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 13);
        assert_eq!(res2, 0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 20855);
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
