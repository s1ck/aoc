use std::str::FromStr;

use derive_more::{Deref, DerefMut};

type Input = Line;
type Output = usize;

register!(
    "input/day25.txt";
    (input: input!(parse Input)) -> Output {
        part1(input);
        part2();
    }
);

fn part1(mut lines: Vec<Input>) -> Output {
    let mut steps = 1;

    loop {
        if simulate(&mut lines) {
            return steps;
        }

        steps += 1;
    }
}

fn part2() -> Output {
    0
}

fn simulate(lines: &mut [Line]) -> bool {
    let mut done = true;

    let rows = lines.len();
    let cols = lines[0].len();

    // Move east cucumbers
    for row in lines.iter_mut() {
        let move_last = row[0] == Field::Empty && row[cols - 1] == Field::East;

        let mut col = 0;
        while col < cols - 1 {
            if row[col] == Field::East && row[col + 1] == Field::Empty {
                row[col] = Field::Empty;
                row[col + 1] = Field::East;
                done = false;
                col += 1;
            }
            col += 1;
        }

        if move_last {
            row[cols - 1] = Field::Empty;
            row[0] = Field::East;
            done = false;
        }
    }

    // Move south cucumbers
    for col in 0..cols {
        let move_last = lines[0][col] == Field::Empty && lines[rows - 1][col] == Field::South;

        let mut row = 0;
        while row < rows - 1 {
            if lines[row][col] == Field::South && lines[row + 1][col] == Field::Empty {
                lines[row][col] = Field::Empty;
                lines[row + 1][col] = Field::South;
                done = false;
                row += 1;
            }
            row += 1;
        }

        if move_last {
            lines[rows - 1][col] = Field::Empty;
            lines[0][col] = Field::South;
            done = false;
        }
    }

    done
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    East,
    South,
    Empty,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::East,
            'v' => Self::South,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

#[derive(Deref, DerefMut, Clone, Debug, PartialEq, Eq)]
pub struct Line(Vec<Field>);

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().map(Field::from).collect::<Vec<_>>()))
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
        v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 58);
        assert_eq!(res2, 0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 549);
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
        b.iter(|| part1(input.clone()));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2());
    }
}
