use std::fmt::Display;

use aoc::{lines, PuzzleInput};

type Input = Field;
type Output = usize;

register!(
    "input/day3.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(
    Input {
        values: field,
        height,
        width,
    }: &Input,
) -> Output {
    let mut count = 0;

    for (row, line) in field.iter().enumerate().take(*height).skip(1) {
        let idx = (row * 3) % width;
        let mask = 1 << (31 - idx);
        if line & mask == mask {
            count += 1;
        }
    }

    count
}

fn part2(
    Input {
        values: field,
        height,
        width,
    }: &Input,
) -> Output {
    let mut slope_1 = 0;
    let mut slope_2 = 0;
    let mut slope_3 = 0;
    let mut slope_4 = 0;
    let mut slope_5 = 0;

    for (row, line) in field.iter().enumerate().take(*height).skip(1) {
        // slope 1: right 1, down 1
        let idx = row % width;
        let mask = 1 << (31 - idx);
        if line & mask == mask {
            slope_1 += 1;
        }

        // slope 2: right 3, down 1
        let idx = (row * 3) % width;
        let mask = 1 << (31 - idx);
        if line & mask == mask {
            slope_2 += 1;
        }

        // slope 3: right 5, down 1
        let idx = (row * 5) % width;
        let mask = 1 << (31 - idx);
        if line & mask == mask {
            slope_3 += 1;
        }

        // slope 4: right 7, down 1
        let idx = (row * 7) % width;
        let mask = 1 << (31 - idx);
        if line & mask == mask {
            slope_4 += 1;
        }

        // slope 5: right 1, down 2
        if row % 2 == 0 {
            let idx = (row / 2) % width;
            let mask = 1 << (31 - idx);
            if line & mask == mask {
                slope_5 += 1;
            }
        }
    }

    slope_1 * slope_2 * slope_3 * slope_4 * slope_5
}

const LINE_COUNT: usize = 323;

pub struct Field {
    values: [u32; LINE_COUNT],
    height: usize,
    width: usize,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.values[0..self.height]
            .iter()
            .try_for_each(|line| writeln!(f, "{line:032b}"))
    }
}

impl PuzzleInput for Field {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut values = [0_u32; LINE_COUNT];
        let lines = lines(input);
        let mut width = 0;
        let mut height = 0;

        for (i, line) in lines.enumerate() {
            width = line.len();
            height = i;

            let mut enc = 0_u32;
            for (j, &b) in line.as_bytes().iter().enumerate() {
                enc |= u32::from(b == b'#') << (31 - j);
            }
            values[i] = enc;
        }

        Self {
            values,
            height: height + 1,
            width,
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
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 7);
        assert_eq!(res2, 336);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 289);
        assert_eq!(res2, 5522401584);
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
