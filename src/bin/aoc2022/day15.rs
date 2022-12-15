use aoc::{lines, PuzzleInput};
use atoi::FromRadix10Signed;
use fxhash::FxHashSet;

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
    map.coverage(10)
    // map.coverage(2_000_000)
}

fn part2(map: &Input) -> Output {
    0
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Device {
    Sensor(i32, i32, i32),
    Beacon(i32, i32),
}

impl Device {
    fn covers(&self, x: i32, y: i32) -> bool {
        match self {
            Device::Sensor(s_x, s_y, range) => manhattan((*s_x, *s_y), (x, y)) <= *range,
            Device::Beacon(b_x, b_y) => *b_x == x && *b_y == y,
        }
    }
}

pub struct Map {
    sensors: Vec<Device>,
    beacons: FxHashSet<Device>,
    x_min: i32,
    x_max: i32,
}

impl Map {
    fn coverage(&self, y: i32) -> usize {
        let mut coverage = 0;
        for x in self.x_min..=self.x_max {
            if self.beacons.contains(&Device::Beacon(x, y)) {
                continue;
            }
            if self
                .sensors
                .iter()
                .any(|d| matches!(d, Device::Sensor(_, _, _)) && d.covers(x, y))
            {
                coverage += 1;
            }
        }
        coverage
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

                beacons.insert(Device::Beacon(b_x, b_y));
                Device::Sensor(s_x, s_y, range)
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
        assert_eq!(res1, 26);
        assert_eq!(res2, 56000011);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 4961647);
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
