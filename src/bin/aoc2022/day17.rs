use aoc::PuzzleInput;
use fxhash::FxHashSet;

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

    for r in 0..2022 {
        let rock_type = RockType::from(r % 5);
        mov = chamber.add_rock(rock_type, &pattern.0, mov);
    }

    chamber.top()
}

fn part2(pattern: &Input) -> Output {
    0
}

const WIDTH: usize = 7;

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
            _ => panic!(),
        }
    }
}

pub struct Rock {
    points: FxHashSet<(usize, usize)>,
}

impl Rock {
    pub fn new(rock_type: RockType, y: usize) -> Self {
        let mut points = FxHashSet::with_capacity_and_hasher(10, Default::default());
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
        }
    }

    pub fn add_rock(&mut self, rock_type: RockType, moves: &[Move], mut mov: usize) -> usize {
        let top = self.top();
        let mut rock = Rock::new(rock_type, top + 4);

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
                break mov;
            }
        }
    }

    pub fn top(&self) -> usize {
        self.rocks.iter().map(|(_, y)| *y).max().unwrap_or(1)
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
        assert_eq!(res2, 0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 3197);
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
