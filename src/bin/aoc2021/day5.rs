use std::{collections::HashMap, str::FromStr};

type Input = Line;
type Output = usize;

register!(
    "input/day5.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(lines: &[Input]) -> Output {
    lines
        .iter()
        .filter(|line| line.is_straight())
        .fold(HashMap::new(), |mut points, line| {
            line.points()
                .iter()
                .for_each(|p| *points.entry(*p).or_insert(0) += 1);
            points
        })
        .values()
        .filter(|c| **c > 1)
        .count()
}

fn part2(lines: &[Input]) -> Output {
    lines
        .iter()
        .fold(HashMap::new(), |mut points, line| {
            line.points()
                .iter()
                .for_each(|p| *points.entry(*p).or_insert(0) += 1);
            points
        })
        .values()
        .filter(|c| **c > 1)
        .count()
}

pub struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_once(',')
            .map(|(x, y)| Self {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
            .unwrap())
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_once(" -> ")
            .map(|(p1, p2)| Self {
                start: p1.parse().unwrap(),
                end: p2.parse().unwrap(),
            })
            .unwrap())
    }
}

impl Line {
    fn points(&self) -> Vec<(i32, i32)> {
        let dist_x = (self.start.x - self.end.x).abs();
        let dist_y = (self.start.y - self.end.y).abs();

        let dir_x = self.end.x.cmp(&self.start.x) as i32;
        let dir_y = self.end.y.cmp(&self.start.y) as i32;

        let step_x = dist_x.cmp(&0) as i32 * dir_x;
        let step_y = dist_y.cmp(&0) as i32 * dir_y;

        let length = dist_x.max(dist_y) + 1;

        (0..length).fold(vec![], |mut points, i| {
            points.push((self.start.x + (i * step_x), self.start.y + (i * step_y)));
            points
        })
    }

    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"0,9 -> 5,9
                       8,0 -> 0,8
                       9,4 -> 3,4
                       2,2 -> 2,1
                       7,0 -> 7,4
                       6,4 -> 2,0
                       0,9 -> 2,9
                       3,4 -> 1,4
                       0,0 -> 8,8
                       5,5 -> 8,2"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 5);
        assert_eq!(res2, 12);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 6113);
        assert_eq!(res2, 20373);
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
