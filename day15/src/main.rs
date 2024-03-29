use common::intcode2::{Computer, WhatsUp};
use std::collections::{HashMap, VecDeque};

fn main() {
    let mut map = HashMap::new();
    let (n_steps, final_bot) = search_breadth(&mut map);

    let mut map = HashMap::new();
    let n = explore_all(final_bot, &mut map);

    Remote::new().display(&map);
    println!("Part 1: {:?} steps", n_steps);
    println!("Part 2: {} minutes", n);
}

fn search_breadth(map: &mut HashMap<Pos, Tile>) -> (usize, Remote) {
    let mut queue = VecDeque::from(vec![(0, Remote::new())]);

    loop {
        let (steps, mut bot) = queue.pop_front().unwrap();

        for &dir in &[
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            if !map.contains_key(&(bot.pos + dir)) {
                match bot.step(dir, map) {
                    Status::Wall => {}
                    Status::Ok => {
                        queue.push_back((steps + 1, bot.clone()));
                        bot.step(-dir, map);
                    }
                    Status::Target => return (steps + 1, bot),
                }
            }
        }
    }
}

fn explore_all(start_bot: Remote, map: &mut HashMap<Pos, Tile>) -> usize {
    let mut queue = VecDeque::from(vec![(0, start_bot)]);

    let mut last_step = 0;
    while let Some((steps, mut bot)) = queue.pop_front() {
        last_step = steps;
        for &dir in &[
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            if !map.contains_key(&(bot.pos + dir)) {
                match bot.step(dir, map) {
                    Status::Wall => {}
                    Status::Ok => {
                        queue.push_back((steps + 1, bot.clone()));
                        bot.step(-dir, map);
                    }
                    Status::Target => {
                        queue.push_back((steps + 1, bot.clone()));
                        bot.step(-dir, map);
                    }
                }
            }
        }
    }
    last_step
}

#[derive(Clone)]
struct Remote {
    vm: Computer,
    pos: Pos,
}

impl Remote {
    fn new() -> Self {
        let pos = Pos { x: 0, y: 0 };
        Remote {
            vm: Computer::new(&INPUT),
            pos,
        }
    }

    fn step(&mut self, dir: Direction, map: &mut HashMap<Pos, Tile>) -> Status {
        let r = self.vm.run(Some(dir as i64));
        match r {
            Some(WhatsUp::Output(0)) => {
                map.insert(self.pos + dir, Tile::Wall);
                Status::Wall
            }
            Some(WhatsUp::Output(1)) => {
                self.pos += dir;
                map.insert(self.pos, Tile::Empty);
                Status::Ok
            }
            Some(WhatsUp::Output(2)) => {
                self.pos += dir;
                map.insert(self.pos + dir, Tile::Target);
                Status::Target
            }
            _ => panic!("Error"),
        }
    }

    fn display(&self, map: &HashMap<Pos, Tile>) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for Pos { x, y } in map.keys() {
            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match map.get(&Pos { x, y }) {
                    _ if self.pos == Pos { x, y } => print!("@"),
                    _ if x == 0 && y == 0 => print!("."),
                    Some(Tile::Wall) => print!("█"),
                    Some(Tile::Empty) => print!("░"),
                    Some(Tile::Target) => print!("*"),
                    None => print!(" "),
                }
            }
            println!()
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Target,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Status {
    Wall = 0,
    Ok = 1,
    Target = 2,
}

const INPUT: [i64; 1045] = [
    3, 1033, 1008, 1033, 1, 1032, 1005, 1032, 31, 1008, 1033, 2, 1032, 1005, 1032, 58, 1008, 1033,
    3, 1032, 1005, 1032, 81, 1008, 1033, 4, 1032, 1005, 1032, 104, 99, 102, 1, 1034, 1039, 1001,
    1036, 0, 1041, 1001, 1035, -1, 1040, 1008, 1038, 0, 1043, 102, -1, 1043, 1032, 1, 1037, 1032,
    1042, 1106, 0, 124, 1001, 1034, 0, 1039, 102, 1, 1036, 1041, 1001, 1035, 1, 1040, 1008, 1038,
    0, 1043, 1, 1037, 1038, 1042, 1106, 0, 124, 1001, 1034, -1, 1039, 1008, 1036, 0, 1041, 1002,
    1035, 1, 1040, 102, 1, 1038, 1043, 1002, 1037, 1, 1042, 1106, 0, 124, 1001, 1034, 1, 1039,
    1008, 1036, 0, 1041, 1001, 1035, 0, 1040, 1001, 1038, 0, 1043, 1002, 1037, 1, 1042, 1006, 1039,
    217, 1006, 1040, 217, 1008, 1039, 40, 1032, 1005, 1032, 217, 1008, 1040, 40, 1032, 1005, 1032,
    217, 1008, 1039, 7, 1032, 1006, 1032, 165, 1008, 1040, 33, 1032, 1006, 1032, 165, 1101, 2, 0,
    1044, 1105, 1, 224, 2, 1041, 1043, 1032, 1006, 1032, 179, 1102, 1, 1, 1044, 1105, 1, 224, 1,
    1041, 1043, 1032, 1006, 1032, 217, 1, 1042, 1043, 1032, 1001, 1032, -1, 1032, 1002, 1032, 39,
    1032, 1, 1032, 1039, 1032, 101, -1, 1032, 1032, 101, 252, 1032, 211, 1007, 0, 60, 1044, 1105,
    1, 224, 1101, 0, 0, 1044, 1106, 0, 224, 1006, 1044, 247, 101, 0, 1039, 1034, 101, 0, 1040,
    1035, 1002, 1041, 1, 1036, 1002, 1043, 1, 1038, 101, 0, 1042, 1037, 4, 1044, 1105, 1, 0, 92,
    17, 17, 33, 88, 37, 85, 63, 23, 14, 79, 46, 37, 69, 8, 6, 63, 55, 61, 21, 86, 19, 37, 78, 49,
    15, 54, 28, 54, 94, 91, 14, 11, 40, 56, 96, 20, 20, 82, 28, 12, 91, 68, 43, 18, 63, 16, 82, 71,
    8, 83, 88, 25, 79, 67, 26, 55, 33, 51, 74, 68, 59, 64, 58, 78, 30, 65, 64, 9, 48, 87, 26, 85,
    32, 82, 92, 21, 34, 99, 1, 20, 66, 34, 85, 65, 58, 87, 12, 21, 13, 51, 90, 54, 19, 12, 85, 3,
    88, 47, 31, 93, 95, 49, 70, 95, 55, 7, 67, 2, 92, 42, 80, 88, 42, 24, 91, 2, 59, 41, 41, 70,
    89, 42, 83, 43, 92, 44, 93, 62, 26, 63, 99, 81, 35, 98, 70, 71, 79, 8, 90, 26, 66, 94, 22, 47,
    55, 90, 93, 6, 87, 92, 88, 40, 73, 40, 97, 14, 73, 90, 31, 92, 16, 35, 93, 36, 27, 69, 57, 97,
    80, 34, 58, 42, 95, 34, 9, 93, 22, 94, 45, 79, 32, 33, 90, 72, 77, 58, 29, 63, 56, 95, 37, 61,
    58, 51, 57, 8, 25, 86, 75, 25, 63, 64, 93, 57, 7, 79, 85, 57, 53, 97, 16, 63, 40, 71, 52, 23,
    33, 75, 13, 56, 65, 90, 26, 12, 66, 93, 26, 36, 64, 30, 10, 75, 18, 77, 76, 86, 33, 98, 4, 23,
    52, 64, 66, 82, 38, 90, 17, 63, 94, 24, 97, 20, 92, 70, 63, 80, 19, 73, 8, 74, 93, 16, 98, 77,
    52, 38, 90, 46, 49, 76, 84, 53, 50, 22, 93, 19, 16, 61, 47, 54, 67, 56, 78, 21, 77, 52, 88, 4,
    64, 91, 90, 10, 97, 10, 51, 89, 15, 57, 97, 22, 79, 59, 92, 17, 84, 71, 30, 96, 58, 82, 52, 93,
    48, 20, 62, 4, 89, 64, 53, 85, 37, 92, 52, 89, 43, 80, 86, 2, 41, 81, 53, 53, 82, 77, 31, 66,
    92, 31, 44, 81, 14, 49, 96, 66, 42, 91, 2, 61, 82, 36, 32, 90, 8, 61, 32, 67, 52, 25, 81, 15,
    63, 27, 59, 61, 1, 15, 88, 87, 62, 10, 85, 47, 75, 24, 46, 63, 24, 77, 34, 73, 34, 45, 71, 10,
    96, 46, 43, 75, 31, 23, 72, 37, 87, 57, 88, 63, 30, 6, 86, 91, 16, 53, 16, 89, 81, 11, 32, 75,
    22, 82, 69, 50, 88, 53, 67, 50, 65, 67, 26, 81, 83, 20, 14, 23, 89, 98, 57, 64, 3, 79, 7, 69,
    89, 57, 1, 61, 65, 14, 52, 76, 66, 83, 3, 57, 90, 82, 53, 13, 72, 94, 37, 26, 97, 77, 32, 53,
    43, 78, 22, 36, 65, 83, 98, 55, 82, 58, 48, 24, 68, 92, 18, 22, 90, 65, 28, 81, 33, 63, 79, 3,
    31, 65, 92, 53, 46, 74, 7, 80, 37, 79, 79, 83, 42, 82, 84, 33, 21, 79, 79, 21, 81, 55, 4, 95,
    10, 53, 84, 14, 25, 86, 65, 24, 74, 53, 26, 61, 47, 19, 66, 86, 58, 99, 37, 83, 35, 46, 3, 11,
    89, 27, 66, 53, 33, 67, 8, 95, 44, 45, 70, 71, 65, 59, 49, 77, 25, 3, 56, 83, 39, 91, 3, 52,
    86, 67, 57, 99, 86, 40, 39, 3, 99, 25, 69, 94, 93, 62, 36, 37, 91, 17, 26, 80, 98, 77, 15, 5,
    90, 25, 40, 69, 11, 85, 66, 56, 40, 83, 61, 10, 85, 33, 28, 86, 26, 41, 61, 4, 86, 78, 20, 71,
    78, 47, 94, 39, 92, 26, 61, 91, 52, 69, 20, 47, 45, 99, 38, 96, 39, 98, 76, 58, 28, 94, 27, 47,
    97, 2, 45, 54, 64, 94, 98, 27, 69, 54, 23, 72, 89, 96, 22, 58, 21, 16, 79, 28, 45, 55, 78, 75,
    15, 92, 67, 10, 81, 80, 64, 61, 13, 30, 98, 65, 57, 35, 4, 22, 96, 72, 92, 47, 51, 87, 33, 78,
    26, 83, 20, 5, 93, 22, 73, 83, 68, 24, 17, 61, 69, 39, 62, 53, 20, 95, 84, 53, 83, 36, 48, 99,
    33, 13, 42, 90, 97, 87, 9, 55, 64, 34, 94, 7, 78, 62, 42, 43, 83, 54, 82, 57, 24, 36, 98, 95,
    54, 63, 75, 52, 15, 40, 92, 87, 77, 5, 13, 93, 48, 82, 71, 65, 97, 96, 1, 3, 68, 49, 97, 9, 77,
    88, 99, 25, 78, 4, 84, 97, 77, 4, 92, 91, 76, 53, 71, 58, 64, 55, 68, 97, 96, 48, 99, 2, 86,
    51, 69, 15, 72, 42, 72, 44, 86, 55, 73, 0, 0, 21, 21, 1, 10, 1, 0, 0, 0, 0, 0, 0,
];
