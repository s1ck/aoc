type Input = u16;
type Output = usize;

register!(
    "input/day1.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    // max item = 2010
    let mut seen = [false; 2011];

    for &item in items {
        let diff = 2020 - item;
        if seen[usize::from(diff)] {
            return Output::from(diff) * Output::from(item);
        }
        seen[usize::from(item)] = true;
    }

    unreachable!()
}

fn part2(mut items: &[Input]) -> Output {
    // max item = 2010
    let mut seen = [false; 2011];

    while let Some(&first) = items.take_first() {
        let diff = 2020 - first;

        for &second in items {
            if second < diff {
                let third = diff - second;
                if seen[usize::from(third)] {
                    return Output::from(first) * Output::from(second) * Output::from(third);
                }
                seen[usize::from(second)] = true;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"
            1721
            979
            366
            299
            675
            1456
            "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 514579);
        assert_eq!(res2, 241861950);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 870331);
        assert_eq!(res2, 283025088);
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
