use std::str;

use aoc::{lines, PuzzleInput};
use atoi::FromRadix10;
use fxhash::{FxHashMap, FxHashSet};

type Input = Ops;
type Output = isize;

register!(
    "input/day21.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(ops: &Input) -> Output {
    ops.eval("root")
}

fn part2(ops: &Input) -> Output {
    let mut nodes = FxHashSet::default();
    // Figure out all monkeys that lead to `humn`.
    ops.humns("root", &mut nodes);
    // Figure out the target value and if its produced
    // by left-hand side or right-hand side.
    let (op, target) = match &ops.ops["root"] {
        Monkey::Binary(_, lhs, rhs) => {
            if nodes.contains(lhs.as_str()) {
                (lhs, ops.eval(rhs))
            } else {
                (rhs, ops.eval(lhs))
            }
        }
        Monkey::Literal(_) => unreachable!(),
    };
    // Evaluate the formula in reverse to arrive at the same target.
    ops.eval_rev(op, target, &nodes).1.expect("No solution")
}

pub struct Ops {
    ops: FxHashMap<String, Monkey>,
}

impl Ops {
    fn eval(&self, op: &str) -> isize {
        match &self.ops[op] {
            Monkey::Literal(n) => *n,
            Monkey::Binary(op, lhs, rhs) => match op {
                Op::Add => self.eval(lhs) + self.eval(rhs),
                Op::Sub => self.eval(lhs) - self.eval(rhs),
                Op::Mul => self.eval(lhs) * self.eval(rhs),
                Op::Div => self.eval(lhs) / self.eval(rhs),
            },
        }
    }

    fn eval_rev(
        &self,
        op_key: &str,
        target: Output,
        nodes: &FxHashSet<&str>,
    ) -> (Output, Option<Output>) {
        match &self.ops[op_key] {
            Monkey::Literal(n) if op_key == "humn" => (*n, Some(target)),
            Monkey::Literal(n) => (*n, None),
            Monkey::Binary(op, lhs, rhs) => {
                if nodes.contains(lhs.as_str()) {
                    let rhs = self.eval(rhs);
                    let target = match op {
                        Op::Add => target - rhs,
                        Op::Sub => target + rhs,
                        Op::Mul => target / rhs,
                        Op::Div => target * rhs,
                    };
                    self.eval_rev(lhs, target, nodes)
                } else if nodes.contains(rhs.as_str()) {
                    let lhs = self.eval(lhs);
                    let target = match op {
                        Op::Add => target - lhs,
                        Op::Sub => lhs - target,
                        Op::Mul => target / lhs,
                        Op::Div => lhs / target,
                    };
                    self.eval_rev(rhs, target, nodes)
                } else {
                    (self.eval(op_key), None)
                }
            }
        }
    }

    // Adds all keys to `nodes` that are on the path from `op` to `humn`.
    fn humns<'nodes, 'ops: 'nodes>(
        &'ops self,
        op: &str,
        nodes: &'nodes mut FxHashSet<&'ops str>,
    ) -> bool {
        if op == "humn" {
            return true;
        }
        match &self.ops[op] {
            Monkey::Literal(_) => false,
            Monkey::Binary(_, lhs, rhs) => {
                if self.humns(lhs, nodes) {
                    nodes.insert(lhs);
                    true
                } else if self.humns(rhs, nodes) {
                    nodes.insert(rhs);
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Monkey {
    Literal(isize),
    Binary(Op, String, String),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl PuzzleInput for Ops {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let monkeys = lines(input)
            .map(|l| {
                let b = l.as_bytes();
                let k = unsafe { str::from_utf8_unchecked(&b[0..4]) }.to_owned();
                let (n, consumed) = isize::from_radix_10(&b[6..]);
                if consumed > 0 {
                    (k, Monkey::Literal(n))
                } else {
                    let lhs = unsafe { str::from_utf8_unchecked(&b[6..10]) }.to_owned();
                    let rhs = unsafe { str::from_utf8_unchecked(&b[13..17]) }.to_owned();
                    let op = match &b[11] {
                        b'+' => Op::Add,
                        b'-' => Op::Sub,
                        b'*' => Op::Mul,
                        b'/' => Op::Div,
                        _ => unreachable!(),
                    };
                    (k, Monkey::Binary(op, lhs, rhs))
                }
            })
            .collect::<FxHashMap<_, _>>();

        Self { ops: monkeys }
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
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 152);
        assert_eq!(res2, 301);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 118565889858886);
        assert_eq!(res2, 3032671800353);
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
