use std::str::FromStr;

type Input = Backpack;
type Output = u32;

register!(
    "input/day3.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items
        .iter()
        .map(|b| b.0 & b.1)
        .map(u64::trailing_zeros)
        .sum()
}

fn part2(items: &[Input]) -> Output {
    items
        .iter()
        .map(|b| b.0 | b.1)
        .array_chunks()
        .map(|[a, b, c]| (a & b & c).trailing_zeros())
        .sum()
}

pub struct Backpack(u64, u64);

impl FromStr for Backpack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn mask(comp: u64, c: u8) -> u64 {
            if c.is_ascii_lowercase() {
                comp | 1 << (c - 96)
            } else {
                comp | 1 << (c - 38)
            }
        }
        let (comp1, comp2) = s.split_at(s.len() / 2);
        let comp1 = comp1.bytes().fold(0, mask);
        let comp2 = comp2.bytes().fold(0, mask);
        Ok(Self(comp1, comp2))
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
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 157);
        assert_eq!(res2, 70);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 8085);
        assert_eq!(res2, 2515);
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
