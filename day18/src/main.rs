use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut input = String::new();
    File::open("data/day18-input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let (map, start_pos) = parse_map(&input);

    build_mapgraph(start_pos, &map);
}

struct Part1 {
    map: HashMap<(Tile, Tile), usize>,
    solutions: Vec<Vec<Tile>>,
}

impl Part1 {
    fn backtrack(&mut self, candidate: Vec<Tile>) {
        if self.reject(self, &candidate) {
            return
        }
        if self.accept(self, &candidate) {
            self.solutions.push(candidate.clone())
        }
    }
}

fn build_mapgraph(pos: Pos, map: &HashMap<Pos, Tile>) -> HashMap<(Tile, Tile), usize> {
    let mut nodes = HashSet::new();
    let mut edges = HashMap::new();

    nodes.insert(pos);

    let mut queue = vec![pos];
    while let Some(pos) = queue.pop() {
        println!("{:?}", pos);
        for (tile, dist) in find_all_reachable(pos, map) {
            edges.insert((map[&pos], map[&tile]), dist);

            if !nodes.contains(&tile) {
                queue.push(tile);
                nodes.insert(tile);
            }
        }
    }

    edges
}

fn find_all_reachable(start_pos: Pos, map: &HashMap<Pos, Tile>) -> HashSet<(Pos, usize)> {
    let mut reachable = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from(vec![(0, start_pos)]);
    while let Some((dist, pos)) = queue.pop_front() {
        visited.insert(pos);
        match map[&pos] {
            Tile::Wall => {}
            Tile::Key(_) | Tile::Door(_) if pos != start_pos => {reachable.insert((pos, dist));}
            _ => {
                if !visited.contains(&(pos + Direction::North)) {
                    queue.push_back((dist + 1, pos + Direction::North))
                }
                if !visited.contains(&(pos + Direction::South)) {
                    queue.push_back((dist + 1, pos + Direction::South))
                }
                if !visited.contains(&(pos + Direction::East)) {
                    queue.push_back((dist + 1, pos + Direction::East))
                }
                if !visited.contains(&(pos + Direction::West)) {
                    queue.push_back((dist + 1, pos + Direction::West))
                }
            }
        }
    }
    println!("{:?} -> {:?}", start_pos, reachable);
    reachable
}

fn parse_map(input: &str) -> (HashMap<Pos, Tile>, Pos) {
    let mut start_pos = Pos{x:0, y:0};

    let map = input.lines().enumerate()
        .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, ch)| (col as i64, row as i64, ch)))
        .inspect(|&(x, y, ch)| if ch == '@' {start_pos = Pos{x, y}})
        .map(|(x, y, ch)| (Pos{x, y}, Tile::from(ch)))
        .collect();

    (map, start_pos)
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Start,
    Wall,
    Door(char),
    Key(char),
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '@' => Tile::Start,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            ch if ch.is_ascii_uppercase() => Tile::Door(ch),
            ch if ch.is_ascii_lowercase() => Tile::Key(ch),
            _ => panic!("invalid tile: {}", ch),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input =
"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let (map, start_pos) = parse_map(&input);

        build_mapgraph(start_pos, &map);
    }

}
