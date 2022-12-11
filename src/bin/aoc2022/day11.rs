use aoc::{lines, PuzzleInput};

type Input = Monkey;
type Output = u32;

register!(
    "input/day11.txt";
    (input: input!(blocks Input)) -> Output {

        part1(input.clone());
        part2(input);
    }
);

fn part1(mut monkeys: Vec<Input>) -> Output {
    for _ in 0..20 {
        iteration(&mut monkeys);
    }
    monkeys.sort_unstable_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}

fn part2(items: Vec<Input>) -> Output {
    0
}

fn iteration(monkeys: &mut [Input]) {
    for i in 0..monkeys.len() {
        for j in 0..(&monkeys[i].curr).len() {
            // Monkey inspects an item with a worry level of 79.
            let worry_level = &monkeys[i].curr[j];
            *(&mut monkeys[i].inspections) += 1;
            // Worry level is multiplied by 19 to 1501.
            let worry_level = match (&monkeys[i]).operation {
                Op::Square => worry_level * worry_level,
                Op::Mul(n) => worry_level * n,
                Op::Add(n) => worry_level + n,
            };
            // Monkey gets bored with item. Worry level is divided by 3 to 500.
            let worry_level = worry_level / 3;
            // Current worry level is not divisible by 23.
            if worry_level % (&monkeys[i]).divisor == 0 {
                monkeys[(&monkeys[i]).targets.0].curr.push(worry_level);
            } else {
                monkeys[(&monkeys[i]).targets.1].curr.push(worry_level);
            }
        }
        (&mut monkeys[i].curr).clear();
    }

    // prepare monkeys for next round
    // monkeys.iter_mut().for_each(|m| {
    //     std::mem::swap(&mut m.curr, &mut m.next);
    //     m.next.clear();
    // });

    // debug
    // println!();
    // monkeys
    //     .iter()
    //     .enumerate()
    //     .for_each(|(i, m)| println!("Monkey {i}: {:?}", m.curr,))
}

#[derive(Debug, Clone)]
pub enum Op {
    Square,
    Mul(u32),
    Add(u32),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    curr: Vec<u32>,
    next: Vec<u32>,
    operation: Op,
    divisor: u32,
    inspections: u32,
    targets: (usize, usize),
}

impl Monkey {
    pub fn new(items: Vec<u32>, operation: Op, divisor: u32, targets: (usize, usize)) -> Self {
        Self {
            curr: items,
            next: vec![],
            inspections: 0,
            divisor,
            operation,
            targets,
        }
    }
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
                    .split(",")
                    .map(|n| n.trim().parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let operation = lines // Operation: new = old [*|+] 19
            .next()
            .map(|l| l.split_once("old ").unwrap().1)
            .map(|op| {
                let (op, n) = op.split_once(' ').unwrap();
                match op {
                    "*" => n.parse().map(|n| Op::Mul(n)).unwrap_or(Op::Square),
                    "+" => Op::Add(n.parse().unwrap()),
                    _ => unreachable!(),
                }
            })
            .unwrap();

        let divisor = lines // Test: divisible by 23
            .next()
            .map(|l| l.split_once("by ").unwrap().1)
            .map(|n| n.parse::<u32>().unwrap())
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

        Self::new(items, operation, divisor, (true_target, false_target))
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
        assert_eq!(res2, 0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 50616);
        assert_eq!(res2, 0);
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
