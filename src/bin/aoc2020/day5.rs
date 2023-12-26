type Input = String;
type Output = usize;

register!(
    "input/day5.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items
        .iter()
        .map(|pass| seat_id(pass))
        .max()
        .unwrap_or_default()
}

fn part2(items: &[Input]) -> Output {
    let mut seats = items.iter().map(|pass| seat_id(pass)).collect::<Vec<_>>();
    seats.sort_unstable();
    seats.array_windows().find(|[l, r]| r - l > 1).unwrap()[0] + 1
}

fn seat_id(pass: &str) -> usize {
    let r = fold(&pass[0..7], b'F', (0, 127));
    let c = fold(&pass[7..10], b'L', (0, 7));

    r * 8 + c
}

fn fold(pass: &str, sep: u8, init: (usize, usize)) -> usize {
    let (low, high) = &pass[..pass.len() - 1].bytes().fold(init, |(l, h), cmd| {
        if cmd == sep {
            (l, l + l.abs_diff(h) / 2)
        } else {
            (h - l.abs_diff(h) / 2, h)
        }
    });

    match pass.as_bytes()[pass.len() - 1] {
        b if b == sep => *low,
        _ => *high,
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
        FBFBBFFRLR        
        BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL
        "#;
        let (res1, _) = Solver::run_on(input);
        assert_eq!(res1, 820);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 885);
        assert_eq!(res2, 623);
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
