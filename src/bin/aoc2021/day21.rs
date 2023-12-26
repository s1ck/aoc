use fxhash::{FxBuildHasher, FxHashMap};
use std::{num::ParseIntError, str::FromStr};

type Input = Player;
type Output = usize;

register!(
    "input/day21.txt";
    (input: input!(parse Input)) -> Output {
        part1(input.clone());
        part2(&input);
    }
);

fn part1(mut players: Vec<Player>) -> Output {
    let mut dice = Dice::new();

    loop {
        for p in &mut players {
            let keep = dice.roll();
            let on = dice.roll();
            let rollin = dice.roll();

            let add = keep + on + rollin;
            let score = (p.pos as usize - 1 + add) % 10 + 1;

            p.score += score;
            p.pos = score as u8;

            if p.score >= 1000 {
                break;
            }
        }

        let winner = players.iter().find(|p| p.score >= 1000);
        let looser = players
            .iter()
            .reduce(|res, next| if next.score < res.score { next } else { res })
            .unwrap();

        if winner.is_some() {
            return looser.score * dice.rolls;
        }
    }
}

fn part2(players: &[Player]) -> Output {
    fn play(
        current: Player,
        next: Player,
        game_states: &mut FxHashMap<(Player, Player), (usize, usize)>,
    ) -> (usize, usize) {
        // 21 is the winning score
        if current.score >= 21 {
            return (1, 0);
        }
        if next.score >= 21 {
            return (0, 1);
        }

        // retrieve if already computed
        if let Some(state) = game_states.get(&(current, next)) {
            return *state;
        }

        // compute for given state
        let mut current_wins_total = 0;
        let mut next_wins_total = 0;

        for keep in [1, 2, 3] {
            for on in [1, 2, 3] {
                for rollin in [1, 2, 3] {
                    let mut player = current;
                    player.pos = (current.pos + keep + on + rollin) % 10;
                    player.score = current.score + player.pos as usize + 1;

                    let (next_wins, current_wins) = play(next, player, game_states);
                    current_wins_total += current_wins;
                    next_wins_total += next_wins;
                }
            }
        }

        game_states.insert((current, next), (current_wins_total, next_wins_total));

        (current_wins_total, next_wins_total)
    }

    assert_eq!(players.len(), 2);

    let mut p1 = players[0];
    let mut p2 = players[1];

    // 10 options for p1 * 10 options for p2 * 21 options for score1 * 21 options for score2
    // 10 * 10 * 21 * 21 = 40_000
    let mut game_states = FxHashMap::with_capacity_and_hasher(40_000, FxBuildHasher::default());
    p1.pos -= 1;
    p2.pos -= 1;

    let (wins1, wins2) = play(p1, p2, &mut game_states);

    wins1.max(wins2)
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash, Eq)]
pub struct Player {
    pos: u8,
    score: usize,
}

impl FromStr for Player {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let pos = parts[4].parse::<u8>()?;
        let score = 0;

        Ok(Self { pos, score })
    }
}

struct Dice {
    rolls: usize,
    last: usize,
}

impl Dice {
    fn new() -> Self {
        Self { rolls: 0, last: 0 }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;

        if self.last + 1 == 101 {
            self.last = 0;
        }

        self.last += 1;
        self.last
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
        Player 1 starting position: 4
        Player 2 starting position: 8
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 739785);
        assert_eq!(res2, 444356092776315);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 989352);
        assert_eq!(res2, 430229563871565);
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
        b.iter(|| part1(input.clone()));
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        let input = Solver::parse_input(Solver::puzzle_input());
        b.iter(|| part2(&input));
    }
}
