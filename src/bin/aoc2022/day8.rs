use aoc::{lines, PuzzleInput};

type Input = Forest;
type Output = usize;

register!(
    "input/day8.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(forest: &Input) -> Output {
    let rows = &forest.rows;
    let cols = &forest.cols;
    let stride = forest.stride;
    let total = stride * stride;

    let mut visible = vec![0_u8; stride * stride];

    for i in (0..total).step_by(stride) {
        // '/' < '0' <=> 47_u8 < 48_u8
        let mut max_tree_left = b'/';
        for (j, tree) in rows.iter().enumerate().skip(i).take(stride) {
            if *tree > max_tree_left {
                visible[j] += 1;
                max_tree_left = u8::max(max_tree_left, *tree);
            }
        }
        let mut max_right = b'/';
        for j in (i..i + stride).rev() {
            if rows[j] > max_right {
                visible[j] += 1;
                max_right = u8::max(max_right, rows[j]);
            }
        }
    }
    for i in (0..total).step_by(stride) {
        let mut max_tree_top = b'/';
        for (j, tree) in cols.iter().enumerate().skip(i).take(stride) {
            if *tree > max_tree_top {
                let row_idx = stride * (j % stride) + (j / stride);
                visible[row_idx] += 1;
                max_tree_top = u8::max(max_tree_top, *tree);
            }
        }
        let mut max_tree_bottom = b'/';
        for j in (i..i + stride).rev() {
            if cols[j] > max_tree_bottom {
                let row_idx = stride * (j % stride) + (j / stride);
                visible[row_idx] += 1;
                max_tree_bottom = u8::max(max_tree_bottom, cols[j]);
            }
        }
    }

    visible.iter().filter(|v| **v > 0).count()
}

fn part2(forest: &Input) -> Output {
    let rows = &forest.rows;
    let cols = &forest.cols;
    let stride = forest.stride;
    let total = stride * stride;

    let mut views = vec![1_usize; stride * stride];

    for i in (0..total).step_by(stride) {
        for j in i..i + stride {
            // view to right
            let mut k = j;
            let mut view = 0;
            loop {
                if k + 1 == i + stride {
                    break;
                }
                if rows[k + 1] >= rows[j] {
                    view += 1;
                    break;
                }
                view += 1;
                k += 1;
            }
            views[j] *= view;
            // view to bottom
            let mut k = j;
            let mut view = 0;
            loop {
                if k + 1 == i + stride {
                    break;
                }
                if cols[k + 1] >= cols[j] {
                    view += 1;
                    break;
                }
                view += 1;
                k += 1;
            }
            views[stride * (j % stride) + (j / stride)] *= view;
        }
        for j in (i..i + stride).rev() {
            // view to left
            let mut k = j;
            let mut view = 0;
            loop {
                if k == i {
                    break;
                }
                if rows[k - 1] >= rows[j] {
                    view += 1;
                    break;
                }
                view += 1;
                k -= 1;
            }
            views[j] *= view;
            // view to top
            let mut k = j;
            let mut view = 0;
            loop {
                if k == i {
                    break;
                }
                if cols[k - 1] >= cols[j] {
                    view += 1;
                    break;
                }
                view += 1;
                k -= 1;
            }
            views[stride * (j % stride) + (j / stride)] *= view;
        }
    }

    views.into_iter().max().unwrap_or_default()
}

pub struct Forest {
    rows: Vec<u8>,
    cols: Vec<u8>,
    stride: usize,
}

impl PuzzleInput for Forest {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut lines = lines(input);
        let first = lines.next().unwrap();
        let stride = first.len();

        let mut rows = vec![0_u8; stride * stride];
        let mut cols = vec![0_u8; stride * stride];

        rows[0..stride].copy_from_slice(first.as_bytes());
        rows[stride..]
            .chunks_mut(stride)
            .zip(lines)
            .for_each(|(row, line)| row.copy_from_slice(line.as_bytes()));

        // transpose
        rows.chunks(stride).enumerate().for_each(|(i, row)| {
            row.iter()
                .enumerate()
                .for_each(|(j, e)| cols[j * stride + i] = *e);
        });

        Self { rows, cols, stride }
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
