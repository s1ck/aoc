use fxhash::{FxBuildHasher, FxHashMap};

use aoc::{lines, PuzzleInput};

type Input = Manual;
type Output = usize;

register!(
    "input/day14.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &Manual) -> Output {
    iterate(&items.template, &items.rules, 10)
}

fn part2(items: &Manual) -> Output {
    iterate(&items.template, &items.rules, 40)
}

fn iterate(template: &[char], rules: &FxHashMap<(char, char), char>, steps: u32) -> usize {
    let mut counts = template.array_windows().fold(
        FxHashMap::with_capacity_and_hasher(template.len(), FxBuildHasher::default()),
        |mut counts, [left, right]| {
            *counts.entry((*left, *right)).or_insert(0) += 1;
            counts
        },
    );

    let mut char_counts = template.iter().fold(
        FxHashMap::with_capacity_and_hasher(template.len(), FxBuildHasher::default()),
        |mut counts, n| {
            *counts.entry(*n).or_insert(0) += 1;
            counts
        },
    );

    let mut next = FxHashMap::with_capacity_and_hasher(template.len(), FxBuildHasher::default());

    for _ in 0..steps {
        next.clear();
        for (t @ (left, right), count) in &counts {
            let mid = rules.get(t).unwrap();

            *next.entry((*left, *mid)).or_insert(0) += count;
            *next.entry((*mid, *right)).or_insert(0) += count;

            *char_counts.entry(*mid).or_insert(0) += count;
        }

        std::mem::swap(&mut counts, &mut next);
    }

    char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
}

pub struct Manual {
    template: Vec<char>,
    rules: FxHashMap<(char, char), char>,
}

impl PuzzleInput for Manual {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let (template, rules) = input.trim().split_once("\n\n").unwrap();

        let template = template.chars().collect::<Vec<_>>();

        let rules = lines(rules)
            .map(|line| line.split_once(" -> ").unwrap())
            .map(|(pair, insertion)| {
                let mut pair = pair.chars();
                let pair = (pair.next().unwrap(), pair.next().unwrap());
                let insertion = insertion.chars().next().unwrap();
                (pair, insertion)
            })
            .collect::<FxHashMap<_, _>>();

        Self { template, rules }
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
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 1588);
        assert_eq!(res2, 2188189693529);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 5656);
        assert_eq!(res2, 12271437788530);
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
