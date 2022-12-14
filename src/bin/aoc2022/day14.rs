use std::{
    cell::RefCell,
    fmt::{Display, Write},
    time::Duration,
};

use aoc::{lines, PuzzleInput};

type Input = Cave;
type Output = usize;

register!(
    "input/day14.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input.clone());
        part2(&input);
    }
);

pub const W: usize = 600;
pub const H: usize = 200;

fn part1(cave: &Input) -> Output {
    let mut i = 0;
    loop {
        if !cave.enter_sand_man() {
            break i;
        }
        i += 1;
    }
}

fn part2(cave: &Input) -> Output {
    let max_y = cave.bounding_box().1 .1 + 2;
    let from = (0, max_y);
    let to = (W - 1, max_y);
    cave.fill(from, to, Cell::Rock);
    let mut i = 0;
    loop {
        match cave.enter_more_sand_man(max_y) {
            Some(sand) => i += sand,
            None => break i,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Cell {
    Rock,
    Sand,
    Air,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Cell::Rock => "#",
            Cell::Sand => "o",
            Cell::Air => ".",
        })
    }
}

#[derive(Debug, Clone)]
pub struct Cave {
    map: RefCell<[[Cell; W]; H]>,
}

impl Cave {
    // Returns 'true' if the sand sticked, false otherwise
    fn enter_sand_man(&self) -> bool {
        Self::add_sand(&mut self.map.borrow_mut(), (500, 0)).1 != H - 1
    }

    fn add_sand(map: &mut [[Cell; W]; H], from @ (x, y): (usize, usize)) -> (usize, usize) {
        if y == H - 1 {
            return from;
        }
        match map[y + 1][x] {
            Cell::Air => Self::add_sand(map, (x, y + 1)),
            Cell::Rock | Cell::Sand => {
                return match (map[y + 1][x - 1], map[y + 1][x + 1]) {
                    (Cell::Air, _) => Self::add_sand(map, (x - 1, y + 1)),
                    (_, Cell::Air) => Self::add_sand(map, (x + 1, y + 1)),
                    _ => {
                        map[y][x] = Cell::Sand;
                        (x, y)
                    }
                }
            }
        }
    }

    fn enter_more_sand_man(&self, max_y: usize) -> Option<usize> {
        if self.map.borrow()[0][500] != Cell::Air {
            None
        } else {
            Self::add_sand_2(&mut self.map.borrow_mut(), (500, 0), max_y)
        }
    }

    fn add_sand_2(
        map: &mut [[Cell; W]; H],
        from @ (x, y): (usize, usize),
        max_y: usize,
    ) -> Option<usize> {
        if x == 0 || x == W - 1 {
            let diff_y = max_y - y;
            map[y][x] = Cell::Sand;
            return Some(diff_y);
        }
        match map[y + 1][x] {
            Cell::Air => Self::add_sand_2(map, (x, y + 1), max_y),
            Cell::Rock | Cell::Sand => {
                return match (map[y + 1][x - 1], map[y + 1][x + 1]) {
                    (Cell::Air, _) => Self::add_sand_2(map, (x - 1, y + 1), max_y),
                    (_, Cell::Air) => Self::add_sand_2(map, (x + 1, y + 1), max_y),
                    _ => {
                        map[y][x] = Cell::Sand;
                        Some(1)
                    }
                }
            }
        }
    }

    fn fill(&self, mut from: (usize, usize), mut to: (usize, usize), cell: Cell) {
        if from > to {
            std::mem::swap(&mut from, &mut to);
        }
        let (x0, y0) = from;
        let (x1, y1) = to;

        for x in x0..=x1 {
            self.map.borrow_mut()[y0][x] = cell;
        }
        for y in y0..=y1 {
            self.map.borrow_mut()[y][x0] = cell;
        }
    }

    // For visual debugging
    fn bounding_box(&self) -> ((usize, usize), (usize, usize)) {
        let min_y = self
            .map
            .borrow()
            .iter()
            .position(|row| row.iter().any(|c| *c == Cell::Rock || *c == Cell::Sand))
            .unwrap_or(0);

        let max_y = H
            - 1
            - self
                .map
                .borrow()
                .iter()
                .rev()
                .position(|row| row.iter().any(|c| *c == Cell::Rock || *c == Cell::Sand))
                .unwrap_or(0);

        let min_x = self
            .map
            .borrow()
            .iter()
            .filter_map(|row| {
                row.iter()
                    .position(|c| *c == Cell::Rock || *c == Cell::Sand)
            })
            .min()
            .unwrap_or(0);

        let max_x = W
            - 1
            - self
                .map
                .borrow()
                .iter()
                .filter_map(|row| {
                    row.iter()
                        .rev()
                        .position(|c| *c == Cell::Rock || *c == Cell::Sand)
                })
                .min()
                .unwrap_or(0);

        ((min_x, min_y), (max_x, max_y))
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ((min_x, min_y), (max_x, max_y)) = self.bounding_box();

        for y in min_y..=max_y {
            f.write_str(
                &self.map.borrow()[y][min_x..=max_x]
                    .iter()
                    .map(|c| format!("{c}"))
                    .collect::<String>(),
            )?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl PuzzleInput for Cave {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let cave = Cave {
            map: RefCell::new([[Cell::Air; W]; H]),
        };

        lines(input).for_each(|line| {
            line.split(" -> ")
                .map(|pair| pair.split_once(',').unwrap())
                .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                .reduce(|prev, next| {
                    cave.fill(prev, next, Cell::Rock);
                    next
                });
        });

        cave
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
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 24);
        assert_eq!(res2, 93);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1003);
        assert_eq!(res2, 25771);
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
