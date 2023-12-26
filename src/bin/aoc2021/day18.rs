use std::str::FromStr;

type Input = Tree;
type Output = u32;

register!(
    "input/day18.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(trees: &[Tree]) -> Output {
    trees
        .iter()
        .cloned()
        .reduce(|mut t1, t2| {
            t1.add(t2);
            t1.reduce();
            t1
        })
        .unwrap()
        .magnitude()
}

fn part2(trees: &[Tree]) -> Output {
    trees
        .iter()
        .map(|left| {
            trees
                .iter()
                .cloned()
                .map(|right| {
                    let mut l = left.clone();
                    l.add(right);
                    l.reduce();
                    l.magnitude()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    Reg(u8),
    Pair(Box<(Tree, Tree)>),
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(v) => write!(f, "{v}"),
            Self::Pair(pair) => write!(f, "[{},{}]", pair.0, pair.1),
        }
    }
}

impl Tree {
    fn of(val: impl Into<Self>) -> Self {
        val.into()
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Reg(v) => u32::from(*v),
            Self::Pair(pair) => 3 * pair.0.magnitude() + 2 * pair.1.magnitude(),
        }
    }

    fn add(&mut self, other: Self) {
        let lhs = std::mem::replace(self, Self::of(0));
        let num = Self::of((lhs, other));
        *self = num;
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn insert(&mut self, value: u8, d: Direction) {
        match (self, d) {
            (Self::Reg(v), _) => *v += value,
            (Self::Pair(pair), Direction::Left) => pair.0.insert(value, d),
            (Self::Pair(pair), Direction::Right) => pair.1.insert(value, d),
        }
    }

    fn explode(&mut self) -> bool {
        fn explode(tree: &mut Tree, depth: u8) -> Option<(Option<u8>, Option<u8>)> {
            match tree {
                Tree::Pair(pair) if depth == 4 => match (&pair.0, &pair.1) {
                    (&Tree::Reg(l), &Tree::Reg(r)) => {
                        *tree = Tree::Reg(0);
                        Some((Some(l), Some(r)))
                    }
                    _ => unreachable!(),
                },
                Tree::Pair(pair) => {
                    if let Some((left, right)) = explode(&mut pair.0, depth + 1) {
                        if let Some(r) = right {
                            pair.1.insert(r, Direction::Left);
                            return Some((left, None));
                        }
                        return Some((left, None));
                    } else if let Some((left, right)) = explode(&mut pair.1, depth + 1) {
                        if let Some(l) = left {
                            pair.0.insert(l, Direction::Right);
                            return Some((None, right));
                        }
                        return Some((None, right));
                    }
                    None
                }
                Tree::Reg(_) => None,
            }
        }

        explode(self, 0).is_some()
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Reg(v) if *v > 9 => {
                *self = Self::of((*v / 2, (*v + 1) / 2));
                true
            }
            Self::Reg(_) => false,
            Self::Pair(pair) => pair.0.split() || pair.1.split(),
        }
    }
}

impl From<u8> for Tree {
    fn from(v: u8) -> Self {
        Self::Reg(v)
    }
}

impl<T: Into<Self>, U: Into<Self>> From<(T, U)> for Tree {
    fn from((left, right): (T, U)) -> Self {
        Self::Pair(Box::new((left.into(), right.into())))
    }
}

impl FromStr for Tree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (node, _) = parse(s.as_bytes());
        Ok(node)
    }
}

fn parse(bytes: &[u8]) -> (Tree, &[u8]) {
    match bytes[0] {
        b'[' => {
            let (lhs, rem) = parse(&bytes[1..]);
            let (rhs, rem) = parse(&rem[1..]); // skip comma
            (Tree::Pair(Box::new((lhs, rhs))), &rem[1..]) // skip ]
        }
        b'0'..=b'9' => (Tree::Reg(bytes[0] - b'0'), &bytes[1..]),
        c => panic!("unexpected: '{c}'"),
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
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 4140);
        assert_eq!(res2, 3993);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 3987);
        assert_eq!(res2, 4500);
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
