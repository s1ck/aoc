use std::{collections::HashMap, ops::Deref};

use aoc::{lines, PuzzleInput};

type Input = Passport;
type Output = usize;

register!(
    "input/day4.txt";
    (input: input!(blocks Input)) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(items: &[Input]) -> Output {
    items.iter().filter(|item| item.is_valid_part1()).count()
}

fn part2(items: &[Input]) -> Output {
    items.iter().filter(|item| item.is_valid_part2()).count()
}

#[derive(Debug)]
pub struct Passport(HashMap<String, String>);

impl Deref for Passport {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Passport {
    fn is_valid_part1(&self) -> bool {
        self.len() >= 7
            && self.contains_key("byr")
            && self.contains_key("iyr")
            && self.contains_key("eyr")
            && self.contains_key("hgt")
            && self.contains_key("hcl")
            && self.contains_key("ecl")
            && self.contains_key("pid")
    }

    fn is_valid_part2(&self) -> bool {
        let byr = self
            .get("byr")
            .and_then(|byr| byr.parse::<u16>().ok())
            .filter(|byr| (1920..=2002).contains(byr))
            .is_some();

        let iyr = self
            .get("iyr")
            .and_then(|iyr| iyr.parse::<u16>().ok())
            .filter(|iyr| (2010..=2020).contains(iyr))
            .is_some();

        let eyr = self
            .get("eyr")
            .and_then(|eyr| eyr.parse::<u16>().ok())
            .filter(|eyr| (2020..=2030).contains(eyr))
            .is_some();

        let hgt = self
            .get("hgt")
            .filter(|hgt| {
                let (n, unit) = hgt.split_at(hgt.len() - 2);
                matches!(
                    (n.parse::<u8>().ok(), unit),
                    (Some(59..=76), "in") | (Some(150..=193), "cm")
                )
            })
            .is_some();

        let hcl = self
            .get("hcl")
            .filter(|hcl| hcl.len() == 7)
            .filter(|hcl| &hcl[0..1] == "#")
            .filter(|hcl| hcl[1..].bytes().all(|b| b.is_ascii_hexdigit()))
            .is_some();

        let ecl = self
            .get("ecl")
            .filter(|ecl| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str()))
            .is_some();

        let pid = self
            .get("pid")
            .filter(|pid| pid.len() == 9)
            .filter(|pid| pid.bytes().all(|b| b.is_ascii_digit()))
            .is_some();

        byr && iyr && eyr && hgt && hcl && ecl && pid
    }
}

impl PuzzleInput for Passport {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let lines = lines(input);
        let keys = lines
            .flat_map(|line| line.split(' ').map(|e| e.split_once(':').unwrap()))
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<_, _>>();

        Self(keys)
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
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
        "#;

        let (res1, _) = Solver::run_on(input);
        assert_eq!(res1, 2);
    }

    #[test]
    fn test_part2() {
        let input = r#"
        eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007

        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "#;

        let (_, res2) = Solver::run_on(input);
        assert_eq!(res2, 4);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 233);
        assert_eq!(res2, 111);
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
