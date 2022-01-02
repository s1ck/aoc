use aoc::PuzzleInput;
use fxhash::{FxBuildHasher, FxHashSet};

type Input = TrenchMap;
type Output = usize;

register!(
    "input/day20.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input.map, &input.algo);
        part2(&input.map, &input.algo);
    }
);

fn part1(map: &FxHashSet<(isize, isize)>, algo: &[bool]) -> Output {
    simulate(map, algo, 2)
}

fn part2(map: &FxHashSet<(isize, isize)>, algo: &[bool]) -> Output {
    simulate(map, algo, 50)
}

fn simulate(map: &FxHashSet<(isize, isize)>, algo: &[bool], steps: usize) -> usize {
    assert!(steps >= 1);
    let mut draw_infinite = false;
    // algo[0] encodes if we need to draw '#' for points that represent infinity.
    // If this is the case, we need to toggle this with each step to remove the
    // '#'s drawn in the previous step.
    let toggle = algo[0];

    let mut next = step(map, algo, draw_infinite);

    for _ in 1..steps {
        if toggle {
            draw_infinite = !draw_infinite;
        }
        next = step(&next, algo, draw_infinite);
    }

    next.len()
}

fn step(
    map: &FxHashSet<(isize, isize)>,
    algo: &[bool],
    draw_infinite: bool,
) -> FxHashSet<(isize, isize)> {
    let r_min = *map.iter().map(|(r, _)| r).min().unwrap();
    let r_max = *map.iter().map(|(r, _)| r).max().unwrap();
    let c_min = *map.iter().map(|(_, c)| c).min().unwrap();
    let c_max = *map.iter().map(|(_, c)| c).max().unwrap();

    let mut res = FxHashSet::with_capacity_and_hasher(map.capacity(), FxBuildHasher::default());

    for r in (r_min - 2)..(r_max + 2) {
        for c in (c_min - 2)..(c_max + 2) {
            let mut idx: u32 = 0;
            let mut bit: u32 = 8;

            for r_delta in [-1, 0, 1] {
                for c_delta in [-1, 0, 1] {
                    let r_nbr = r + r_delta;
                    let c_nbr = c + c_delta;

                    let pick = if r_nbr < r_min || r_nbr > r_max || c_nbr < c_min || c_nbr > c_max {
                        draw_infinite
                    } else {
                        map.contains(&(r_nbr, c_nbr))
                    };

                    if pick {
                        idx += 2_u32.pow(bit);
                    }
                    bit = bit.wrapping_sub(1);
                }
            }

            if algo[idx as usize] {
                res.insert((r, c));
            }
        }
    }

    res
}

pub struct TrenchMap {
    map: FxHashSet<(isize, isize)>,
    algo: Vec<bool>,
}

impl PuzzleInput for TrenchMap {
    type Out = Self;

    fn from_input(lines: &str) -> Self::Out {
        let mut lines = aoc::lines(lines);

        let algo = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>();

        assert_eq!(algo.len(), 512);

        let map = lines
            .enumerate()
            .flat_map(|(row, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(col, bit)| {
                        if bit == '#' {
                            Some((row as isize, col as isize))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<FxHashSet<_>>();

        Self { map, algo }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
        
        #..#.
        #....
        ##..#
        ..#..
        ..###
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 35);
        assert_eq!(res2, 3351);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 5359);
        assert_eq!(res2, 12333);
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
        b.iter(|| part1(&input.map, &input.algo));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input.map, &input.algo));
    }
}
