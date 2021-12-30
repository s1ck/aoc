use std::{collections::HashMap, convert::Infallible, str::FromStr};

use aoc::{lines, PuzzleInput};

type Input = Bingo;
type Output = u32;

register!(
    "input/day4.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input.draws, &mut input.boards);
        part2(&input.draws, &mut input.boards);
    }
);

fn part1(draws: &[u32], boards: &mut [Board]) -> Output {
    for draw in draws {
        for board in boards.iter_mut() {
            board.mark(*draw);

            if board.bingo() {
                return *draw * board.sum_unmarked();
            }
        }
    }

    Output::MAX
}

fn part2(draws: &[u32], boards: &mut [Board]) -> Output {
    let mut boards = boards.iter_mut().collect::<Vec<_>>();

    for draw in draws {
        let (winners, loosers) = boards
            .into_iter()
            .map(|board| board.mark(*draw))
            .partition::<Vec<_>, _>(|board| board.bingo());

        if winners.len() == 1 && loosers.is_empty() {
            return *draw * winners[0].sum_unmarked();
        }

        boards = loosers;
    }

    Output::MAX
}

const SIZE: usize = 5;

#[derive(Debug, Clone, Copy)]
enum ItemState {
    Marked,
    Unmarked,
}

#[derive(Debug, Clone, Copy)]
struct Item {
    n: u32,
    state: ItemState,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            n: 0,
            state: ItemState::Unmarked,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    size: usize,
    rows: [[Item; SIZE]; SIZE],
    index: HashMap<u32, (usize, usize)>,
}

impl Board {
    fn mark(&mut self, n: u32) -> &mut Self {
        if let Some((i, j)) = self.index.get_mut(&n) {
            self.rows[*i][*j].state = ItemState::Marked;
        }
        self
    }

    fn bingo(&self) -> bool {
        for row in 0..self.size {
            if self.check_row(row) {
                return true;
            }
        }

        for col in 0..self.size {
            if self.check_col(col) {
                return true;
            }
        }

        false
    }

    fn check_row(&self, i: usize) -> bool {
        for col in 0..self.size {
            if matches!(self.rows[i][col].state, ItemState::Unmarked) {
                return false;
            }
        }
        true
    }

    fn check_col(&self, i: usize) -> bool {
        for row in 0..self.size {
            if matches!(self.rows[row][i].state, ItemState::Unmarked) {
                return false;
            }
        }
        true
    }

    fn sum_unmarked(&self) -> u32 {
        self.rows.iter().fold(0, |sum, row| {
            sum + row
                .iter()
                .filter_map(|item| match item.state {
                    ItemState::Unmarked => Some(item.n),
                    ItemState::Marked => None,
                })
                .sum::<u32>()
        })
    }
}

impl FromStr for Board {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = [[Item::default(); SIZE]; SIZE];
        let mut index = HashMap::new();

        lines(s).enumerate().for_each(|(i, row)| {
            row.split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .enumerate()
                .for_each(|(j, n)| {
                    index.insert(n, (i, j));
                    rows[i][j].n = n;
                });
        });

        Ok(Self {
            size: rows.len(),
            rows,
            index,
        })
    }
}

pub struct Bingo {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

impl PuzzleInput for Bingo {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut blocks = input.split("\n\n");

        let draws = blocks.next().unwrap();
        let draws = lines(draws)
            .flat_map(|s| s.split(','))
            .flat_map(str::parse::<u32>)
            .collect();

        let boards = blocks.map(|b| b.parse::<Board>().unwrap()).collect();

        Self { draws, boards }
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
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 4512);
        assert_eq!(res2, 1924);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 16674);
        assert_eq!(res2, 7075);
    }

    #[bench]
    fn bench_parsing(b: &mut Bencher) {
        let input = Solver::puzzle_input();
        b.bytes = input.len() as u64;
        b.iter(|| Solver::parse_input(input));
    }

    #[bench]
    fn bench_pt1(b: &mut Bencher) {
        let mut input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part1(&input.draws, &mut input.boards));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let mut input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input.draws, &mut input.boards));
    }
}
