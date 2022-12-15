use std::fmt::Display;

use aoc::{lines, PuzzleInput};
use atoi::FromRadix10Signed;
use fxhash::FxHashSet;

type Input = Map;
type Output = Result;

register!(
    "input/day15.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(map: &Input) -> Output {
    Result(map.coverage(10), map.coverage(2_000_000))
}

fn part2(map: &Input) -> Output {
    Result(map.tuning_frequency(20), map.tuning_frequency(4_000_000))
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
    fn covers(&self, x: i32, y: i32) -> bool {
        manhattan((self.x, self.y), (x, y)) <= self.range
    }
}

pub struct Map {
    sensors: Vec<Sensor>,
    beacons: FxHashSet<(i32, i32)>,
    x_min: i32,
    x_max: i32,
}

impl Map {
    fn coverage(&self, y: i32) -> usize {
        let mut coverage = 0;
        for x in self.x_min..=self.x_max {
            if self.beacons.contains(&(x, y)) {
                continue;
            }
            if self.sensors.iter().any(|s| s.covers(x, y)) {
                coverage += 1;
            }
        }
        coverage
    }

    fn tuning_frequency(&self, x_y_max: i32) -> usize {
        let map_range = 0..=x_y_max;
        // An undiscovered beacon must exist outside of the coverage
        // of each sensor. We need to check the points that are range + 1
        // away from each sensor. If the missing beacon is nearby,
        // it must be at one of those range + 1 positions.
        for Sensor { x, y, range } in &self.sensors {
            for dx in 0..range + 2 {
                let dy = range + 1 - dx;
                for quadrant in [(-1, -1), (-1, 1), (1, 1), (1, -1)] {
                    let xx = x + (dx * quadrant.0);
                    let yy = y + (dy * quadrant.1);

                    if !map_range.contains(&xx) || !map_range.contains(&yy) {
                        continue;
                    }

                    // if no sensor covers that point, it must be the beacon
                    if !self.sensors.iter().any(|s| s.covers(xx, yy)) {
                        return 4_000_000 * xx as usize + yy as usize;
                    }
                }
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
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
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
                x_min = i32::min(x_min, s_x - range);
                x_max = i32::max(x_max, s_x + range);

                beacons.insert((b_x, b_y));
                Sensor {
                    x: s_x,
                    y: s_y,
                    range,
                }
            })
            .collect::<Vec<_>>();

        Self {
            sensors,
            beacons,
            x_min,
            x_max,
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
        assert_eq!(res1.0, 26);
        assert_eq!(res2.0, 56000011);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1.1, 4961647);
        assert_eq!(res2.1, 12274327017867);
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
