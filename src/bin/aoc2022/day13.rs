use std::{
    cmp::Ordering,
    fmt::Debug,
    iter::{self, Peekable},
    slice::Iter,
};

use aoc::{lines, PuzzleInput};

type Input = Pair;
type Output = usize;

register!(
    "input/day13.txt";
    (input: input!(blocks Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p.0.cmp(&p.1) {
            Ordering::Less => Some(i + 1),
            _ => None,
        })
        .sum()
}

fn part2(items: &[Input]) -> Output {
    let mut packets = Vec::with_capacity(items.len() * 2 + 2);
    items
        .iter()
        .flat_map(|p| iter::once(&p.0).chain(iter::once(&p.1)))
        .enumerate()
        .collect_into(&mut packets);

    let d0 = Node::Lst(vec![Node::Num(2)]);
    let d1 = Node::Lst(vec![Node::Num(6)]);

    packets.push((packets.len(), &d0));
    packets.push((packets.len(), &d1));
    packets.sort_unstable_by(|(_, l), (_, r)| l.cmp(r));

    let p0 = packets
        .iter()
        .position(|(i, _)| *i == packets.len() - 1)
        .map(|p| p + 1)
        .unwrap();

    let p1 = packets
        .iter()
        .position(|(i, _)| *i == packets.len() - 2)
        .map(|p| p + 1)
        .unwrap();

    p0 * p1
}

pub struct Pair(Node, Node);

impl PuzzleInput for Pair {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut lines = lines(input);
        Self(lines.next().unwrap().into(), lines.next().unwrap().into())
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub enum Node {
    Num(u8),
    Lst(Vec<Node>),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(l), Self::Num(r)) => l.cmp(r),
            (Self::Num(_), Self::Lst(_)) => self.to_list().cmp(other),
            (Self::Lst(_), Self::Num(_)) => self.cmp(&other.to_list()),
            (Self::Lst(l), Self::Lst(r)) => {
                let len = usize::max(l.len(), r.len());

                for i in 0..len {
                    match (l.get(i), r.get(i)) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(r)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(l), Some(r)) => match l.cmp(r) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Equal => continue,
                        },
                    };
                }
                Ordering::Equal
            }
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{n}"),
            Self::Lst(l) => write!(f, "{l:?}"),
        }
    }
}

impl Node {
    fn to_list(&self) -> Self {
        match self {
            Self::Num(_) => Self::Lst(vec![self.clone()]),
            Self::Lst(_) => self.clone(),
        }
    }
}

impl<'a> From<&'a str> for Node {
    fn from(value: &str) -> Self {
        fn parse(bytes: &mut Peekable<Iter<'_, u8>>) -> Node {
            let mut entries = vec![];
            loop {
                match bytes.next() {
                    Some(b'[') => entries.push(parse(bytes)),
                    Some(b']') | None => {
                        return Node::Lst(entries);
                    }
                    Some(d) if d.is_ascii_digit() => {
                        // THE ACTUAL INPUT HAS NUMBERS FROM 0 to 10 INCLUSIVE
                        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARGH
                        let d = d - b'0';
                        let n = match bytes.peek() {
                            Some(n) if n.is_ascii_digit() => Some(10 * d + (**n - b'0')),
                            _ => None,
                        };
                        let d = n.map_or(d, |n| {
                            let _ = bytes.next();
                            n
                        });
                        entries.push(Node::Num(d));
                    }
                    _ => {}
                }
            }
        }
        parse(&mut value.as_bytes()[1..].iter().peekable())
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
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 13);
        assert_eq!(res2, 140);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 5825);
        assert_eq!(res2, 24477);
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
