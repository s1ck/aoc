use std::{convert::Infallible, str::FromStr};

use derive_more::Deref;

type Input = Line;
type Output = usize;

register!(
    "input/day16.txt";
    (input: input!(first input!(parse Input))) -> Output {
        part1(&input);
        part2(&input);
    }
);

fn part1(line: &str) -> Output {
    parse_packet(line).0
}

fn part2(line: &str) -> Output {
    parse_packet(line).2
}

fn parse_packet(p: &str) -> (usize, usize, usize) {
    fn parse_version(p: &str) -> (u32, usize) {
        (u32::from_str_radix(&p[0..3], 2).unwrap(), 3)
    }

    fn parse_type_id(p: &str) -> (u8, usize) {
        (u8::from_str_radix(&p[0..3], 2).unwrap(), 3)
    }

    fn parse_length_type_id(p: &str) -> (u8, usize) {
        (u8::from_str_radix(&p[0..1], 2).unwrap(), 1)
    }

    fn parse_literal(mut p: &str) -> (usize, usize, usize) {
        let mut number = String::new();
        let mut read = 0;

        loop {
            number.push_str(&p[1..5]);
            read += 5;
            if matches!(&p[0..1], "1") {
                p = &p[5..];
            } else {
                break;
            }
        }

        (0, read, usize::from_str_radix(number.as_str(), 2).unwrap())
    }

    fn parse_operator(p: &str) -> (usize, usize, Vec<usize>) {
        let mut offset = 0;
        let mut version_sum = 0;

        let (length_type_id, read) = parse_length_type_id(p);
        offset += read;

        let mut values = vec![];

        if length_type_id == 0 {
            let bits = u32::from_str_radix(&p[1..16], 2).unwrap();

            offset += 15;
            let boundary = offset + bits as usize;

            while offset < boundary {
                let (version, read, value) = parse_packet(&p[offset..]);
                values.push(value);
                offset += read;
                version_sum += version;
            }
        } else {
            let packets = u32::from_str_radix(&p[1..12], 2).unwrap();

            offset += 11;

            for _ in 0..packets {
                let (version, read, value) = parse_packet(&p[offset..]);
                values.push(value);
                offset += read;
                version_sum += version;
            }
        }

        (version_sum, offset, values)
    }

    let mut offset = 0;
    let (version, read) = parse_version(p);
    offset += read;
    let (type_id, read) = parse_type_id(&p[offset..]);
    offset += read;

    let (version_sum, read, value) = if type_id == 4 {
        parse_literal(&p[offset..])
    } else {
        let (version_sum, read, values) = parse_operator(&p[offset..]);

        let value = match type_id {
            0 => values.iter().sum(),
            1 => values.iter().product(),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => usize::from(values[0] > values[1]),
            6 => usize::from(values[0] < values[1]),
            7 => usize::from(values[0] == values[1]),
            _ => unreachable!(),
        };

        (version_sum, read, value)
    };

    (version as usize + version_sum, offset + read, value)
}

#[derive(Deref)]
pub struct Line(String);

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::fmt::Write;

        Ok(Self(s.chars().fold(String::new(), |mut res, c| {
            write!(&mut res, "{:04b}", c.to_digit(16).unwrap()).unwrap();
            res
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{Solution, SolutionExt};
    use test::Bencher;

    #[test]
    fn test_ex_part1() {
        let (res1, _) = Solver::run_on("8A004A801A8002F478");
        assert_eq!(res1, 16);

        let (res1, _) = Solver::run_on("620080001611562C8802118E34");
        assert_eq!(res1, 12);

        let (res1, _) = Solver::run_on("C0015000016115A2E0802F182340");
        assert_eq!(res1, 23);

        let (res1, _) = Solver::run_on("A0016C880162017C3686B18A3D4780");
        assert_eq!(res1, 31);
    }

    #[test]
    fn test_ex_part2() {
        let (_, res2) = Solver::run_on("C200B40A82");
        assert_eq!(res2, 3);

        let (_, res2) = Solver::run_on("04005AC33890");
        assert_eq!(res2, 54);

        let (_, res2) = Solver::run_on("880086C3E88112");
        assert_eq!(res2, 7);

        let (_, res2) = Solver::run_on("CE00C43D881120");
        assert_eq!(res2, 9);

        let (_, res2) = Solver::run_on("D8005AC2A8F0");
        assert_eq!(res2, 1);

        let (_, res2) = Solver::run_on("F600BC2D8F");
        assert_eq!(res2, 0);

        let (_, res2) = Solver::run_on("9C005AC2F8F0");
        assert_eq!(res2, 0);

        let (_, res2) = Solver::run_on("9C0141080250320F1802104A08");
        assert_eq!(res2, 1);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 979);
        assert_eq!(res2, 277110354175);
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
