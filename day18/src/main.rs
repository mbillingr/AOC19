use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("data/day18-input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let (map, start_pos) = parse_map(&input);
    let edges = build_mapgraph(vec![start_pos], &map);

    let mut searcher = PathSearch::new(edges);
    let mut keys: Vec<_> = searcher
        .all_keys
        .iter()
        .copied()
        .filter(|&k| match k {
            Tile::Start(_) => false,
            _ => true,
        })
        .collect();
    keys.sort();
    println!(
        "Part 1: {}",
        searcher.recurse(&mut vec![Tile::Start(0)], keys)
    );

    let p1 = start_pos + Direction::North + Direction::East;
    let p2 = start_pos + Direction::North + Direction::West;
    let p3 = start_pos + Direction::South + Direction::East;
    let p4 = start_pos + Direction::South + Direction::West;

    let mut map = map;
    map.insert(start_pos, Tile::Wall);
    map.insert(start_pos + Direction::North, Tile::Wall);
    map.insert(start_pos + Direction::South, Tile::Wall);
    map.insert(start_pos + Direction::East, Tile::Wall);
    map.insert(start_pos + Direction::West, Tile::Wall);
    map.insert(p1, Tile::Start(1));
    map.insert(p2, Tile::Start(2));
    map.insert(p3, Tile::Start(3));
    map.insert(p4, Tile::Start(4));

    let edges = build_mapgraph(vec![p1, p2, p3, p4], &map);

    let mut searcher = PathSearch::new(edges);
    let mut keys: Vec<_> = searcher
        .all_keys
        .iter()
        .copied()
        .filter(|&k| match k {
            Tile::Start(_) => false,
            _ => true,
        })
        .collect();
    keys.sort();
    println!(
        "Part 2: {}",
        searcher.recurse2(
            &mut vec![
                vec![Tile::Start(1)],
                vec![Tile::Start(2)],
                vec![Tile::Start(3)],
                vec![Tile::Start(4)]
            ],
            keys
        )
    );
}

struct PathSearch {
    neighbors: HashMap<Tile, Vec<(Tile, usize)>>,
    all_keys: HashSet<Tile>,

    recursion_cache: HashMap<(Tile, Vec<Tile>), usize>,
    recursion2_cache: HashMap<(Vec<Tile>, Vec<Tile>), usize>,
}

impl PathSearch {
    fn new(map: HashMap<Tile, Vec<(Tile, usize)>>) -> Self {
        PathSearch {
            all_keys: map
                .keys()
                .filter(|a| match a {
                    Tile::Key(_) => true,
                    Tile::Start(_) => true,
                    _ => false,
                })
                .copied()
                .collect(),
            neighbors: map,
            recursion_cache: HashMap::new(),
            recursion2_cache: HashMap::new(),
        }
    }

    fn recurse2(&mut self, visited: &mut Vec<Vec<Tile>>, remaining: Vec<Tile>) -> usize {
        if remaining.is_empty() {
            return 0;
        }

        let last: Vec<_> = visited.iter().map(|v| *v.last().unwrap()).collect();
        let state = (last, remaining);
        if let Some(n) = self.recursion2_cache.get(&state) {
            return *n;
        }

        let keys: Vec<_> = visited.iter().flat_map(|v| v).copied().collect();

        let mut best = usize::max_value();
        for next in state.1.iter().copied() {
            for i in 0..visited.len() {
                if let Some(n) = self.dijkstra_search(next, *visited[i].last().unwrap(), &keys) {
                    visited[i].push(next);
                    let new_remaining = state.1.iter().copied().filter(|&r| r != next).collect();
                    //println!("{:?} {:?}", visited, new_remaining);
                    let partial = self.recurse2(visited, new_remaining);
                    visited[i].pop();
                    if partial + n < best {
                        best = partial + n;
                    }
                    break;
                }
            }
        }
        self.recursion2_cache.insert(state, best);
        best
    }

    fn recurse(&mut self, visited: &mut Vec<Tile>, remaining: Vec<Tile>) -> usize {
        if remaining.is_empty() {
            return 0;
        }

        let state = (*visited.last().unwrap(), remaining);
        if let Some(n) = self.recursion_cache.get(&state) {
            return *n;
        }

        let mut best = usize::max_value();
        for next in state.1.iter().copied() {
            if let Some(n) = self.dijkstra_search(next, *visited.last().unwrap(), &visited) {
                visited.push(next);
                let new_remaining = state.1.iter().copied().filter(|&r| r != next).collect();
                let partial = self.recurse(visited, new_remaining);
                visited.pop();
                if partial + n < best {
                    best = partial + n;
                }
            }
        }
        self.recursion_cache.insert(state, best);
        best
    }

    fn dijkstra_search(&self, tile: Tile, from: Tile, keys: &[Tile]) -> Option<usize> {
        let mut dist = HashMap::new();
        dist.insert(from, 0);

        let mut queue = BinaryHeap::new();
        queue.push((0, from));

        for &t in self.neighbors.keys() {
            if t != from {
                dist.insert(t, isize::max_value() - 1);
            }
        }

        while let Some((negdist, u)) = queue.pop() {
            if u == tile {
                return Some((-negdist) as usize);
            }
            for &(v, duv) in &self.neighbors[&u] {
                if !self.passable(v, keys) {
                    continue;
                }
                let alt = -negdist + duv as isize;
                if alt < dist[&v] {
                    dist.insert(v, alt);
                    queue.push((-alt, v));
                }
            }
        }

        None
    }

    fn passable(&self, tile: Tile, candidate: &[Tile]) -> bool {
        if let Tile::Door(ch) = tile {
            candidate.contains(&Tile::Key(ch.to_ascii_lowercase()))
        } else {
            true
        }
    }
}

fn build_mapgraph(
    positions: Vec<Pos>,
    map: &HashMap<Pos, Tile>,
) -> HashMap<Tile, Vec<(Tile, usize)>> {
    let mut nodes = HashSet::new();
    let mut edges = HashMap::new();

    for &pos in &positions {
        nodes.insert(pos);
    }

    let mut queue = positions;
    while let Some(pos) = queue.pop() {
        //println!("{:?}", pos);
        for (tile, dist) in find_all_reachable(pos, map) {
            edges
                .entry(map[&pos])
                .or_insert(vec![])
                .push((map[&tile], dist));

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
            Tile::Key(_) | Tile::Door(_) if pos != start_pos => {
                reachable.insert((pos, dist));
            }
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
    //println!("{:?} -> {:?}", start_pos, reachable);
    reachable
}

fn parse_map(input: &str) -> (HashMap<Pos, Tile>, Pos) {
    let mut start_pos = Pos { x: 0, y: 0 };

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)| (col as i64, row as i64, ch))
        })
        .inspect(|&(x, y, ch)| {
            if ch == '@' {
                start_pos = Pos { x, y }
            }
        })
        .map(|(x, y, ch)| (Pos { x, y }, Tile::from(ch)))
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Tile {
    Empty,
    Start(u8),
    Wall,
    Door(char),
    Key(char),
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '@' => Tile::Start(0),
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            ch if ch.is_ascii_uppercase() => Tile::Door(ch),
            ch if ch.is_ascii_lowercase() => Tile::Key(ch),
            _ => panic!("invalid tile: {}", ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example18_1() {
        let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        let (map, start_pos) = parse_map(&input);

        let edges = build_mapgraph(vec![start_pos], &map);

        let mut searcher = PathSearch::new(edges);
        let mut keys: Vec<_> = searcher
            .all_keys
            .iter()
            .copied()
            .filter(|&k| match k {
                Tile::Start(_) => false,
                _ => true,
            })
            .collect();
        keys.sort();
        println!("{}", searcher.recurse(&mut vec![Tile::Start(0)], keys));
    }
}
