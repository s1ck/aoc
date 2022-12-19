use aoc::PuzzleInput;
use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};
use tap::Tap;

type Input = Pattern;
type Output = usize;

register!(
    "input/day17.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(pattern: &Input) -> Output {
    let mut chamber = Chamber::new();
    let mut mov = 0;

    for it in 0..2022 {
        let rock_type = RockType::from(it % 5);
        mov = chamber.add_rock(rock_type, &pattern.0, mov);
    }

    chamber.top()
}

fn part2(pattern: &Input) -> Output {
    let mut chamber = Chamber::new();
    let mut mov = 0;
    let mut cache = FxHashMap::default();
    let mut extra = 0;

    let iterations = 1_000_000_000_000;
    let mut it = 0;

    while it < iterations {
        let rock_idx = it % 5;
        let rock_type = RockType::from(rock_idx);
        mov = chamber.add_rock(rock_type, &pattern.0, mov);
        // Check if there is a repetition in the chamber.
        // If we detect a repetition for the same shape
        // and the same movement index, we can use that
        // to skip iterations.
        cache
            .entry((rock_idx, mov, chamber.top_n(42)))
            .and_modify(|(prev_i, prev_top)| {
                // The increase in the height of the chamber.
                let dt = chamber.top - *prev_top;
                // The number of iterations it took to repeat the pattern.
                let di = it - *prev_i;
                // The number of times we can repeat the pattern before
                // we hit the total number of iterations.
                let reps = (iterations - it) / di;
                // Skip iterations.
                it += reps * di;
                // Adjust the height of the chamber.
                extra += reps * dt;
            })
            .or_insert((it, chamber.top));

        it += 1;
    }

    chamber.top() + extra
}

const WIDTH: usize = 7;

#[derive(Clone, Copy)]
pub enum RockType {
    HLine,
    Cross,
    LShap,
    VLine,
    Block,
}

impl From<usize> for RockType {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::HLine,
            1 => Self::Cross,
            2 => Self::LShap,
            3 => Self::VLine,
            4 => Self::Block,
            _ => unreachable!(),
        }
    }
}

pub struct Rock<'a> {
    points: &'a mut FxHashSet<(usize, usize)>,
}

impl<'a> Rock<'a> {
    pub fn new(rock_type: RockType, points: &'a mut FxHashSet<(usize, usize)>, y: usize) -> Self {
        match rock_type {
            RockType::HLine => {
                points.insert((2, y));
                points.insert((3, y));
                points.insert((4, y));
                points.insert((5, y));
            }
            RockType::Cross => {
                points.insert((2, y + 1));
                points.insert((3, y));
                points.insert((3, y + 2));
                points.insert((4, y + 1));
            }
            RockType::LShap => {
                points.insert((2, y));
                points.insert((3, y));
                points.insert((4, y));
                points.insert((4, y + 1));
                points.insert((4, y + 2));
            }
            RockType::VLine => {
                points.insert((2, y));
                points.insert((2, y + 1));
                points.insert((2, y + 2));
                points.insert((2, y + 3));
            }
            RockType::Block => {
                points.insert((2, y));
                points.insert((3, y));
                points.insert((2, y + 1));
                points.insert((3, y + 1));
            }
        };

        Self { points }
    }

    pub fn move_left(&mut self, buffer: &mut Vec<(usize, usize)>) {
        if self.points.iter().any(|p| p.0 == 0) {
            return;
        }

        assert!(buffer.is_empty());

        self.points
            .drain()
            .map(|(x, y)| (x - 1, y))
            .collect_into(buffer);

        buffer.drain(0..).for_each(|p| {
            self.points.insert(p);
        });
    }

    pub fn move_right(&mut self, buffer: &mut Vec<(usize, usize)>) {
        if self.points.iter().any(|p| p.0 == WIDTH - 1) {
            return;
        }

        assert!(buffer.is_empty());

        self.points
            .drain()
            .map(|(x, y)| (x + 1, y))
            .collect_into(buffer);

        buffer.drain(0..).for_each(|p| {
            self.points.insert(p);
        });
    }

    pub fn move_down(&mut self, buffer: &mut Vec<(usize, usize)>) {
        assert!(buffer.is_empty());

        self.points
            .drain()
            .map(|(x, y)| (x, y - 1))
            .collect_into(buffer);

        buffer.drain(0..).for_each(|p| {
            self.points.insert(p);
        });
    }

    pub fn move_up(&mut self, buffer: &mut Vec<(usize, usize)>) {
        assert!(buffer.is_empty());

        self.points
            .drain()
            .map(|(x, y)| (x, y + 1))
            .collect_into(buffer);

        buffer.drain(0..).for_each(|p| {
            self.points.insert(p);
        });
    }

    pub fn overlaps_with(&self, points: &FxHashSet<(usize, usize)>) -> bool {
        self.points.intersection(points).next().is_some()
    }
}

pub struct Chamber {
    rocks: FxHashSet<(usize, usize)>,
    buffer: Vec<(usize, usize)>,
    rock: FxHashSet<(usize, usize)>,
    top: usize,
}

impl Chamber {
    pub fn new() -> Self {
        let mut rocks = FxHashSet::default();
        for x in 0..WIDTH {
            rocks.insert((x, 0));
        }
        Self {
            rocks,
            buffer: Vec::with_capacity(5),
            rock: FxHashSet::with_capacity_and_hasher(5, FxBuildHasher::default()),
            top: 0,
        }
    }

    pub fn add_rock(&mut self, rock_type: RockType, moves: &[Move], mut mov: usize) -> usize {
        let mut rock = Rock::new(rock_type, &mut self.rock, self.top + 4);

        loop {
            match moves[mov] {
                Move::Left => {
                    rock.move_left(&mut self.buffer);
                    if rock.overlaps_with(&self.rocks) {
                        rock.move_right(&mut self.buffer);
                    }
                }
                Move::Right => {
                    rock.move_right(&mut self.buffer);
                    if rock.overlaps_with(&self.rocks) {
                        rock.move_left(&mut self.buffer);
                    }
                }
            }
            mov = (mov + 1) % moves.len();

            rock.move_down(&mut self.buffer);

            if rock.overlaps_with(&self.rocks) {
                rock.move_up(&mut self.buffer);
                rock.points.drain().for_each(|p| {
                    self.rocks.insert(p);
                });
                self.top = self.top();
                break mov;
            }
        }
    }

    pub fn top(&self) -> usize {
        self.rocks.iter().map(|(_, y)| *y).max().unwrap_or(1)
    }

    pub fn top_n(&self, n: usize) -> Vec<(usize, usize)> {
        let top = self.top();
        self.rocks
            .iter()
            .filter(|(_, y)| top.abs_diff(*y) < n)
            .copied()
            .map(|(x, y)| (x, top - y))
            .collect::<Vec<_>>()
            .tap_mut(|v| v.sort_unstable())
    }
}

#[derive(Debug)]
pub struct Pattern(Vec<Move>);

#[derive(Debug)]
pub enum Move {
    Left,
    Right,
}

impl PuzzleInput for Pattern {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        Self(
            input
                .trim()
                .bytes()
                .map(|b| match b {
                    b'<' => Move::Left,
                    b'>' => Move::Right,
                    _ => panic!("unexpected char {b}"),
                })
                .collect::<Vec<_>>(),
        )
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
        >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 3068);
        assert_eq!(res2, 1514285714288);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 3197);
        assert_eq!(res2, 1568513119571);
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
