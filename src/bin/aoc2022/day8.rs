type Input = Line;
type Output = usize;

register!(
    "input/day8.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    fn is_visible(row: usize, col: usize, trees: &[Input]) -> bool {
        let rows = trees.len();
        let cols = trees[0].0.len();
        let ctr = trees[row].0[col];

        let t = (0..row).all(|r| trees[r].0[col] < ctr);
        let b = (row + 1..rows).all(|r| trees[r].0[col] < ctr);
        let l = (0..col).all(|c| trees[row].0[c] < ctr);
        let r = (col + 1..cols).all(|c| trees[row].0[c] < ctr);

        t || b || l || r
    }

    let rows = items.len();
    let cols = items[0].0.len();
    let mut cnt = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if is_visible(row, col, items) {
                cnt += 1;
            }
        }
    }

    cnt + 2 * rows + 2 * (cols - 2)
}

fn part2(items: &[Input]) -> Output {
    fn scenic_score(row: usize, col: usize, trees: &[Input]) -> Output {
        fn count((cnt, done): (Output, bool), n: u8, ctr: u8) -> (Output, bool) {
            if !done && (n >= ctr) {
                (cnt + 1, true)
            } else if !done {
                (cnt + 1, done)
            } else {
                (cnt, true)
            }
        }
        let rows = trees.len();
        let cols = trees[0].0.len();
        let ctr = trees[row].0[col];

        let t = (0..row)
            .rev()
            .fold((0, false), |acc, r| count(acc, trees[r].0[col], ctr))
            .0;
        let b = (row + 1..rows)
            .fold((0, false), |acc, r| count(acc, trees[r].0[col], ctr))
            .0;
        let l = (0..col)
            .rev()
            .fold((0, false), |acc, c| count(acc, trees[row].0[c], ctr))
            .0;
        let r = (col + 1..cols)
            .fold((0, false), |acc, c| count(acc, trees[row].0[c], ctr))
            .0;

        t * b * l * r
    }

    let rows = items.len();
    let cols = items[0].0.len();
    let mut score = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            score = Output::max(score, scenic_score(row, col, items));
        }
    }

    score
}

pub struct Line(Vec<u8>);

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
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
        30373
        25512
        65332
        33549
        35390
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 21);
        assert_eq!(res2, 8);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1676);
        assert_eq!(res2, 313200);
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
