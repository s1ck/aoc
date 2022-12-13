use std::{
    cmp::Ordering,
    fmt::Debug,
    iter::{self, Peekable},
    slice::{self, Iter},
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
    let d0 = Node::Lst(&[Node::Num(2)]);
    let d1 = Node::Lst(&[Node::Num(6)]);
    let mut packets = items
        .iter()
        .flat_map(|p| [&p.0, &p.1])
        .chain(iter::once(&d0).chain(iter::once(&d1)))
        .collect::<Vec<_>>();

    packets.sort_unstable();

    packets
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if **n == d0 || **n == d1 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

pub struct Pair(Node, Node);

impl PuzzleInput for Pair {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut lines = lines(input);
        Self(lines.next().unwrap().into(), lines.next().unwrap().into())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Node {
    Num(u8),
    Lst(&'static [Self]),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(l), Self::Num(r)) => l.cmp(r),
            (Self::Num(_), Self::Lst(r)) => slice::from_ref(self).cmp(r),
            (Self::Lst(l), Self::Num(_)) => l.cmp(&slice::from_ref(other)),
            (Self::Lst(l), Self::Lst(r)) => l.cmp(r),
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

impl<'a> From<&'a str> for Node {
    fn from(value: &str) -> Self {
        fn parse(bytes: &mut Peekable<Iter<'_, u8>>) -> Node {
            let mut entries = vec![];
            loop {
                match bytes.next() {
                    Some(b'[') => entries.push(parse(bytes)),
                    Some(b']') | None => {
                        return Node::Lst(entries.leak());
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
