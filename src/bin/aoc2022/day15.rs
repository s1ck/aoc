use std::{
    fmt::Display,
    ops::{ControlFlow, RangeInclusive},
};

use aoc::{lines, PuzzleInput};
use atoi::FromRadix10Signed;
use fxhash::FxHashSet;
use tap::Tap;

type Input = Map;
type Output = usize;

register!(
    "input/day15.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(map: &Input) -> Output {
    if map.is_example() {
        map.coverage(10)
    } else {
        map.coverage(2_000_000)
    }
}

fn part2(map: &Input) -> Output {
    if map.is_example() {
        map.tuning_frequency(20)
    } else {
        map.tuning_frequency(4_000_000)
    }
}

pub struct Result(usize, usize);

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Sensor {
    x: i32,
    y: i32,
    range: i32,
}

impl Sensor {
    fn coverage_at_row(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let dy = self.y.abs_diff(y) as i32;
        if dy > self.range {
            None
        } else {
            let dx = self.range.abs_diff(dy) as i32;
            Some(self.x - dx..=self.x + dx)
        }
    }
}

pub struct Map {
    sensors: Vec<Sensor>,
    beacons: FxHashSet<(i32, i32)>,
}

impl Map {
    fn is_example(&self) -> bool {
        self.sensors.len() == 14
    }

    fn coverage(&self, y: i32) -> usize {
        // Get the covered range for each sensor at row y.
        let mut ranges = self
            .sensors
            .iter()
            .filter_map(|s| s.coverage_at_row(y))
            .collect::<Vec<_>>();

        if ranges.is_empty() {
            return 0;
        }

        // Order ranges by (start, end).
        ranges.sort_unstable_by_key(|r| (*r.start(), *r.end()));
        // The ranges are overlapping.
        let range = ranges[0].start()..=ranges[ranges.len() - 1].end();

        let beacons = self
            .beacons
            .iter()
            .filter(|(b_x, b_y)| *b_y == y && range.contains(&b_x))
            .count();

        (*range.end() - *range.start()) as usize + 1 - beacons
    }

    fn tuning_frequency(&self, x_y_max: i32) -> usize {
        for y in (0..x_y_max).rev() {
            let ranges = self
                .sensors
                .iter()
                .filter_map(|s| s.coverage_at_row(y))
                .filter(|r| *r.start() <= x_y_max && *r.end() >= 0)
                .collect::<Vec<_>>()
                .tap_mut(|v| v.sort_unstable_by_key(|r| (*r.start(), *r.end())));

            let mut ranges = ranges.into_iter();
            let first = ranges.next().unwrap();

            let res = ranges.try_fold(*first.end(), |end, range| {
                let gap = end + 1;
                if gap < *range.start() && gap > 0 && gap <= x_y_max {
                    ControlFlow::Break(Ok(gap))
                } else if end > x_y_max {
                    ControlFlow::Break(Err(end))
                } else {
                    ControlFlow::Continue(end.max(*range.end()))
                }
            });

            if let Some(Ok(x)) = res.break_value() {
                return x as usize * 4_000_000 + y as usize;
            }
        }

        usize::MAX
    }
}

fn manhattan((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> i32 {
    (x0.abs_diff(x1) + y0.abs_diff(y1)) as i32
}

impl PuzzleInput for Map {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut beacons = FxHashSet::default();

        let sensors = lines(input)
            .map(|line| {
                let line = line.as_bytes();
                let mut offset = 12;
                let (s_x, used) = i32::from_radix_10_signed(&line[offset..]);
                offset += used + 4;
                let (s_y, used) = i32::from_radix_10_signed(&line[offset..]);
                offset += used + 25;
                let (b_x, used) = i32::from_radix_10_signed(&line[offset..]);
                offset += used + 4;
                let (b_y, _) = i32::from_radix_10_signed(&line[offset..]);

                let range = manhattan((s_x, s_y), (b_x, b_y));

                beacons.insert((b_x, b_y));
                Sensor {
                    x: s_x,
                    y: s_y,
                    range,
                }
            })
            .collect::<Vec<_>>();

        Self { sensors, beacons }
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
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 26);
        assert_eq!(res2, 56000011);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 4961647);
        assert_eq!(res2, 12274327017867);
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
