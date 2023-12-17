use std::str::FromStr;

type Input = Game;
type Output = usize;

register!(
    "input/day2.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(games: &[Input]) -> Output {
    games
        .iter()
        .filter(|game| game.is_valid())
        .map(|g| g.id)
        .sum()
}

fn part2(games: &[Input]) -> Output {
    games
        .iter()
        .map(|game| {
            game.sets
                .iter()
                .fold(Set::default(), |agg, next| agg.max(next))
                .power()
        })
        .sum()
}

pub struct Game {
    pub id: usize,
    sets: Vec<Set>,
}

#[derive(Default)]
pub struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }

    fn max(self, other: &Self) -> Self {
        Self {
            red: usize::max(self.red, other.red),
            green: usize::max(self.green, other.green),
            blue: usize::max(self.blue, other.blue),
        }
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.sets.iter().all(Set::is_valid)
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, sets) = s.split_once(": ").unwrap();
        let (_, id) = id.split_once(' ').unwrap();
        let id = id.parse::<usize>().unwrap();

        let sets = sets
            .trim()
            .split("; ")
            .map(|set| {
                set.split(", ").fold((0, 0, 0), |mut agg, set| {
                    let (n, color) = set.split_once(' ').unwrap();
                    let n = n.parse::<usize>().unwrap();
                    match color {
                        "red" => agg.0 += n,
                        "green" => agg.1 += n,
                        "blue" => agg.2 += n,
                        _ => panic!("unsupported color: {color}"),
                    };
                    agg
                })
            })
            .map(|(red, green, blue)| Set { red, green, blue })
            .collect::<Vec<_>>();

        Ok(Self { id, sets })
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
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 8);
        assert_eq!(res2, 2286);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 2061);
        assert_eq!(res2, 72596);
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
