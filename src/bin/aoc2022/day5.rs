use aoc::PuzzleInput;

type Input = Unload;
type Output = String;

register!(
    "input/day5.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &Input) -> Output {
    items.0.clone()
}

fn part2(items: &Input) -> Output {
    items.1.clone()
}

type Stacks = Vec<Vec<char>>;

pub struct Unload(String, String);

#[derive(Clone, Copy)]
pub enum CrateMover {
    CrateMover9000,
    CrateMover9001,
}

impl PuzzleInput for Unload {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        fn stack_line(stacks: &mut Stacks, line: &[u8]) {
            // each stack takes up 4 chars
            let stack_cnt = (line.len() + 1) / 4;
            if stacks.is_empty() {
                stacks.resize_with(stack_cnt, Vec::new);
            }

            for s in 0..stack_cnt {
                match line[s * 4 + 1] {
                    b' ' => continue,
                    item => stacks[s].push(item as char),
                }
            }
        }

        fn move_crates(stacks: &mut Stacks, line: &str, crane: CrateMover) {
            let mut tokens = line.split_whitespace();
            let _ = tokens.next(); // 'move'
            let cnt = tokens.next().unwrap().parse::<usize>().unwrap();
            let _ = tokens.next(); // 'from'
            let src = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
            let _ = tokens.next(); // 'to'
            let tgt = tokens.next().unwrap().parse::<usize>().unwrap() - 1;

            match crane {
                CrateMover::CrateMover9000 => {
                    // split_off + reverse + append is slower
                    for _ in 0..cnt {
                        let item = stacks[src].pop().unwrap();
                        stacks[tgt].push(item);
                    }
                }
                CrateMover::CrateMover9001 => {
                    let src_len = stacks[src].len();
                    // split_off is O(1) as it reuses the buffer
                    // from the source vec for the new vec
                    let mut crates = stacks[src].split_off(src_len - cnt);
                    stacks[tgt].append(&mut crates);
                }
            }
        }

        let mut stacks: Stacks = vec![];
        // Fill up the stacks.
        let mut lines = input.lines().filter(|l| !l.is_empty());
        lines
            .by_ref()
            .take_while(|l| !l.trim_start().starts_with('1'))
            .for_each(|l| stack_line(&mut stacks, l.as_bytes()));
        // get stack content in correct order
        stacks.iter_mut().for_each(|s| s.reverse());

        // Clone the stacks to perform moves with
        // CrateMover9000 & 9001 at the same time.
        let mut staxxx = stacks.clone();

        // Let's move it.
        lines.for_each(|l| {
            move_crates(&mut stacks, l, CrateMover::CrateMover9000);
            move_crates(&mut staxxx, l, CrateMover::CrateMover9001);
        });

        let p1 = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
        let p2 = staxxx.iter().map(|s| s.last().unwrap()).collect::<String>();

        Self(p1, p2)
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
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, "CMZ".to_string());
        assert_eq!(res2, "MCD".to_string());
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, "JCMHLVGMG".to_string());
        assert_eq!(res2, "LVMRWSSPZ".to_string());
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
