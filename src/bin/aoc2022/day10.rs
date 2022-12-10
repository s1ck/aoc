use std::{convert::Infallible, str::FromStr};

type Input = Cmd;
type Output = String;

register!(
    "input/day10.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    let ranges = [0..=20, 21..=60, 61..=100, 101..=140, 141..=180, 181..=220];
    let mut cache = [0; 6];
    let res = items.iter().fold((1, 1), |(reg, cycle), cmd| {
        for (i, range) in ranges.iter().enumerate() {
            if range.contains(&cycle) {
                cache[i] = reg;
            }
        }
        match cmd {
            Cmd::Add(v) => (reg + v, cycle + 2),
            Cmd::Noop => (reg, cycle + 1),
        }
    });

    ranges
        .iter()
        .enumerate()
        .map(|(i, r)| cache[i] * r.end())
        .sum::<i32>()
        .to_string()
}

fn part2(items: &[Input]) -> Output {
    fn draw_pixel(mid: i32, cycle: i32, crt: &mut [String]) {
        let idx = cycle / 40;
        let offset = idx * 40;
        let sprite = offset + mid - 1..=offset + mid + 1;
        let row = &mut crt[idx as usize];

        if sprite.contains(&cycle) {
            row.push('#');
        } else {
            row.push('.');
        }
    }

    let mut crt: [String; 6] = Default::default();
    items
        .iter()
        .fold((1_i32, 0), |(reg, cycle), cmd| match cmd {
            Cmd::Add(v) => {
                draw_pixel(reg, cycle, &mut crt);
                draw_pixel(reg, cycle + 1, &mut crt);
                (reg + v, cycle + 2)
            }
            Cmd::Noop => {
                draw_pixel(reg, cycle, &mut crt);
                (reg, cycle + 1)
            }
        });

    crt.join("\n")
}

pub enum Cmd {
    Add(i32),
    Noop,
}

impl FromStr for Cmd {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, v) = s.split_once(' ').unwrap_or(("noop", ""));
        let cmd = match cmd {
            "addx" => Self::Add(v.parse::<i32>().unwrap()),
            "noop" => Self::Noop,
            _ => unreachable!(),
        };
        Ok(cmd)
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
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, "13140");
        assert_eq!(
            res2,
            r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
        );
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, "14040");
        assert_eq!(
            res2,
            r#"####..##...##....##.####...##.####.#....
...#.#..#.#..#....#....#....#.#....#....
..#..#....#.......#...#.....#.###..#....
.#...#.##.#.......#..#......#.#....#....
#....#..#.#..#.#..#.#....#..#.#....#....
####..###..##...##..####..##..#....####."#
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
