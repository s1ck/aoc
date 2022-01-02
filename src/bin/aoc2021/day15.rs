use std::{cmp::Ordering, collections::BinaryHeap};

use graph::prelude::*;

type Input = Vec<u8>;
type Output = u32;

register!(
    "input/day15.txt";
    (input: input!(Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    let g = parse(items, 1);
    dijkstra(&g, 0, g.node_count() - 1)
}

fn part2(items: &[Input]) -> Output {
    let g = parse(items, 5);
    dijkstra(&g, 0, g.node_count() - 1)
}

fn dijkstra(g: &UndirectedCsrGraph<usize, (), u32>, start: usize, end: usize) -> u32 {
    let mut dist = (0..g.node_count()).map(|_| u32::MAX).collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;

    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        // found shortest path
        if position == end {
            return cost;
        }

        // already found a shorter path
        if cost > dist[position] {
            continue;
        }

        for Target { target, value } in g.neighbors_with_values(position) {
            let next = State {
                cost: cost + value,
                position: *target,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    u32::MAX
}

fn parse(field: &[Input], scale: usize) -> UndirectedCsrGraph<usize, (), u32> {
    let height = field.len();
    let width = field[0].len();
    let new_height = height * scale;
    let new_width = width * scale;

    let mut edges = Vec::new();

    (0..height).for_each(|row| {
        (0..width).for_each(|col| {
            (0..scale).for_each(|s_row| {
                (0..scale).for_each(|s_col| {
                    let n = (field[row][col] - b'0') as usize;
                    let n = (n - 1 + s_row + s_col) % 9 + 1;

                    let s_row = row + (s_row * height);
                    let s_col = col + (s_col * width);

                    let source = s_row * new_width + s_col;

                    for (n_row, n_col) in [
                        (s_row.wrapping_sub(1), s_col),
                        (s_row, s_col.wrapping_sub(1)),
                        (s_row, s_col + 1),
                        (s_row + 1, s_col),
                    ] {
                        if n_row < new_height && n_col < new_width {
                            let target = n_row * new_width + n_col;
                            edges.push((target, source, n as u32));
                        }
                    }
                });
            });
        });
    });

    GraphBuilder::new()
        .csr_layout(CsrLayout::Deduplicated)
        .edges_with_values(edges)
        .build()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 40);
        assert_eq!(res2, 315);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 462);
        assert_eq!(res2, 2846);
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
