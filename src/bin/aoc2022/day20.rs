type Input = i64;
type Output = i64;

register!(
    "input/day20.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    mix(items, 1, 1)
}

fn part2(items: &[Input]) -> Output {
    mix(items, 811_589_153, 10)
}

fn mix(items: &[Input], dec_key: i64, rounds: usize) -> Output {
    let len = items.len();
    let mut idx = (0..len).collect::<Vec<_>>();

    for _ in 0..rounds {
        (0..len).for_each(|i| {
            let pos = idx.iter().position(|o| *o == i).unwrap();
            let shift = items[i] * dec_key;
            idx.remove(pos);
            let target = (pos as Input + shift).rem_euclid(len as Input - 1);
            idx.insert(target as usize, i);
        });
    }

    let idx_0 = items.iter().position(|val| *val == 0).unwrap();
    let idx_0_new = idx.iter().position(|idx| *idx == idx_0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| items[idx[(idx_0_new + i) % len]] * dec_key)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"
        1
        2
        -3
        3
        -2
        0
        4
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 3);
        assert_eq!(res2, 1623178306);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 13522);
        assert_eq!(res2, 17113168880158);
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
