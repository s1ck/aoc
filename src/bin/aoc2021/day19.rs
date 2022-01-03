use std::{
    ops::{Add, Sub},
    str::FromStr,
};

use aoc::{lines, PuzzleInput};
use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};
use graph::prelude::*;

type Input = Cube;
type Output = usize;

register!(
    "input/day19.txt";
    (input: input!(blocks Input)) -> Output {
        part1(input.clone());
        part2(input);
    }
);

fn part1(mut items: Vec<Input>) -> Output {
    compute(&mut items).0
}

fn part2(mut items: Vec<Input>) -> Output {
    compute(&mut items).1
}

fn compute(cubes: &mut [Cube]) -> (usize, usize) {
    let mut rotations = FxHashMap::with_capacity_and_hasher(cubes.len(), FxBuildHasher::default());

    // find rotation and translation information for overlapping cubes
    for id_left in 0..cubes.len() {
        for id_right in (id_left + 1)..cubes.len() {
            let matches = cubes[id_left].overlap(&cubes[id_right]);

            if !matches.is_empty() {
                let (diff_left, diff_right) = matches
                    .array_windows()
                    .map(|[(l0, r0), (l1, r1)]| (*l0 - *l1, *r0 - *r1))
                    .find(|(l_diff, _)| l_diff.x != l_diff.y && l_diff.y != l_diff.z)
                    .unwrap();

                let (rotation, sign) = diff_right.mapping(&diff_left);
                let scanner = Vector::center(&matches[0].0, &matches[0].1.rotate(rotation, sign));
                rotations.insert((id_left, id_right), (scanner, rotation, sign));

                let (rotation, sign) = diff_left.mapping(&diff_right);
                let scanner = Vector::center(&matches[0].1, &matches[0].0.rotate(rotation, sign));
                rotations.insert((id_right, id_left), (scanner, rotation, sign));
            }
        }
    }

    // find paths from each cube to cube 0 and transform according to rotations
    let g: UndirectedCsrGraph<usize> = GraphBuilder::new()
        .edges(rotations.keys().copied().collect::<Vec<_>>())
        .build();

    let mut beacons = cubes[0].points.iter().copied().collect::<FxHashSet<_>>();
    let mut scanners = vec![Vector::default(); cubes.len()];

    for cube_id in 1..cubes.len() {
        let path = dfs(&g, cube_id);

        let mut scanner_base: Option<Vector> = None;

        for (from, to) in path.iter().rev() {
            let (center, rotation, sign) = rotations.get(&(*from, *to)).unwrap();

            cubes[cube_id].points.iter_mut().for_each(|point| {
                let rotated = point.rotate(*rotation, *sign);
                let translated = *center + rotated;

                *point = translated;
            });

            scanner_base = scanner_base.map_or(Some(*center), |c| {
                let rotated = c.rotate(*rotation, *sign);
                let translated = *center + rotated;
                Some(translated)
            });
        }

        scanners[cube_id] = scanner_base.unwrap();

        cubes[cube_id].points.iter().for_each(|p| {
            beacons.insert(*p);
        });
    }

    let max_manhattan_sum = scanners
        .iter()
        .map(|c1| {
            scanners
                .iter()
                .map(|c2| c1.manhattan_distance(c2).sum())
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    (beacons.len(), max_manhattan_sum as usize)
}

fn dfs(g: &UndirectedCsrGraph<usize>, end: usize) -> Vec<(usize, usize)> {
    fn dfs_inner(
        g: &UndirectedCsrGraph<usize>,
        current: usize,
        end: usize,
        path: &mut Vec<(usize, usize)>,
        visited: &mut FxHashSet<usize>,
    ) -> bool {
        if current == end {
            return true;
        }
        for neighbor in g.neighbors(current) {
            if visited.contains(neighbor) {
                continue;
            }
            visited.insert(*neighbor);
            path.push((current, *neighbor));
            if dfs_inner(g, *neighbor, end, path, visited) {
                return true;
            }
            path.pop();
        }
        false
    }

    let mut path = vec![];
    let mut visited = FxHashSet::with_capacity_and_hasher(g.node_count(), FxBuildHasher::default());
    visited.insert(0);

    dfs_inner(g, 0, end, &mut path, &mut visited);

    path
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default, Eq, Hash, Ord)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl FromStr for Vector {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, s) = s.split_once(',').unwrap();
        let (y, z) = s.split_once(',').unwrap();

        Ok(Self {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap(),
            z: z.parse::<i32>().unwrap(),
        })
    }
}

impl Vector {
    fn mapping(&self, other: &Self) -> ([usize; 3], [i8; 3]) {
        let mut rotation = [0; 3];
        let mut sign = [1; 3];

        let (r0, s0) = if self.x.abs() == other.x.abs() {
            (0, if self.x == other.x { 1 } else { -1 })
        } else if self.x.abs() == other.y.abs() {
            (1, if self.x == other.y { 1 } else { -1 })
        } else {
            (2, if self.x == other.z { 1 } else { -1 })
        };
        rotation[0] = r0;
        sign[0] = s0;

        let (r1, s1) = if self.y.abs() == other.x.abs() {
            (0, if self.y == other.x { 1 } else { -1 })
        } else if self.y.abs() == other.y.abs() {
            (1, if self.y == other.y { 1 } else { -1 })
        } else {
            (2, if self.y == other.z { 1 } else { -1 })
        };

        rotation[1] = r1;
        sign[1] = s1;

        let (r2, s2) = if self.z.abs() == other.x.abs() {
            (0, if self.z == other.x { 1 } else { -1 })
        } else if self.z.abs() == other.y.abs() {
            (1, if self.z == other.y { 1 } else { -1 })
        } else {
            (2, if self.z == other.z { 1 } else { -1 })
        };

        rotation[2] = r2;
        sign[2] = s2;

        (rotation, sign)
    }

    fn rotate(&self, rotation: [usize; 3], sign: [i8; 3]) -> Self {
        let mut res = Self { x: 0, y: 0, z: 0 };

        let target = match rotation[0] {
            0 => &mut res.x,
            1 => &mut res.y,
            2 => &mut res.z,
            _ => unreachable!(),
        };

        *target = self.x * i32::from(sign[0]);

        let target = match rotation[1] {
            0 => &mut res.x,
            1 => &mut res.y,
            2 => &mut res.z,
            _ => unreachable!(),
        };

        *target = self.y * i32::from(sign[1]);

        let target = match rotation[2] {
            0 => &mut res.x,
            1 => &mut res.y,
            2 => &mut res.z,
            _ => unreachable!(),
        };

        *target = self.z * i32::from(sign[2]);

        res
    }

    fn center(left: &Self, right: &Self) -> Self {
        let c_x = if right.x > 0 {
            left.x - right.x.abs()
        } else {
            left.x + right.x.abs()
        };

        let c_y = if right.y > 0 {
            left.y - right.y.abs()
        } else {
            left.y + right.y.abs()
        };

        let c_z = if right.z > 0 {
            left.z - right.z.abs()
        } else {
            left.z + right.z.abs()
        };

        Self {
            x: c_x,
            y: c_y,
            z: c_z,
        }
    }

    fn manhattan_distance(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(other.x) as i32,
            y: self.y.abs_diff(other.y) as i32,
            z: self.z.abs_diff(other.z) as i32,
        }
    }

    fn sum(&self) -> i32 {
        self.x + self.y + self.z
    }
}

#[derive(Clone)]
pub struct Cube {
    id: u32,
    points: Vec<Vector>,
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Scanner: {}", self.id)?;
        self.points.iter().try_for_each(|p| writeln!(f, "{}", p))?;
        Ok(())
    }
}

impl PuzzleInput for Cube {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let mut lines = lines(input);
        let id = lines
            .next()
            .unwrap()
            .split(' ')
            .nth(2)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let points = lines
            .map(|line| line.parse::<Vector>().unwrap())
            .collect::<Vec<_>>();

        Self { id, points }
    }
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(str::trim).collect::<Vec<_>>();
        let id = lines[0].split(' ').nth(2).unwrap().parse::<u32>().unwrap();
        let points = lines
            .iter()
            .skip(1)
            .map(|line| line.parse::<Vector>().unwrap())
            .collect::<Vec<_>>();

        Ok(Self { id, points })
    }
}

impl Cube {
    fn distances(&self) -> Vec<FxHashSet<[u32; 3]>> {
        self.points
            .iter()
            .map(|p| {
                self.points
                    .iter()
                    .filter(|other| *p != **other)
                    .map(|other| {
                        let mut d = [0; 3];

                        d[0] = p.x.abs_diff(other.x);
                        d[1] = p.y.abs_diff(other.y);
                        d[2] = p.z.abs_diff(other.z);

                        d.sort_unstable();
                        d
                    })
                    .collect::<FxHashSet<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn overlap(&self, other: &Self) -> Vec<(Vector, Vector)> {
        let d_self = self.distances();
        let d_other = other.distances();

        let matches = d_self
            .iter()
            .enumerate()
            .flat_map(|(i_self, p_self)| {
                d_other
                    .iter()
                    .enumerate()
                    .filter_map(|(i_other, p_other)| {
                        if p_self.intersection(p_other).count() == 11 {
                            Some((self.points[i_self], other.points[i_other]))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        matches
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
         --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 79);
        assert_eq!(res2, 3621);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 398);
        assert_eq!(res2, 10965);
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
        b.iter(|| part2(input.clone()));
    }
}
