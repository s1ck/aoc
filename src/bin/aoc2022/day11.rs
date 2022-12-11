use aoc::{lines, PuzzleInput};

type Input = Monkey;
type Output = usize;

register!(
    "input/day11.txt";
    (input: input!(blocks Input)) -> Output {
        part1(input.clone());
        part2(input);
    }
);

fn part1(monkeys: Vec<Input>) -> Output {
    simulate::<20, _>(monkeys, |worry_level| worry_level / 3)
}

fn part2(monkeys: Vec<Input>) -> Output {
    // For each item, we need to maintain the forwarding condition.
    // We can make use of the following rule in modulo arithmetics:
    // A MOD C == 0 <=> (A MOD (C * D) MOD C) == 0 and
    // A MOD D == 0 <=> (A MOD (C * D) MOD D) == 0
    // I.e., if a number A is divisible by C, A is also divisible
    // by any multiple of C.
    //
    // In order to maintain that condition for each monkey, we can
    // use the product of all divisors to keep the levels low:
    // A MOD C == 0 <=> (A % (C * D)) MOD C == 0 and
    // A MOD D == 0 <=> (A % (C * D)) MOD D == 0
    let multiple = monkeys.iter().fold(1, |acc, m| acc * m.divisor);
    simulate::<10_000, _>(monkeys, |worry_level| worry_level % multiple)
}

fn simulate<const N: usize, F>(mut monkeys: Vec<Input>, f: F) -> usize
where
    F: Fn(usize) -> usize,
{
    for _ in 0..N {
        iterate(&mut monkeys, &f);
    }
    monkeys.sort_unstable_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}

fn iterate<F>(monkeys: &mut [Input], f: F)
where
    F: Fn(usize) -> usize,
{
    for i in 0..monkeys.len() {
        for j in 0..monkeys[i].items.len() {
            let worry_level = &monkeys[i].items[j];
            *(&mut monkeys[i].inspections) += 1;
            let worry_level = match monkeys[i].operation {
                Op::Square => worry_level * worry_level,
                Op::Mul(n) => worry_level * n,
                Op::Add(n) => worry_level + n,
            };
            let worry_level = f(worry_level);
            if worry_level % monkeys[i].divisor == 0 {
                monkeys[monkeys[i].targets.0].items.push(worry_level);
            } else {
                monkeys[monkeys[i].targets.1].items.push(worry_level);
            }
        }
        monkeys[i].items.clear();
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Square,
    Mul(usize),
    Add(usize),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<usize>,
    operation: Op,
    divisor: usize,
    inspections: usize,
    targets: (usize, usize),
}

impl PuzzleInput for Monkey {
    type Out = Self;

    fn from_input(block: &str) -> Self::Out {
        let mut lines = lines(block);
        let _ = lines.next(); // Monkey i:
        let items = lines // Starting items: 79, 98
            .next()
            .map(|l| l.split_once(':').unwrap().1)
            .map(|nums| {
                nums.trim()
                    .split(',')
                    .map(|n| n.trim().parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let operation = lines // Operation: new = old [*|+] 19
            .next()
            .map(|l| l.split_once("old ").unwrap().1)
            .map(|op| {
                let (op, n) = op.split_once(' ').unwrap();
                match op {
                    "*" => n.parse().map(Op::Mul).unwrap_or(Op::Square),
                    "+" => Op::Add(n.parse().unwrap()),
                    _ => unreachable!(),
                }
            })
            .unwrap();

        let divisor = lines // Test: divisible by 23
            .next()
            .map(|l| l.split_once("by ").unwrap().1)
            .map(|n| n.parse::<usize>().unwrap())
            .unwrap_or_default();

        let true_target = lines // If true: throw to monkey i
            .next()
            .map(|l| l.split_once("monkey ").unwrap().1)
            .map(|n| n.parse::<usize>().unwrap())
            .unwrap();

        let false_target = lines // If false: throw to monkey i
            .next()
            .map(|l| l.split_once("monkey ").unwrap().1)
            .map(|n| n.parse::<usize>().unwrap())
            .unwrap();

        Self {
            items,
            operation,
            divisor,
            inspections: 0,
            targets: (true_target, false_target),
        }
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
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 10605);
        assert_eq!(res2, 2713310158);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 50616);
        assert_eq!(res2, 11309046332);
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
        b.iter(|| part1(input.clone()));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(input.clone()));
    }
}
