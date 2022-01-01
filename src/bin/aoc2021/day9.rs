use std::{
    collections::{HashSet, VecDeque},
    convert::Infallible,
    str::FromStr,
};

use derive_more::Deref;

type Input = Row;
type Output = u32;

register!(
    "input/day9.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input).0;
        part2(&input);
    }
);

fn part1(field: &[Row]) -> (Output, Vec<(usize, usize)>) {
    let h = field.len();
    let w = field[0].len();

    let mut sum = 0_u32;
    let mut depths = vec![];

    for i in 0..h {
        for j in 0..w {
            let ctr = field[i][j];
            let top = if i == 0 { ctr + 1 } else { field[i - 1][j] };
            let right = if j == w - 1 { ctr + 1 } else { field[i][j + 1] };
            let bottom = if i == h - 1 { ctr + 1 } else { field[i + 1][j] };
            let left = if j == 0 { ctr + 1 } else { field[i][j - 1] };

            if top > ctr && right > ctr && bottom > ctr && left > ctr {
                depths.push((i, j));
                sum += u32::from(ctr) + 1;
            }
        }
    }

    (sum, depths)
}

fn part2(field: &[Row]) -> Output {
    fn bfs(field: &[Row], root: (usize, usize)) -> u32 {
        let mut size = 0;
        let mut queue = VecDeque::new();
        queue.push_back(root);

        let h = field.len();
        let w = field[0].len();

        let mut seen = HashSet::new();

        while let Some((i, j)) = queue.pop_front() {
            let ctr = field[i][j];
            if ctr == 9 {
                continue;
            }
            size += 1;

            let top = if i == 0 { ctr + 1 } else { field[i - 1][j] };
            let right = if j == w - 1 { ctr + 1 } else { field[i][j + 1] };
            let bottom = if i == h - 1 { ctr + 1 } else { field[i + 1][j] };
            let left = if j == 0 { ctr + 1 } else { field[i][j - 1] };

            if i > 0 && top > ctr {
                let p = (i - 1, j);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push_back(p);
                }
            }
            if j < w - 1 && right > ctr {
                let p = (i, j + 1);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push_back(p);
                }
            }
            if i < h - 1 && bottom > ctr {
                let p = (i + 1, j);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push_back(p);
                }
            }
            if j > 0 && left > ctr {
                let p = (i, j - 1);
                if !seen.contains(&p) {
                    seen.insert(p);
                    queue.push_back(p);
                }
            }
        }
        size
    }

    let (_, depths) = part1(field);
    let mut sizes = depths
        .iter()
        .map(|depth| bfs(field, *depth))
        .collect::<Vec<_>>();
    sizes.sort_by(|a, b| a.cmp(b).reverse());

    sizes.iter().take(3).product()
}

#[derive(Deref)]
pub struct Row(Vec<u8>);

impl FromStr for Row {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().map(|c| c as u8 - b'0').collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"2199943210
                       3987894921
                       9856789892
                       8767896789
                       9899965678"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 15);
        assert_eq!(res2, 1134);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 594);
        assert_eq!(res2, 858494);
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
