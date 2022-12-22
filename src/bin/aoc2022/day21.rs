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
    ops.humns("root", &mut nodes);

    let (op, target) = match &ops.ops["root"] {
        Op::Add(l, r) | Op::Sub(l, r) | Op::Mul(l, r) | Op::Div(l, r) => {
            if nodes.contains(l.as_str()) {
                (l, ops.eval(r))
            } else {
                (r, ops.eval(l))
            }
        }
        Op::Literal(_) => unreachable!(),
    };

    ops.eval_rev(op, target, &nodes).1.unwrap_or(Output::MAX)
}

pub struct Ops {
    ops: FxHashMap<String, Op>,
}

impl Ops {
    fn eval(&self, op: &str) -> isize {
        match &self.ops[op] {
            Op::Literal(n) => *n,
            Op::Add(l, r) => self.eval(l) + self.eval(r),
            Op::Sub(l, r) => self.eval(l) - self.eval(r),
            Op::Mul(l, r) => self.eval(l) * self.eval(r),
            Op::Div(l, r) => self.eval(l) / self.eval(r),
        }
    }

    fn eval_rev(
        &self,
        op: &str,
        target: Output,
        nodes: &FxHashSet<&str>,
    ) -> (Output, Option<Output>) {
        match &self.ops[op] {
            Op::Literal(n) if op == "humn" => (*n, Some(target)),
            Op::Literal(n) => (*n, None),
            Op::Add(l, r) => {
                if nodes.contains(l.as_str()) {
                    let rhs = self.eval(r);
                    let (lhs, result) = self.eval_rev(l, target - rhs, nodes);
                    (lhs + rhs, result)
                } else if nodes.contains(r.as_str()) {
                    let lhs = self.eval(l);
                    let (rhs, result) = self.eval_rev(r, target - lhs, nodes);
                    (lhs + rhs, result)
                } else {
                    (self.eval(op), None)
                }
            }
            Op::Sub(l, r) => {
                if nodes.contains(l.as_str()) {
                    let rhs = self.eval(r);
                    let (lhs, result) = self.eval_rev(l, target + rhs, nodes);
                    (lhs - rhs, result)
                } else if nodes.contains(r.as_str()) {
                    let lhs = self.eval(l);
                    let (rhs, result) = self.eval_rev(r, lhs - target, nodes);
                    (lhs - rhs, result)
                } else {
                    (self.eval(op), None)
                }
            }
            Op::Mul(l, r) => {
                if nodes.contains(l.as_str()) {
                    let rhs = self.eval(r);
                    let (lhs, result) = self.eval_rev(l, target / rhs, nodes);
                    (lhs * rhs, result)
                } else if nodes.contains(r.as_str()) {
                    let lhs = self.eval(l);
                    let (rhs, result) = self.eval_rev(r, target / lhs, nodes);
                    (lhs * rhs, result)
                } else {
                    (self.eval(op), None)
                }
            }
            Op::Div(l, r) => {
                if nodes.contains(l.as_str()) {
                    let rhs = self.eval(r);
                    let (lhs, result) = self.eval_rev(l, target * rhs, nodes);
                    (lhs / rhs, result)
                } else if nodes.contains(r.as_str()) {
                    let lhs = self.eval(l);
                    let (rhs, result) = self.eval_rev(r, lhs / target, nodes);
                    (lhs / rhs, result)
                } else {
                    (self.eval(op), None)
                }
            }
        }
    }

    // Adds all keys to `nodes` that are part of the humn sub-tree.
    fn humns<'nodes, 'ops: 'nodes>(
        &'ops self,
        op: &str,
        nodes: &'nodes mut FxHashSet<&'ops str>,
    ) -> bool {
        if op == "humn" {
            return true;
        }
        match &self.ops[op] {
            Op::Literal(_) => false,
            Op::Add(l, r) | Op::Sub(l, r) | Op::Mul(l, r) | Op::Div(l, r) => {
                if self.humns(l, nodes) {
                    nodes.insert(l);
                    true
                } else if self.humns(r, nodes) {
                    nodes.insert(r);
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Literal(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl PuzzleInput for Ops {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let ops = lines(input)
            .map(|l| {
                let l = l.as_bytes();
                let k = String::from_utf8_lossy(&l[0..4]).to_string();
                let (n, consumed) = isize::from_radix_10(&l[6..]);
                if consumed > 0 {
                    (k, Op::Literal(n))
                } else {
                    let lhs = String::from_utf8_lossy(&l[6..10]).to_string();
                    let rhs = String::from_utf8_lossy(&l[13..17]).to_string();
                    let op = match &l[11] {
                        b'+' => Op::Add(lhs, rhs),
                        b'-' => Op::Sub(lhs, rhs),
                        b'*' => Op::Mul(lhs, rhs),
                        b'/' => Op::Div(lhs, rhs),
                        _ => unreachable!(),
                    };
                    (k, op)
                }
            })
            .collect::<FxHashMap<_, _>>();

        Self { ops }
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
