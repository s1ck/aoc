use aoc::{lines, PuzzleInput};
use graph::prelude::*;
use indexmap::IndexSet;

type Input = CaveSystem;
type Output = u32;

register!(
    "input/day12.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(cave_system: &Input) -> Output {
    let start = cave_system.cave_id("start");
    let end = cave_system.cave_id("end");

    cave_system.dfs(start, start, end, &mut vec![], false)
}

fn part2(cave_system: &Input) -> Output {
    let start = cave_system.cave_id("start");
    let end = cave_system.cave_id("end");

    cave_system.dfs(start, start, end, &mut vec![], true)
}

#[derive(Debug)]
enum CaveSize {
    Big,
    Small,
}

pub struct CaveSystem {
    g: UndirectedCsrGraph<usize, CaveSize>,
    ids: IndexSet<String>,
}

impl CaveSystem {
    fn cave_id(&self, label: &str) -> usize {
        self.ids.get_full(label).unwrap().0
    }

    fn size(&self, id: usize) -> &CaveSize {
        self.g.node_value(id)
    }

    fn edges(&self, id: usize) -> &[usize] {
        self.g.neighbors(id).as_slice()
    }

    fn dfs(
        &self,
        current_cave: usize,
        source: usize,
        target: usize,
        path: &mut Vec<usize>,
        can_revisit: bool,
    ) -> u32 {
        if current_cave == target {
            return 1;
        }

        let can_revisit = match (
            matches!(self.size(current_cave), CaveSize::Small),
            path.contains(&current_cave),
            can_revisit,
            current_cave == source,
        ) {
            (true, true, _, true) | (true, true, false, _) => return 0,
            (true, true, true, _) => false,
            _ => can_revisit,
        };

        path.push(current_cave);
        let mut count = 0;
        for neighbor in self.edges(current_cave) {
            count += self.dfs(*neighbor, source, target, path, can_revisit);
        }
        path.pop();

        count
    }
}

impl PuzzleInput for CaveSystem {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut ids = IndexSet::new();
        let mut edges = Vec::new();

        lines(input)
            .map(str::trim)
            .map(|line| line.split_once('-').unwrap())
            .for_each(|(source, target)| {
                let (source, _) = ids.insert_full(source.to_string());
                let (target, _) = ids.insert_full(target.to_string());
                edges.push((source, target));
            });

        let node_values = ids
            .iter()
            .map(|cave| {
                if cave.chars().any(char::is_lowercase) {
                    CaveSize::Small
                } else {
                    CaveSize::Big
                }
            })
            .collect::<Vec<_>>();

        let g = GraphBuilder::new()
            .csr_layout(CsrLayout::Deduplicated)
            .edges(edges)
            .node_values(node_values)
            .build();

        Self { g, ids }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex_1() {
        let input = r#"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 10);
        assert_eq!(res2, 36);
    }

    #[test]
    fn test_ex_2() {
        let input = r#"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 19);
        assert_eq!(res2, 103);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 5104);
        assert_eq!(res2, 149220);
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
