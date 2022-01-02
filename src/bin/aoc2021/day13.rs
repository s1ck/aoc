use std::collections::HashSet;

use aoc::{lines, PuzzleInput};

type Input = Origami;
type Output = String;

register!(
    "input/day13.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(origami: &Origami) -> Output {
    fold(&origami.coords, origami.folds[0]).len().to_string()
}

fn part2(origami: &Origami) -> Output {
    let coords = origami.coords.clone();
    let coords = origami.folds.iter().fold(coords, |c, f| fold(&c, *f));

    let (width, height) = coords.iter().max().unwrap();

    let row = vec!['.'; *width as usize + 1];
    let mut m = vec![row; *height as usize + 1];

    coords
        .iter()
        .for_each(|(x, y)| m[*y as usize][*x as usize] = '#');

    m.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn fold(coords: &[(u32, u32)], (axis, line): (char, u32)) -> Vec<(u32, u32)> {
    coords
        .iter()
        .fold(HashSet::new(), |mut set, (x, y)| {
            set.insert(match axis {
                'x' if *x > line => (line - (*x - line), *y),
                'y' if *y > line => (*x, line - (*y - line)),
                _ => (*x, *y),
            });
            set
        })
        .into_iter()
        .collect::<Vec<_>>()
}

pub struct Origami {
    coords: Vec<(u32, u32)>,
    folds: Vec<(char, u32)>,
}

impl PuzzleInput for Origami {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let (coords, folds) = input.split_once("\n\n").unwrap();

        let coords = lines(coords)
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
            .collect::<Vec<_>>();

        let folds = lines(folds)
            .map(str::trim)
            .map(|line| line.split(' ').nth(2).unwrap())
            .map(|fold| fold.split_once('=').unwrap())
            .map(|(axis, pos)| (axis.chars().next().unwrap(), pos.parse::<u32>().unwrap()))
            .collect::<Vec<_>>();

        Self { coords, folds }
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
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 17.to_string());
        assert_eq!(
            res2,
            String::from(
                r#"
#####
#...#
#...#
#...#
#####
                "#
                .trim()
            )
        );
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 701.to_string());
        assert_eq!(
            res2,
            String::from(
                r#"
####.###..####.#..#.###..####...##.#...
#....#..#.#....#.#..#..#.#.......#.#...
###..#..#.###..##...###..###.....#.#...
#....###..#....#.#..#..#.#.......#.#...
#....#....#....#.#..#..#.#....#..#.#...
#....#....####.#..#.###..####..##..####      
                "#
                .trim()
            )
        );
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
