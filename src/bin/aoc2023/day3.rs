use aoc::{lines, PuzzleInput};

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
    // dbg!(schematic);

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

    for (r, row) in schematic.rows.iter().enumerate() {
        let mut is_adjacent = false;

        let mut number = 0;
        for (c, cell) in row.iter().enumerate() {
            if let Cell::Number(n) = cell {
                number = 10 * number + n;

                for (delta_r, delta_c) in deltas {
                    let r = r as isize + delta_r;
                    let c = c as isize + delta_c;
                    if r >= 0 && r < rows && c >= 0 && c < cols {
                        let neighbor = &schematic.rows[r as usize][c as usize];
                        if matches!(neighbor, Cell::Symbol) {
                            is_adjacent = true;
                        }
                    }
                }
            } else if number > 0 {
                if is_adjacent {
                    sum += number;
                }
                number = 0;
                is_adjacent = false;
            }
        }
    }

    sum as usize
}

fn part2(items: &Input) -> Output {
    0
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
    Symbol,
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
                        d if d.is_digit(10) => Cell::Number(d.to_digit(10).unwrap()),
                        d => Cell::Symbol,
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
        assert_eq!(res2, 0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 550934);
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
