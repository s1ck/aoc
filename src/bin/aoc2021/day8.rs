use std::{collections::HashSet, convert::Infallible, str::FromStr};

type Input = Line;
type Output = u32;

register!(
    "input/day8.txt";
    (input: input!(parse Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(inputs: &[Input]) -> Output {
    inputs.iter().fold(0, |sum, input| {
        sum + input.output.iter().fold(0, |sum, word| {
            sum + match word.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        })
    })
}

fn part2(inputs: &[Input]) -> Output {
    fn decode(input: &Input) -> u32 {
        let Input { input, output } = input;

        let input = input
            .iter()
            .map(|word| word.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let mut found = Vec::with_capacity(10);
        found.resize(10, None);

        found[1] = input.iter().find(|word| word.len() == 2);
        found[4] = input.iter().find(|word| word.len() == 4);
        found[7] = input.iter().find(|word| word.len() == 3);
        found[8] = input.iter().find(|word| word.len() == 7);
        found[9] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|&word| {
                (found[8].unwrap() - word).len() == 1
                    && (found[1].unwrap() - word).is_empty()
                    && (found[4].unwrap() - word).is_empty()
            });
        found[6] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|&word| {
                (found[8].unwrap() - word).len() == 1 && (found[1].unwrap() - word).len() == 1
            });
        found[0] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|word| word.len() == 6);
        found[5] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|&word| {
                (word - found[9].unwrap()).is_empty() && (found[1].unwrap() - word).len() == 1
            });
        found[2] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|&word| word.len() == 5 && (found[1].unwrap() - word).len() == 1);
        found[3] = input
            .iter()
            .filter(|&word| !found.contains(&Some(word)))
            .find(|_| true);

        output
            .iter()
            .rev()
            .map(|word| word.chars().collect::<HashSet<_>>())
            .map(|word| {
                found
                    .iter()
                    .enumerate()
                    .find(|(_, &n)| n.unwrap() == &word)
                    .unwrap()
                    .0 as u32
            })
            .enumerate()
            .fold(0, |sum, (i, n)| sum + (10_u32.pow(i as u32) * n))
    }

    inputs.iter().fold(0, |sum, input| sum + decode(input))
}

#[derive(Debug)]
pub struct Line {
    input: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = s.split_once(" | ").unwrap();
        let input = input.split(' ').map(String::from).collect::<Vec<_>>();
        let output = output.split(' ').map(String::from).collect::<Vec<_>>();

        Ok(Self { input, output })
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
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
                "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 26);
        assert_eq!(res2, 61229);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 310);
        assert_eq!(res2, 915941);
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
