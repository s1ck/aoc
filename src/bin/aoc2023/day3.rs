use std::collections::HashSet;

use aoc::{lines, PuzzleInput};
use fxhash::FxHashMap;

type Input = Schematic;
type Output = usize;

register!(
    "input/day3.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(schematic: &Input) -> Output {
    solve(schematic).0
}

fn part2(schematic: &Input) -> Output {
    solve(schematic).1
}

fn solve(schematic: &Input) -> (usize, usize) {
    let rows = schematic.row_count() as isize;
    let cols = schematic.col_count() as isize;
    let deltas = [
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
    ];

    let mut sum = 0;

    let mut gears = FxHashMap::default();

    for (r, row) in schematic.rows.iter().enumerate() {
        let mut is_adjacent = false;

        let mut number = 0;
        let mut gears_local = HashSet::new();

        for (c, cell) in row.iter().enumerate() {
            if let Cell::Number(n) = cell {
                number = 10 * number + n;

                for (delta_r, delta_c) in deltas {
                    let r = r as isize + delta_r;
                    let c = c as isize + delta_c;
                    if r >= 0 && r < rows && c >= 0 && c < cols {
                        let neighbor = &schematic.rows[r as usize][c as usize];
                        if let Cell::Symbol(s) = neighbor {
                            if *s == '*' {
                                gears_local.insert((r, c));
                            }
                            is_adjacent = true;
                        }
                    }
                }
            } else if number > 0 {
                if is_adjacent {
                    sum += number;
                    gears_local.iter().copied().for_each(|gear| {
                        gears
                            .entry(gear)
                            .and_modify(|nums: &mut Vec<u32>| nums.push(number))
                            .or_insert_with(|| vec![number]);
                    });
                }
                number = 0;
                is_adjacent = false;
                gears_local.clear();
            }
        }
    }

    let ratio = gears
        .values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum::<u32>();

    (sum as usize, ratio as usize)
}

#[derive(Debug)]
pub struct Schematic {
    rows: Vec<Vec<Cell>>,
}

impl Schematic {
    fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn col_count(&self) -> usize {
        self.rows[0].len()
    }
}

#[derive(Debug)]
pub enum Cell {
    Number(u32),
    Symbol(char),
    Blank,
}

impl PuzzleInput for Schematic {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let lines = lines(input);

        let rows = lines
            .map(|line| {
                format!("{line}.")
                    .chars()
                    .map(|c| match c {
                        '.' => Cell::Blank,
                        d if d.is_ascii_digit() => Cell::Number(d.to_digit(10).unwrap()),
                        d => Cell::Symbol(d),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { rows }
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
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 4361);
        assert_eq!(res2, 467835);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 550934);
        assert_eq!(res2, 81997870);
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
