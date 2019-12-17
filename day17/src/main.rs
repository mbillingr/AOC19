use common::intcode2::Computer;
use std::collections::HashMap;

fn main() {
    let output = Computer::new(&INPUT).map(std::iter::empty()).unwrap();

    let output: String =
        String::from_utf8(output.into_iter().map(|o| o as u8).collect::<Vec<u8>>()).unwrap();
    println!("{}", output);

    let grid: Vec<Vec<u8>> = output
        .lines()
        .map(str::bytes)
        .map(Iterator::collect)
        .collect();

    let height = grid.len() - 1;
    let width = grid[0].len();
    println!("{} x {}", width, height);

    let mut alignment_sum = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if grid[i][j] == b'#'
                && grid[i + 1][j] == b'#'
                && grid[i - 1][j] == b'#'
                && grid[i][j + 1] == b'#'
                && grid[i][j - 1] == b'#'
            {
                alignment_sum += i * j;
            }
        }
    }

    let mut start_pos = Pos { x: 0, y: 0 };
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == b'^' {
                start_pos = Pos {
                    x: j as i64,
                    y: i as i64,
                };
            }
        }
    }

    let prog2: Vec<_> = std::iter::once(2)
        .chain(INPUT.iter().skip(1).copied())
        .collect();

    let path = generate_path(start_pos, Direction::North, &grid);

    let (main, subs) = find_partition(&path);
    let mut main: String = main.into_iter().map(|ch| format!("{},", ch)).collect();
    main.pop().unwrap();
    let subs: HashMap<_, _> = subs
        .into_iter()
        .map(|(k, v)| (k, stringify(v).unwrap()))
        .inspect(|(_, v)| assert!(v.len() <= 20))
        .collect();

    let mut program = main;
    program.push(10u8 as char);
    for s in &['A', 'B', 'C'] {
        program.extend(subs[s].chars());
        program.push(10u8 as char);
    }

    program += "n\n";

    let mut comp = Computer::new(&prog2);
    let mut output = comp.map(program.bytes().map(|b| b as _)).unwrap();

    let dust = output.pop().unwrap();

    print!(
        "{}",
        output.iter().map(|&x| x as u8 as char).collect::<String>()
    );

    println!("Part 1: {}", alignment_sum);
    println!("Part 2: {}", dust);
}

fn find_partition(path: &Vec<Command>) -> (Vec<char>, HashMap<char, &[Command]>) {
    recurse_partition(vec![], HashMap::new(), path).unwrap()
}

fn recurse_partition<'a>(
    mut main: Vec<char>,
    mut subs: HashMap<char, &'a [Command]>,
    mut path: &'a [Command],
) -> Option<(Vec<char>, HashMap<char, &'a [Command]>)> {
    while let Some(sub) = match_subprog(&subs, path) {
        main.push(sub);
        path = &path[subs[&sub].len()..];
    }

    if path.is_empty() {
        return Some((main, subs));
    }

    let mut subname = 'A';
    while subs.contains_key(&subname) {
        subname = match subname {
            'A' => 'B',
            'B' => 'C',
            'C' => return None,
            _ => unreachable!(),
        };
    }

    for len in 2..10 {
        let subslice = &path[0..len];
        if stringify(subslice).is_none() {
            continue;
        };

        subs.insert(subname, subslice);

        if let Some(solution) = recurse_partition(main.clone(), subs.clone(), path) {
            return Some(solution);
        }
    }

    None
}

fn match_subprog(subs: &HashMap<char, &[Command]>, path: &[Command]) -> Option<char> {
    let mut sublen: Vec<_> = subs.iter().map(|(&key, &code)| (code.len(), key)).collect();
    sublen.sort();

    for key in sublen.into_iter().map(|(_, k)| k) {
        if path.starts_with(subs[&key]) {
            return Some(key);
        }
    }

    None
}

fn stringify(part: &[Command]) -> Option<String> {
    let mut s = String::with_capacity(25);
    for cmd in part {
        match cmd {
            Command::L => s.push('L'),
            Command::R => s.push('R'),
            Command::N(i) => s += &i.to_string(),
        }
        if s.len() > 20 {
            return None;
        }
        s.push(',');
    }
    s.pop();
    Some(s)
}

fn generate_path(start_pos: Pos, mut dir: Direction, grid: &Vec<Vec<u8>>) -> Vec<Command> {
    let mut pos = start_pos;
    let mut path = vec![];
    while !(dead_end(pos, &grid) && pos != start_pos) {
        let delta = new_direction(pos, dir, grid);
        dir = dir + delta;
        path.push(delta);
        path.push(Command::N(0));
        while at(pos + dir, grid) == b'#' {
            path.last_mut().unwrap().inc();
            pos += dir;
        }
    }
    path
}

fn dead_end(pos: Pos, grid: &Vec<Vec<u8>>) -> bool {
    std::iter::once(pos)
        .chain(std::iter::once(pos + Direction::North))
        .chain(std::iter::once(pos + Direction::South))
        .chain(std::iter::once(pos + Direction::West))
        .chain(std::iter::once(pos + Direction::East))
        .map(|p| at(p, &grid))
        .filter(|&ch| ch == b'#')
        .count()
        < 3
}

fn new_direction(pos: Pos, dir: Direction, grid: &Vec<Vec<u8>>) -> Command {
    let l = at(pos + (dir + Command::L), &grid) == b'#';
    let r = at(pos + (dir + Command::R), &grid) == b'#';
    match (l, r) {
        (true, true) => panic!("T-Junction!? at {:?}", pos),
        (true, false) => Command::L,
        (false, true) => Command::R,
        (false, false) => panic!("Dead end at {:?}", pos),
    }
}

fn at(pos: Pos, grid: &Vec<Vec<u8>>) -> u8 {
    if pos.x < 0 || pos.x >= 45 || pos.y < 0 || pos.y >= 37 {
        b'.'
    } else {
        grid[pos.y as usize][pos.x as usize]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Command {
    L,
    R,
    N(u8),
}

impl Command {
    fn inc(&mut self) {
        match self {
            Command::N(i) => *i += 1,
            _ => panic!("no movement commmand"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
}

impl std::ops::Add<Direction> for Pos {
    type Output = Pos;
    fn add(self, dir: Direction) -> Pos {
        match dir {
            Direction::North => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Direction::East => Pos {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl std::ops::AddAssign<Direction> for Pos {
    fn add_assign(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.y -= 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
            Direction::East => self.x += 1,
        }
    }
}

impl std::ops::Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl std::ops::Add<Command> for Direction {
    type Output = Direction;
    fn add(self, delta: Command) -> Self {
        match (self, delta) {
            (Direction::North, Command::R) => Direction::East,
            (Direction::North, Command::L) => Direction::West,
            (Direction::South, Command::R) => Direction::West,
            (Direction::South, Command::L) => Direction::East,
            (Direction::West, Command::R) => Direction::North,
            (Direction::West, Command::L) => Direction::South,
            (Direction::East, Command::R) => Direction::South,
            (Direction::East, Command::L) => Direction::North,
            (_, _) => panic!("invalid command {:?}", delta),
        }
    }
}

const INPUT: [i64; 1467] = [
    1, 330, 331, 332, 109, 3132, 1102, 1, 1182, 16, 1101, 1467, 0, 24, 101, 0, 0, 570, 1006, 570,
    36, 101, 0, 571, 0, 1001, 570, -1, 570, 1001, 24, 1, 24, 1105, 1, 18, 1008, 571, 0, 571, 1001,
    16, 1, 16, 1008, 16, 1467, 570, 1006, 570, 14, 21102, 58, 1, 0, 1106, 0, 786, 1006, 332, 62,
    99, 21102, 1, 333, 1, 21102, 73, 1, 0, 1106, 0, 579, 1101, 0, 0, 572, 1101, 0, 0, 573, 3, 574,
    101, 1, 573, 573, 1007, 574, 65, 570, 1005, 570, 151, 107, 67, 574, 570, 1005, 570, 151, 1001,
    574, -64, 574, 1002, 574, -1, 574, 1001, 572, 1, 572, 1007, 572, 11, 570, 1006, 570, 165, 101,
    1182, 572, 127, 1001, 574, 0, 0, 3, 574, 101, 1, 573, 573, 1008, 574, 10, 570, 1005, 570, 189,
    1008, 574, 44, 570, 1006, 570, 158, 1105, 1, 81, 21101, 0, 340, 1, 1106, 0, 177, 21101, 0, 477,
    1, 1105, 1, 177, 21101, 514, 0, 1, 21102, 176, 1, 0, 1106, 0, 579, 99, 21102, 1, 184, 0, 1106,
    0, 579, 4, 574, 104, 10, 99, 1007, 573, 22, 570, 1006, 570, 165, 1002, 572, 1, 1182, 21102, 1,
    375, 1, 21101, 0, 211, 0, 1106, 0, 579, 21101, 1182, 11, 1, 21102, 1, 222, 0, 1106, 0, 979,
    21102, 388, 1, 1, 21102, 233, 1, 0, 1105, 1, 579, 21101, 1182, 22, 1, 21101, 244, 0, 0, 1106,
    0, 979, 21102, 1, 401, 1, 21101, 255, 0, 0, 1105, 1, 579, 21101, 1182, 33, 1, 21102, 266, 1, 0,
    1105, 1, 979, 21102, 1, 414, 1, 21102, 1, 277, 0, 1105, 1, 579, 3, 575, 1008, 575, 89, 570,
    1008, 575, 121, 575, 1, 575, 570, 575, 3, 574, 1008, 574, 10, 570, 1006, 570, 291, 104, 10,
    21102, 1, 1182, 1, 21101, 313, 0, 0, 1105, 1, 622, 1005, 575, 327, 1101, 0, 1, 575, 21102, 1,
    327, 0, 1106, 0, 786, 4, 438, 99, 0, 1, 1, 6, 77, 97, 105, 110, 58, 10, 33, 10, 69, 120, 112,
    101, 99, 116, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 101, 32,
    98, 117, 116, 32, 103, 111, 116, 58, 32, 0, 12, 70, 117, 110, 99, 116, 105, 111, 110, 32, 65,
    58, 10, 12, 70, 117, 110, 99, 116, 105, 111, 110, 32, 66, 58, 10, 12, 70, 117, 110, 99, 116,
    105, 111, 110, 32, 67, 58, 10, 23, 67, 111, 110, 116, 105, 110, 117, 111, 117, 115, 32, 118,
    105, 100, 101, 111, 32, 102, 101, 101, 100, 63, 10, 0, 37, 10, 69, 120, 112, 101, 99, 116, 101,
    100, 32, 82, 44, 32, 76, 44, 32, 111, 114, 32, 100, 105, 115, 116, 97, 110, 99, 101, 32, 98,
    117, 116, 32, 103, 111, 116, 58, 32, 36, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32, 99, 111,
    109, 109, 97, 32, 111, 114, 32, 110, 101, 119, 108, 105, 110, 101, 32, 98, 117, 116, 32, 103,
    111, 116, 58, 32, 43, 10, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 109, 97,
    121, 32, 98, 101, 32, 97, 116, 32, 109, 111, 115, 116, 32, 50, 48, 32, 99, 104, 97, 114, 97,
    99, 116, 101, 114, 115, 33, 10, 94, 62, 118, 60, 0, 1, 0, -1, -1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
    10, 0, 109, 4, 1202, -3, 1, 587, 20102, 1, 0, -1, 22101, 1, -3, -3, 21101, 0, 0, -2, 2208, -2,
    -1, 570, 1005, 570, 617, 2201, -3, -2, 609, 4, 0, 21201, -2, 1, -2, 1105, 1, 597, 109, -4,
    2105, 1, 0, 109, 5, 2102, 1, -4, 630, 20102, 1, 0, -2, 22101, 1, -4, -4, 21101, 0, 0, -3, 2208,
    -3, -2, 570, 1005, 570, 781, 2201, -4, -3, 653, 20101, 0, 0, -1, 1208, -1, -4, 570, 1005, 570,
    709, 1208, -1, -5, 570, 1005, 570, 734, 1207, -1, 0, 570, 1005, 570, 759, 1206, -1, 774, 1001,
    578, 562, 684, 1, 0, 576, 576, 1001, 578, 566, 692, 1, 0, 577, 577, 21101, 702, 0, 0, 1105, 1,
    786, 21201, -1, -1, -1, 1105, 1, 676, 1001, 578, 1, 578, 1008, 578, 4, 570, 1006, 570, 724,
    1001, 578, -4, 578, 21101, 0, 731, 0, 1106, 0, 786, 1106, 0, 774, 1001, 578, -1, 578, 1008,
    578, -1, 570, 1006, 570, 749, 1001, 578, 4, 578, 21101, 0, 756, 0, 1105, 1, 786, 1105, 1, 774,
    21202, -1, -11, 1, 22101, 1182, 1, 1, 21102, 1, 774, 0, 1106, 0, 622, 21201, -3, 1, -3, 1106,
    0, 640, 109, -5, 2106, 0, 0, 109, 7, 1005, 575, 802, 21002, 576, 1, -6, 20101, 0, 577, -5,
    1105, 1, 814, 21101, 0, 0, -1, 21101, 0, 0, -5, 21102, 1, 0, -6, 20208, -6, 576, -2, 208, -5,
    577, 570, 22002, 570, -2, -2, 21202, -5, 45, -3, 22201, -6, -3, -3, 22101, 1467, -3, -3, 1201,
    -3, 0, 843, 1005, 0, 863, 21202, -2, 42, -4, 22101, 46, -4, -4, 1206, -2, 924, 21102, 1, 1, -1,
    1105, 1, 924, 1205, -2, 873, 21101, 0, 35, -4, 1105, 1, 924, 2102, 1, -3, 878, 1008, 0, 1, 570,
    1006, 570, 916, 1001, 374, 1, 374, 2102, 1, -3, 895, 1102, 2, 1, 0, 1201, -3, 0, 902, 1001,
    438, 0, 438, 2202, -6, -5, 570, 1, 570, 374, 570, 1, 570, 438, 438, 1001, 578, 558, 921, 21001,
    0, 0, -4, 1006, 575, 959, 204, -4, 22101, 1, -6, -6, 1208, -6, 45, 570, 1006, 570, 814, 104,
    10, 22101, 1, -5, -5, 1208, -5, 37, 570, 1006, 570, 810, 104, 10, 1206, -1, 974, 99, 1206, -1,
    974, 1102, 1, 1, 575, 21101, 0, 973, 0, 1106, 0, 786, 99, 109, -7, 2105, 1, 0, 109, 6, 21101,
    0, 0, -4, 21102, 0, 1, -3, 203, -2, 22101, 1, -3, -3, 21208, -2, 82, -1, 1205, -1, 1030, 21208,
    -2, 76, -1, 1205, -1, 1037, 21207, -2, 48, -1, 1205, -1, 1124, 22107, 57, -2, -1, 1205, -1,
    1124, 21201, -2, -48, -2, 1106, 0, 1041, 21102, 1, -4, -2, 1105, 1, 1041, 21101, 0, -5, -2,
    21201, -4, 1, -4, 21207, -4, 11, -1, 1206, -1, 1138, 2201, -5, -4, 1059, 1202, -2, 1, 0, 203,
    -2, 22101, 1, -3, -3, 21207, -2, 48, -1, 1205, -1, 1107, 22107, 57, -2, -1, 1205, -1, 1107,
    21201, -2, -48, -2, 2201, -5, -4, 1090, 20102, 10, 0, -1, 22201, -2, -1, -2, 2201, -5, -4,
    1103, 2101, 0, -2, 0, 1106, 0, 1060, 21208, -2, 10, -1, 1205, -1, 1162, 21208, -2, 44, -1,
    1206, -1, 1131, 1106, 0, 989, 21102, 1, 439, 1, 1105, 1, 1150, 21101, 0, 477, 1, 1106, 0, 1150,
    21102, 1, 514, 1, 21102, 1, 1149, 0, 1105, 1, 579, 99, 21101, 1157, 0, 0, 1106, 0, 579, 204,
    -2, 104, 10, 99, 21207, -3, 22, -1, 1206, -1, 1138, 1201, -5, 0, 1176, 2102, 1, -4, 0, 109, -6,
    2106, 0, 0, 8, 9, 36, 1, 7, 1, 36, 1, 1, 13, 30, 1, 7, 1, 5, 1, 30, 1, 7, 1, 5, 1, 7, 11, 12,
    1, 7, 1, 5, 1, 7, 1, 9, 1, 12, 1, 7, 1, 5, 1, 7, 1, 1, 13, 8, 1, 7, 1, 5, 1, 7, 1, 1, 1, 7, 1,
    3, 1, 8, 1, 7, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 1, 8, 1, 7, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 10, 5,
    9, 1, 13, 3, 1, 3, 1, 14, 1, 1, 1, 7, 1, 5, 1, 1, 1, 3, 1, 3, 1, 3, 1, 8, 9, 5, 9, 1, 1, 3, 1,
    3, 1, 3, 1, 8, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 1, 3, 1, 3, 1, 8, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 1,
    3, 1, 3, 1, 8, 1, 5, 1, 7, 1, 1, 1, 7, 1, 3, 1, 3, 1, 3, 1, 8, 1, 5, 1, 7, 1, 1, 1, 7, 9, 3, 1,
    8, 1, 5, 1, 7, 1, 1, 1, 11, 1, 7, 1, 8, 1, 5, 1, 7, 1, 1, 11, 1, 9, 8, 1, 5, 1, 7, 1, 11, 1,
    18, 13, 1, 1, 11, 1, 24, 1, 5, 1, 1, 1, 11, 1, 24, 9, 11, 1, 30, 1, 13, 1, 30, 1, 13, 1, 30, 1,
    13, 1, 30, 1, 13, 9, 22, 1, 21, 1, 22, 11, 11, 1, 32, 1, 11, 1, 32, 1, 11, 1, 32, 1, 11, 1, 32,
    1, 11, 1, 32, 1, 11, 1, 32, 1, 11, 1, 32, 1, 11, 1, 32, 13, 2,
];
