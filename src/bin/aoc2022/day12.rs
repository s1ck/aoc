use std::collections::VecDeque;

use aoc::{lines, PuzzleInput};
use fxhash::FxHashSet;

type Input = Map;
type Output = usize;

register!(
    "input/day12.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(map: &Input) -> Output {
    bfs(map, [map.start])
}

fn part2(map: &Input) -> Output {
    let sources = map.map.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(move |(j, b)| match b {
            b'a' => Some((i, j)),
            _ => None,
        })
    });
    bfs(map, sources)
}

fn bfs<I>(map: &Input, sources: I) -> Output
where
    I: IntoIterator<Item = (usize, usize)>,
{
    let mut visit = FxHashSet::default();
    let mut queue = VecDeque::new();

    let (start, end) = (map.start, map.end);
    let map = &map.map;
    let width = 0..map[0].len() as isize;
    let height = 0..map.len() as isize;
    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    sources.into_iter().for_each(|s| queue.push_back((0, s)));

    loop {
        let Some((dist, (row, col))) = queue.pop_front() else {
            break 0;
        };
        if visit.contains(&(row, col)) {
            continue;
        }
        if (row, col) == end {
            break dist;
        }
        visit.insert((row, col));

        for delta in deltas {
            let n_row = row as isize + delta.0;
            let n_col = col as isize + delta.1;
            if width.contains(&n_col)
                && height.contains(&n_row)
                && map[n_row as usize][n_col as usize] <= map[row][col] + 1
            {
                queue.push_back((dist + 1, (n_row as usize, n_col as usize)));
            }
        }
    }
}

#[derive(Debug)]
pub struct Map {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {}

impl PuzzleInput for Map {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let lines = lines(input);
        let mut start = (0, 0);
        let mut end = (0, 0);

        let map = lines
            .enumerate()
            .map(|(i, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(j, b)| match b {
                        b'S' => {
                            start = (i, j);
                            b'a'
                        }
                        b'E' => {
                            end = (i, j);
                            b'z' + 1
                        }
                        _ => *b,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { map, start, end }
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
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 31);
        assert_eq!(res2, 29);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 391);
        assert_eq!(res2, 386);
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
