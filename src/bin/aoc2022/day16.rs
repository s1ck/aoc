use std::cell::RefCell;
use tap::prelude::*;

use aoc::{lines, PuzzleInput};
use atoi::FromRadix10;
use fxhash::FxHashMap;

type Input = Pipes;
type Output = u32;

register!(
    "input/day16.txt";
    (input: input!(verbatim Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(pipes: &Input) -> Output {
    max_pressure(
        pipes.nodes["AA"],
        pipes.flows.clone(),
        &pipes.distances(),
        30,
        false,
        pipes.nodes["AA"],
    )
}

fn part2(pipes: &Input) -> Output {
    max_pressure(
        pipes.nodes["AA"],
        pipes.flows.clone(),
        &pipes.distances(),
        26,
        true,
        pipes.nodes["AA"],
    )
}

std::thread_local! {
    pub static MEMOIZED_MAX_PRESSURE: RefCell<FxHashMap<(usize, Vec<u32>, u32, bool), Output>> = RefCell::new(FxHashMap::default());
}

fn max_pressure(
    curr: usize,
    flows: FxHashMap<usize, u32>,
    distances: &[Vec<u32>],
    time_left: u32,
    use_elephant: bool,
    aa_node_id: usize,
) -> Output {
    if let Some(max) = MEMOIZED_MAX_PRESSURE.with(|m| {
        m.borrow()
            .get(&(
                curr,
                flows
                    .values()
                    .cloned()
                    .collect::<Vec<u32>>()
                    .tap_mut(|f| f.sort_unstable()),
                time_left,
                use_elephant,
            ))
            .cloned()
    }) {
        return max;
    }

    let max = flows
        .iter()
        // Only visit the node if there is enough time to get there and open the valve.
        // We need 1 minute * distance to get to `next` plus 1 minute to open the valve.
        .filter(|(next, _)| distances[curr][**next] + 1 < time_left)
        .map(|(next, flow)| {
            // Only look at the remaining possible flows.
            let mut flows = flows.clone();
            flows.remove(next);
            // It takes us `distance` minutes to get to n and 1 minute to open the valve.
            let time_left = time_left - distances[curr][*next] - 1;
            // Therefore, the valve will be open for `time_left` minutes.
            let cost = time_left * flow;
            // Find the maximum pressure for the remaning flows.
            cost + max_pressure(*next, flows, distances, time_left, use_elephant, aa_node_id)
        })
        .chain(
            // Figure out, if its better to let the elephant process the remanings flows.
            use_elephant
                .then(|| max_pressure(aa_node_id, flows.clone(), distances, 26, false, aa_node_id)),
        )
        .max()
        .unwrap_or_default();

    MEMOIZED_MAX_PRESSURE.with(|m| {
        m.borrow_mut().insert(
            (
                curr,
                flows
                    .values()
                    .cloned()
                    .collect::<Vec<u32>>()
                    .tap_mut(|f| f.sort_unstable()),
                time_left,
                use_elephant,
            ),
            max,
        )
    });

    max
}

#[derive(Debug)]
pub struct Pipes {
    nodes: FxHashMap<String, usize>,
    flows: FxHashMap<usize, u32>,
    edges: FxHashMap<usize, Vec<usize>>,
}

impl Pipes {
    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn distances(&self) -> Vec<Vec<u32>> {
        let mut distances = vec![];
        distances.resize_with(self.node_count(), || vec![u32::MAX; self.node_count()]);

        (0..self.node_count()).for_each(|n| distances[n][n] = 0);
        self.edges
            .iter()
            .for_each(|(s, tgts)| tgts.iter().for_each(|t| distances[*s][*t] = 1));

        for k in 0..self.node_count() {
            for i in 0..self.node_count() {
                for j in 0..self.node_count() {
                    let dist_via_k = {
                        let i_to_k = distances[i][k];
                        let k_to_j = distances[k][j];
                        if i_to_k == u32::MAX || k_to_j == u32::MAX {
                            u32::MAX
                        } else {
                            i_to_k + k_to_j
                        }
                    };
                    if dist_via_k < distances[i][j] {
                        distances[i][j] = dist_via_k;
                    }
                }
            }
        }

        distances
    }
}

impl PuzzleInput for Pipes {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut nodes = FxHashMap::<String, usize>::default();
        let mut flows = FxHashMap::<usize, u32>::default();
        let mut edges = FxHashMap::<usize, Vec<usize>>::default();
        let mut next = 0;

        let mut next_id = || {
            let id = next;
            next += 1;
            id
        };

        lines(input).for_each(|line| {
            let line = line.as_bytes();
            let label = std::str::from_utf8(&line[6..8]).unwrap().to_string();
            let id = nodes.entry(label).or_insert_with(&mut next_id).clone();
            let (flow, used) = u32::from_radix_10(&line[23..]);
            if flow > 0 {
                flows.insert(id, flow);
            }
            // "valves" vs "valve" (if only one target)
            let targets = std::str::from_utf8(&line[23 + used + 19..])
                .unwrap()
                .split_once(' ')
                .unwrap()
                .1
                .split(", ")
                .map(|id| {
                    nodes
                        .entry(id.to_string())
                        .or_insert_with(&mut next_id)
                        .clone()
                })
                .collect::<Vec<_>>();

            edges.insert(id, targets);
        });

        Self {
            nodes,
            flows,
            edges,
        }
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
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 1651);
        assert_eq!(res2, 1707);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1775);
        assert_eq!(res2, 2351);
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
