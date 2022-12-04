type Input = Pair;
type Output = usize;

register!(
    "input/day4.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items.iter().filter(|p| p.cover()).count()
}

fn part2(items: &[Input]) -> Output {
    items.iter().filter(|p| p.overlap()).count()
}

pub struct Pair((u8, u8), (u8, u8));

impl Pair {
    fn cover(&self) -> bool {
        self.0 .0 <= self.1 .0 && self.0 .1 >= self.1 .1
            || self.1 .0 <= self.0 .0 && self.1 .1 >= self.0 .1
    }

    fn overlap(&self) -> bool {
        self.0 .0 <= self.1 .1 && self.1 .0 <= self.0 .1
    }
}

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        let (l, r) = value.split_once(',').expect("no comma");
        let l = l
            .split_once('-')
            .map(|(l, r)| (l.parse::<u8>().unwrap(), r.parse::<u8>().unwrap()))
            .expect("no dash");
        let r = r
            .split_once('-')
            .map(|(l, r)| (l.parse::<u8>().unwrap(), r.parse::<u8>().unwrap()))
            .expect("no dash");
        Self(l, r)
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
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 2);
        assert_eq!(res2, 4);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 538);
        assert_eq!(res2, 792);
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
