use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("data/day20-input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let (map, start_pos) = parse_map(&input);
    println!("Part 1: {}", breadth_first_search(start_pos, &map));
    println!(
        "Part 2: {}",
        recursive_breadth_first_search(start_pos, &map)
    );
}

fn recursive_breadth_first_search(start: Pos, map: &HashMap<Pos, Tile>) -> usize {
    let mut queue = VecDeque::from(vec![(0, 0, start)]);
    let mut visited = HashSet::new();
    while let Some((level, steps, pos)) = queue.pop_front() {
        if visited.contains(&(level, pos)) {
            continue;
        }
        visited.insert((level, pos));

        match map.get(&pos).unwrap_or(&Tile::Wall) {
            Tile::Wall => {}
            Tile::Exit if level != 0 => {}
            Tile::Exit => return steps - 2, // remove one step for entry and one step for exit
            Tile::Empty | Tile::Entry => {
                queue.push_back((level, steps + 1, pos + Direction::North));
                queue.push_back((level, steps + 1, pos + Direction::South));
                queue.push_back((level, steps + 1, pos + Direction::East));
                queue.push_back((level, steps + 1, pos + Direction::West));
            }
            Tile::Outer(_) if level == 0 => {}
            Tile::Inner(out) => {
                queue.push_back((level + 1, steps, *out));
            }
            Tile::Outer(out) => {
                queue.push_back((level - 1, steps, *out));
            }
        }
    }
    unreachable!()
}

fn breadth_first_search(start: Pos, map: &HashMap<Pos, Tile>) -> usize {
    let mut queue = VecDeque::from(vec![(0, start)]);
    let mut visited = HashSet::new();
    while let Some((steps, pos)) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        match map.get(&pos).unwrap_or(&Tile::Wall) {
            Tile::Wall => {}
            Tile::Exit => return steps - 2, // remove one step for entry and one step for exit
            Tile::Empty | Tile::Entry => {
                queue.push_back((steps + 1, pos + Direction::North));
                queue.push_back((steps + 1, pos + Direction::South));
                queue.push_back((steps + 1, pos + Direction::East));
                queue.push_back((steps + 1, pos + Direction::West));
            }
            Tile::Outer(out) | Tile::Inner(out) => {
                queue.push_back((steps, *out));
            }
        }
    }
    unreachable!()
}

fn parse_map(input: &str) -> (HashMap<Pos, Tile>, Pos) {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let width = chars[0].len();
    let height = chars.len();

    let mut portals: HashMap<String, HashSet<Pos>> = HashMap::new();

    for i in 0..height - 1 {
        for j in 0..width - 1 {
            if chars[i][j].is_alphabetic() {
                if chars[i + 1][j].is_alphabetic() {
                    let name = format!("{}{}", chars[i][j], chars[i + 1][j]);
                    if i == 0 {
                        portals.entry(name).or_default().insert(Pos { x: j, y: 1 });
                    } else if i == height - 2 || chars[i - 1][j] == '.' {
                        portals.entry(name).or_default().insert(Pos { x: j, y: i });
                    } else if chars[i + 2][j] == '.' {
                        portals
                            .entry(name)
                            .or_default()
                            .insert(Pos { x: j, y: i + 1 });
                    }
                }

                if chars[i][j + 1].is_alphabetic() {
                    let name = format!("{}{}", chars[i][j], chars[i][j + 1]);
                    if j == 0 {
                        portals.entry(name).or_default().insert(Pos { x: 1, y: i });
                    } else if j == width - 2 || chars[i][j - 1] == '.' {
                        portals.entry(name).or_default().insert(Pos { x: j, y: i });
                    } else if chars[i][j + 2] == '.' {
                        portals
                            .entry(name)
                            .or_default()
                            .insert(Pos { x: j + 1, y: i });
                    }
                }
            }
        }
    }

    let portal_positions: HashMap<_, _> = portals
        .iter()
        .flat_map(|(name, positions)| positions.iter().map(move |pos| (*pos, name.clone())))
        .collect();

    let mut tiles = HashMap::new();

    let mut entry = Pos { x: 0, y: 0 };

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let pos = Pos { x: j, y: i };
            if let Some(name) = portal_positions.get(&pos) {
                match name.as_str() {
                    "AA" => {
                        entry = pos;
                        tiles.insert(pos, Tile::Entry)
                    }
                    "ZZ" => tiles.insert(pos, Tile::Exit),
                    _ => {
                        let other = *portals[name].iter().find(|p| **p != pos).unwrap();
                        let out = [
                            Direction::North,
                            Direction::South,
                            Direction::East,
                            Direction::West,
                        ]
                        .iter()
                        .map(|&d| other + d)
                        .find(|p| chars[p.y][p.x] == '.')
                        .unwrap();
                        if i < 2 || j < 2 || i > height - 3 || j > width - 3 {
                            tiles.insert(pos, Tile::Outer(out))
                        } else {
                            tiles.insert(pos, Tile::Inner(out))
                        }
                    }
                };
            } else {
                match chars[i][j] {
                    '#' => {
                        tiles.insert(pos, Tile::Wall);
                    }
                    '.' => {
                        tiles.insert(pos, Tile::Empty);
                    }
                    _ => {}
                }
            }
        }
    }

    (tiles, entry)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Wall,
    Outer(Pos),
    Inner(Pos),
    Entry,
    Exit,
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            _ => panic!("invalid tile: {}", ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example20_1() {
        let input = "         A    #######
         A    #######
  #######.###########
  #######.........###
  #######.#######.###
  #######.#######.###
  #######.#######.###
  #####  B    ###.###
BC...##  C    ###.###
  ##.##       ###.###
  ##...DE  F  ###.###
  #####    G  ###.###
  #########.#####.###
DE..#######...###.###
  #.#########.###.###
FG..#########.....###
  ###########.#######
             Z    ###
             Z     ##";

        let (map, start_pos) = parse_map(&input);
        println!("{}", breadth_first_search(start_pos, &map));
    }

    #[test]
    fn example20_2() {
        let input = "                   A  #############
                   A  #############
  #################.###############
  #.#...#...................#.#.###
  #.#.#.###.###.###.#########.#.###
  #.#.#.......#...#.....#.#.#...###
  #.#########.###.#####.#.#.###.###
  #.............#.#.....#.......###
  ###.###########.###.#####.#.#.###
  #.....#        A   C    #.#.#.###
  #######        S   P    #####.###
  #.#...#                 #......VT
  #.#.#.#                 #.#######
  #...#.#               YN....#.###
  #.###.#                 #####.###
DI....#.#                 #.....###
  #####.#                 #.###.###
ZZ......#               QG....#..AS
  ###.###                 #########
JO..#.#.#                 #.....###
  #.#.#.#                 ###.#.###
  #...#..DI             BU....#..LF
  #####.#                 #.#######
YN......#               VT..#....QG
  #.###.#                 #.###.###
  #.#...#                 #.....###
  ###.###    J L     J    #.#.#####
  #.....#    O F     P    #.#...###
  #.###.#####.#.#####.#####.###.###
  #...#.#.#...#.....#.....#.#...###
  #.#####.###.###.#.#.#########.###
  #...#.#.....#...#.#.#.#.....#.###
  #.###.#####.###.###.#.#.#########
  #.#.........#...#.............###
  #########.###.###.###############
           B   J   C  #############
           U   P   P               ";

        let (map, start_pos) = parse_map(&input);
        println!("{}", breadth_first_search(start_pos, &map));
    }

    #[test]
    fn example20_3() {
        let input = "         A    #######
         A    #######
  #######.###########
  #######.........###
  #######.#######.###
  #######.#######.###
  #######.#######.###
  #####  B    ###.###
BC...##  C    ###.###
  ##.##       ###.###
  ##...DE  F  ###.###
  #####    G  ###.###
  #########.#####.###
DE..#######...###.###
  #.#########.###.###
FG..#########.....###
  ###########.#######
             Z    ###
             Z     ##";

        let (map, start_pos) = parse_map(&input);
        println!("{:?}", recursive_breadth_first_search(start_pos, &map));
    }
}
