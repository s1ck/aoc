use fxhash::FxHashMap;

use aoc::PuzzleInput;

type Input = Burrows;
type Output = usize;

register!(
    "input/day23.txt";
    (input: input!(verbatim Input)) -> Output {
        search(input.0);
        search(input.1);
    }
);

pub struct Burrows(State, State);

impl PuzzleInput for Burrows {
    type Out = Self;

    fn from_input(_: &str) -> Self::Out {
        let input1 = State::new(
            ['.'; 11],
            [
                ['A', 'B', 'A', 'A'],
                ['D', 'C', 'B', 'B'],
                ['A', 'D', 'C', 'C'],
                ['B', 'C', 'D', 'D'],
            ],
        );

        let input2 = State::new(
            ['.'; 11],
            [
                ['A', 'D', 'D', 'B'],
                ['D', 'C', 'B', 'C'],
                ['A', 'B', 'A', 'D'],
                ['B', 'A', 'C', 'C'],
            ],
        );

        Self(input1, input2)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    corridor: [char; 11],
    rooms: [[char; 4]; 4],
    locks: [[bool; 4]; 4],
}

type MoveOut = ((usize, usize), usize);
type MoveIn = (usize, (usize, usize));

impl State {
    const ROOM_MAP: [char; 4] = ['A', 'B', 'C', 'D'];
    const CORRIDOR_SLOTS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

    fn new(corridor: [char; 11], rooms: [[char; 4]; 4]) -> Self {
        Self {
            corridor,
            rooms,
            locks: [[false; 4]; 4],
        }
    }

    fn is_room_solved(&self, room: usize) -> bool {
        self.rooms[room].iter().all(|e| *e == Self::ROOM_MAP[room])
    }

    fn is_room_full(&self, room: usize) -> bool {
        self.rooms[room].iter().all(|e| *e != '.')
    }

    fn is_room_empty(&self, room: usize) -> bool {
        self.rooms[room].iter().all(|e| *e == '.')
    }

    fn is_solved(&self) -> bool {
        (0..self.rooms.len()).all(|r| self.is_room_solved(r))
    }

    fn move_out(&self, ((room, slot), to): MoveOut) -> Self {
        let mut res = *self;
        res.corridor[to] = self.rooms[room][slot];
        res.rooms[room][slot] = '.';
        res
    }

    fn move_in(&self, (from, (room, slot)): MoveIn) -> Self {
        let mut res = *self;
        res.corridor[from] = '.';
        res.rooms[room][slot] = self.corridor[from];
        res.locks[room][slot] = true;
        res
    }

    fn possible_out_moves(&self) -> Vec<(MoveOut, usize)> {
        let mut moves_out = vec![];

        if let Some(mvs) = self.possible_moves_from_room(0) {
            moves_out.extend(mvs);
        }
        if let Some(mvs) = self.possible_moves_from_room(1) {
            moves_out.extend(mvs);
        }
        if let Some(mvs) = self.possible_moves_from_room(2) {
            moves_out.extend(mvs);
        }
        if let Some(mvs) = self.possible_moves_from_room(3) {
            moves_out.extend(mvs);
        }

        moves_out
    }

    fn possible_in_moves(&self) -> Vec<(MoveIn, usize)> {
        Self::CORRIDOR_SLOTS
            .iter()
            .filter(|corridor| self.corridor[**corridor] != '.')
            .filter_map(|corridor| self.possible_move_to_room(*corridor))
            .collect::<Vec<_>>()
    }

    fn possible_move_to_room(&self, corridor: usize) -> Option<(MoveIn, usize)> {
        let element = self.corridor[corridor];
        let room = (element as u8 - b'A') as usize;

        if self.is_room_full(room) {
            return None;
        }

        // is the corridor free up until the room?
        let is_valid_move = if corridor < ((room + 1) * 2) {
            self.is_valid_move(room, corridor + 1)
        } else {
            self.is_valid_move(room, corridor - 1)
        };

        if !is_valid_move {
            return None;
        }

        if !self.rooms[room]
            .iter()
            .all(|e| *e == '.' || *e == Self::ROOM_MAP[room])
        {
            return None;
        }

        let slot = self.rooms[room]
            .iter()
            .enumerate()
            .find(|(_, e)| **e != '.')
            .map_or(self.rooms[room].len(), |(slot, _)| slot)
            - 1;

        let cost = Self::cost(element) * Self::distance(((room, slot), corridor));

        Some(((corridor, (room, slot)), cost))
    }

    fn possible_moves_from_room(&self, room: usize) -> Option<Vec<(MoveOut, usize)>> {
        if self.is_room_solved(room) || self.is_room_empty(room) {
            return None;
        }
        // find first entry to move
        let (slot, _) = self.rooms[room]
            .iter()
            .enumerate()
            .find(|(_, e)| **e != '.')
            .unwrap();

        // If elements are already in the correct
        // room, we don't need to move anything out.
        if self.rooms[room][slot..]
            .iter()
            .all(|e| *e == Self::ROOM_MAP[room])
        {
            return None;
        }

        // create all valid moves based on the current state
        let moves = Self::CORRIDOR_SLOTS
            .iter()
            .map(|to| ((room, slot), *to))
            .filter(|((room, slot), _)| !self.locks[*room][*slot])
            .filter(|((room, _), corridor)| self.is_valid_move(*room, *corridor))
            .map(|mv @ ((room, slot), _)| {
                (mv, Self::cost(self.rooms[room][slot]) * Self::distance(mv))
            })
            .collect::<Vec<_>>();

        Some(moves)
    }

    fn distance(((room, slot), corridor): MoveOut) -> usize {
        ((room + 1) * 2).abs_diff(corridor) + 1 + slot
    }

    fn cost(e: char) -> usize {
        match e {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => 0,
        }
    }

    fn is_valid_move(&self, room: usize, corridor: usize) -> bool {
        let top = (room + 1) * 2;
        let lo = corridor.min(top);
        let hi = corridor.max(top);

        (lo..=hi).all(|s| self.corridor[s] == '.')
    }
}

fn search(state: State) -> usize {
    let mut states = FxHashMap::default();
    simulate(state, &mut states)
}

fn simulate(state: State, states: &mut FxHashMap<State, usize>) -> usize {
    if state.is_solved() {
        return 0;
    }

    if let Some(cost) = states.get(&state) {
        return *cost;
    }

    let mut costs = vec![];

    for (mv, cost) in state.possible_out_moves() {
        let new_state = state.move_out(mv);
        let current_cost = simulate(new_state, states);
        costs.push(current_cost.saturating_add(cost));
    }

    for (mv, cost) in state.possible_in_moves() {
        let new_state = state.move_in(mv);
        let current_cost = simulate(new_state, states);
        costs.push(current_cost.saturating_add(cost));
    }

    let local_min = costs.iter().copied().min().unwrap_or(usize::MAX);

    states.insert(state, local_min);

    local_min
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 13455);
        assert_eq!(res2, 43567);
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
        b.iter(|| search(input.0));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| search(input.1));
    }
}
