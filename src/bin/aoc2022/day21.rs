use aoc::{lines, PuzzleInput};
use atoi::FromRadix10;
use fxhash::FxHashMap;

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
    0
}

pub struct Ops {
    ops: FxHashMap<String, Op>,
}

impl Ops {
    fn eval(&self, op: &str) -> isize {
        match &self.ops[op] {
            Op::Literal(n) => *n,
            Op::Add(l, r) => self.eval(&l) + self.eval(&r),
            Op::Sub(l, r) => self.eval(&l) - self.eval(&r),
            Op::Mul(l, r) => self.eval(&l) * self.eval(&r),
            Op::Div(l, r) => self.eval(&l) / self.eval(&r),
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
        assert_eq!(res2, 0);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 118565889858886);
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
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input));
    }
}
