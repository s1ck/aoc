use aoc::{lines, PuzzleInput};
use derive_more::{Deref, DerefMut};

const SIZE: usize = 10;

type Input = Field<SIZE>;
type Output = usize;

register!(
    "input/day11.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&mut input.clone());
        part2(&mut input);
    }
);

fn part1<const N: usize>(field: &mut Field<N>) -> Output {
    let mut flashes = 0;

    for _ in 0..N * N {
        let mut queue = field.increment();

        while let Some(next) = queue.pop() {
            flashes += 1;
            field.flash(next, &mut queue);
        }
    }

    flashes
}

fn part2<const N: usize>(field: &mut Field<N>) -> Output {
    let mut step = 1;

    loop {
        let mut flashes = 0;
        let mut queue = field.increment();

        while let Some(next) = queue.pop() {
            flashes += 1;
            field.flash(next, &mut queue);
        }

        if flashes == N * N {
            return step;
        }

        step += 1;
    }
}

#[derive(Deref, DerefMut, Clone)]
pub struct Field<const N: usize>([[u8; N]; N]);

impl<const N: usize> Field<N> {
    fn increment(&mut self) -> Vec<(usize, usize)> {
        let mut queue = vec![];

        for i in 0..self.len() {
            for j in 0..self.len() {
                self[i][j] += 1;

                if self[i][j] > 9 {
                    queue.push((i, j));
                }
            }
        }

        queue
    }

    fn flash(&mut self, (row, col): (usize, usize), queue: &mut Vec<(usize, usize)>) {
        self[row][col] = 0;

        for (n_row, n_col) in [
            (row.wrapping_sub(1), col.wrapping_sub(1)),
            (row.wrapping_sub(1), col),
            (row.wrapping_sub(1), col + 1),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
            (row + 1, col.wrapping_sub(1)),
            (row + 1, col),
            (row + 1, col + 1),
        ] {
            if let Some(n) = self.get_mut(n_row).and_then(|row| row.get_mut(n_col)) {
                if *n > 0 {
                    *n += 1;

                    if *n > 9 && !queue.contains(&(n_row, n_col)) {
                        queue.push((n_row, n_col));
                    }
                }
            }
        }
    }
}

impl PuzzleInput for Field<SIZE> {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut res = [[0; SIZE]; SIZE];

        lines(input).enumerate().for_each(|(row, line)| {
            line.trim()
                .bytes()
                .enumerate()
                .for_each(|(col, b)| res[row][col] = b - b'0');
        });

        Self(res)
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
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 1656);
        assert_eq!(res2, 195);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1697);
        assert_eq!(res2, 344);
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
        b.iter(|| part1(&mut input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let mut input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&mut input));
    }
}
