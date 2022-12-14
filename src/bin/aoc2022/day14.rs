use aoc::{lines, PuzzleInput};

type Input = Cave;
type Output = usize;

register!(
    "input/day14.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(input.clone());
        part2(input);
    }
);

pub const W: usize = 600;
pub const H: usize = 200;

fn part1(mut cave: Input) -> Output {
    let mut i = 0;
    loop {
        match cave.enter_sand_man() {
            Some(sand) => i += sand,
            None => break i,
        }
    }
}

fn part2(mut cave: Input) -> Output {
    let max_y = cave.max_y() + 2;
    cave.fill((0, max_y), (W - 1, max_y), Cell::Rock);

    let mut i = 0;
    loop {
        match cave.enter_more_sand_man(max_y) {
            Some(sand) => i += sand,
            None => break i,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum Cell {
    Rock,
    Sand,
    Air,
}

#[derive(Debug, Clone)]
pub struct Cave {
    map: [[Cell; W]; H],
}

impl Cave {
    fn enter_sand_man(&mut self) -> Option<usize> {
        Self::drop_sand(&mut self.map, (500, 0), H)
    }

    fn enter_more_sand_man(&mut self, max_y: usize) -> Option<usize> {
        if self.map[0][500] == Cell::Air {
            Self::drop_sand(&mut self.map, (500, 0), max_y)
        } else {
            None
        }
    }

    // Returns how many sand particles have been added (if any)
    fn drop_sand(map: &mut [[Cell; W]; H], (x, y): (usize, usize), max_y: usize) -> Option<usize> {
        // Part 1
        if y == H - 1 {
            return None;
        }
        // Part 2
        // If we hit the horizontal boundaries, we know that the particles
        // will just fall down to the side without any rocks in their way.
        // This means, that we will add as many sand particles as we have
        // distance to the floor (max_y). We have to add one sand particle
        // to bubble this up eventually.
        //
        // I think the flaw here is that it won't work if we have rocks
        // placed at the x boundaries.
        if x == 0 || x == W - 1 {
            let diff_y = max_y - y;
            map[y][x] = Cell::Sand;
            return Some(diff_y);
        }
        // add sand
        match map[y + 1][x] {
            Cell::Air => Self::drop_sand(map, (x, y + 1), max_y),
            Cell::Rock | Cell::Sand => match (map[y + 1][x - 1], map[y + 1][x + 1]) {
                (Cell::Air, _) => Self::drop_sand(map, (x - 1, y + 1), max_y),
                (_, Cell::Air) => Self::drop_sand(map, (x + 1, y + 1), max_y),
                _ => {
                    map[y][x] = Cell::Sand;
                    Some(1)
                }
            },
        }
    }

    fn fill(&mut self, mut from: (usize, usize), mut to: (usize, usize), cell: Cell) {
        if from > to {
            std::mem::swap(&mut from, &mut to);
        }
        let (x0, y0) = from;
        let (x1, y1) = to;

        for x in x0..=x1 {
            self.map[y0][x] = cell;
        }
        for y in y0..=y1 {
            self.map[y][x0] = cell;
        }
    }

    fn max_y(&self) -> usize {
        H - 1
            - self
                .map
                .iter()
                .rev()
                .position(|row| row.iter().any(|c| *c == Cell::Rock || *c == Cell::Sand))
                .unwrap_or(0)
    }
}

impl PuzzleInput for Cave {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut cave = Self {
            map: [[Cell::Air; W]; H],
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
        b.iter(|| part1(input.clone()));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(input.clone()));
    }
}
