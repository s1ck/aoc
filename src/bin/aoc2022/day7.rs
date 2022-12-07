use aoc::{lines, PuzzleInput};

type Input = Node;
type Output = u32;

register!(
    "input/day7.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(fs: &Input) -> Output {
    fs.part1()
}

fn part2(fs: &Input) -> Output {
    fs.part2(30_000_000 - (70_000_000 - fs.size))
}

#[derive(Debug)]
pub struct Node {
    subs: Vec<Node>,
    size: u32,
}

impl Node {
    fn part1(&self) -> u32 {
        let sub_sum = self.subs.iter().map(Node::part1).sum();
        if self.size <= 100_000 {
            self.size + sub_sum
        } else {
            sub_sum
        }
    }

    fn part2(&self, remove: u32) -> u32 {
        if self.size < remove {
            u32::MAX
        } else {
            u32::min(
                self.size,
                self.subs
                    .iter()
                    .map(|n| n.part2(remove))
                    .min()
                    .unwrap_or(u32::MAX),
            )
        }
    }
}

impl PuzzleInput for Node {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        fn read_dir<'a, I>(lines: &mut I) -> Node
        where
            I: Iterator<Item = &'a str>,
        {
            let mut size = 0;
            let mut subs = vec![];

            while let Some(line) = lines.next() {
                match line.split_once(' ').unwrap() {
                    ("$", cmd) => match cmd.split_once(' ') {
                        Some(("cd", "..")) => {
                            return Node { subs, size };
                        }
                        Some(("cd", _)) => {
                            let sub = read_dir(lines);
                            size += sub.size;
                            subs.push(sub);
                        }
                        _ => {}
                    },
                    ("dir", _) => {}
                    (file_size, _) => size += file_size.parse::<u32>().unwrap(),
                }
            }

            return Node { subs, size };
        }

        read_dir(&mut lines(input).skip(1))
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
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 95437);
        assert_eq!(res2, 24933642);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1350966);
        assert_eq!(res2, 6296435);
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
