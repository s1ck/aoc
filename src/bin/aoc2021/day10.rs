type Input = String;
type Output = usize;

register!(
    "input/day10.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(lines: &[String]) -> Output {
    lines.iter().map(|line| check_line(line).0).sum::<u32>() as usize
}

fn part2(lines: &[String]) -> Output {
    let mut scores = lines
        .iter()
        .map(|line| check_line(line))
        .filter(|(error, _)| *error == 0)
        .map(|(_, stack)| {
            stack
                .iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |prod, n| prod * 5 + n)
        })
        .collect::<Vec<_>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn check_line(line: &str) -> (u32, Vec<char>) {
    let mut stack = vec![];

    let error = line
        .chars()
        .map(|c| match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
                0
            }
            _ => match stack.pop().unwrap() {
                '(' if c == ')' => 0,
                '[' if c == ']' => 0,
                '{' if c == '}' => 0,
                '<' if c == '>' => 0,
                _ => match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                },
            },
        })
        .find(|x| *x > 0)
        .unwrap_or_default();

    (error, stack)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 26397);
        assert_eq!(res2, 288957);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 364389);
        assert_eq!(res2, 2870201088);
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
