use std::ops::{AddAssign, Mul};

use derive_more::Deref;
use num_enum::TryFromPrimitive;

type Input = Word;
type Output = u32;

register!(
    "input/day3.txt";
    (input: input!(Word)) -> Output {
        part1(&input);
        part2(&input);
    }
);

#[derive(Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Bit {
    Zero = b'0',
    One = b'1',
}

impl AddAssign<&Bit> for u32 {
    fn add_assign(&mut self, rhs: &Bit) {
        *self += match rhs {
            Bit::Zero => 0,
            Bit::One => 1,
        };
    }
}

impl Mul<&Bit> for u32 {
    type Output = Self;

    fn mul(self, rhs: &Bit) -> Self::Output {
        match rhs {
            Bit::Zero => 0,
            Bit::One => self,
        }
    }
}

#[derive(Clone, Deref)]
pub struct Word(Vec<Bit>);

impl From<Word> for u32 {
    fn from(word: Word) -> Self {
        word.as_ref().into()
    }
}

impl From<&Word> for u32 {
    fn from(word: &Word) -> Self {
        word.iter()
            .rev()
            .enumerate()
            .fold(0, |res, (i, n)| res + 2_u32.pow(i as _) * n)
    }
}

impl AsRef<Self> for Word {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl From<String> for Word {
    fn from(s: String) -> Self {
        s.bytes().map(|b| Bit::try_from(b).unwrap()).collect()
    }
}

impl FromIterator<Bit> for Word {
    fn from_iter<T: IntoIterator<Item = Bit>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Clone, Copy)]
enum Type {
    Oxy,
    Co2,
}

fn part1(words: &[Input]) -> Output {
    let input_len = words.len() as u32;
    let word_len = words[0].len() as u32;

    let gamma_rate: u32 = column_counts(words)
        .into_iter()
        .map(|n| {
            if n > input_len / 2 {
                Bit::One
            } else {
                Bit::Zero
            }
        })
        .collect::<Word>()
        .into();

    let mask = u32::pow(2, word_len) - 1;
    let epsilon_rate = !gamma_rate & mask;

    gamma_rate * epsilon_rate
}

fn part2(words: &[Input]) -> Output {
    fn filter_words<'a>(words: &[&'a Word], i: usize, tpe: Type) -> Vec<&'a Word> {
        let column_counts = column_counts(words);
        let mut filter_one = words.len() as u32 - column_counts[i] <= column_counts[i];

        if matches!(tpe, Type::Co2) {
            filter_one = !filter_one;
        }

        words
            .iter()
            .filter_map(|word| match (word[i], filter_one) {
                (Bit::Zero, false) | (Bit::One, true) => Some(*word),
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    fn rating(words: &[&Word], tpe: Type) -> u32 {
        let mut i = 0;
        let mut next = filter_words(words, i, tpe);
        while next.len() > 1 {
            i += 1;
            next = filter_words(&next, i, tpe);
        }
        next[0].into()
    }

    let words = words.iter().collect::<Vec<_>>();

    let oxy_rate = rating(&words, Type::Oxy);
    let co2_rate = rating(&words, Type::Co2);

    oxy_rate * co2_rate
}

fn column_counts<T: AsRef<Word>>(words: &[T]) -> Vec<u32> {
    words
        .iter()
        .fold(vec![0; words[0].as_ref().len()], |mut counts, word| {
            counts
                .iter_mut()
                .zip(word.as_ref().iter())
                .for_each(|(sum, bit)| *sum += bit);
            counts
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex() {
        let input = r#"00100
                       11110
                       10110
                       10111
                       10101
                       01111
                       00111
                       11100
                       10000
                       11001
                       00010
                       01010"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 198);
        assert_eq!(res2, 230);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 3309596);
        assert_eq!(res2, 2981085);
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
