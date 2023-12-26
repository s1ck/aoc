type Input = String;
type Output = usize;

register!(
    "input/day1.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items
        .iter()
        .map(|line| {
            line.chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<_>>()
        })
        .filter(|line| !line.is_empty())
        .map(|line| format!("{}{}", line[0], line[line.len() - 1]))
        .map(|num| num.parse::<usize>().unwrap())
        .sum()
}

fn part2(items: &[Input]) -> Output {
    items
        .iter()
        .map(|line| line_to_vec(line))
        .filter(|line| !line.is_empty())
        .map(|line| format!("{}{}", line[0], line[line.len() - 1]))
        .map(|num| num.parse::<usize>().unwrap())
        .sum()
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn line_to_vec(line: &str) -> Vec<char> {
    line.chars()
        .enumerate()
        .filter_map(|(idx, c)| {
            if c.is_ascii_digit() {
                Some(c)
            } else {
                for (digit, num) in NUMBERS.iter().enumerate() {
                    if line[idx..].starts_with(num) {
                        return char::from_digit(digit as u32 + 1, 10);
                    }
                }
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 209);
        assert_eq!(res2, 281);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 53194);
        assert_eq!(res2, 54249);
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
