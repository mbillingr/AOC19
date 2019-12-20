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

    let edges = build_mapgraph(start_pos, &map);

    let mut part1 = Part1::new(edges);
    //part1.backtrack((0, vec![Tile::Start]));
    //println!("{:?}", part1.solutions);
    let mut keys: Vec<_> = part1.all_keys.iter().copied().filter(|&k| k != Tile::Start).collect();
    keys.sort();
    part1.recurse(&mut vec![Tile::Start], keys);
}

type Candidate = (usize, Vec<Tile>);

struct Part1 {
    //map: HashMap<(Tile, Tile), usize>,
    neighbors: HashMap<Tile, Vec<(Tile, usize)>>,
    edges: HashMap<(Tile, Tile), usize>,
    all_keys: HashSet<Tile>,
    solutions: Vec<Candidate>,
    minlen: usize,

    reachable_cache: HashMap<(Tile, Tile, Vec<Tile>), bool>,
    recursion_cache: HashMap<(Tile, Vec<Tile>), usize>,
}

impl Part1 {
    fn new(map: HashMap<Tile, Vec<(Tile, usize)>>) -> Self {
        Part1 {
            all_keys: map
                .keys()
                .filter(|a| match a {
                    Tile::Key(_) => true,
                    Tile::Start => true,
                    _ => false,
                })
                .copied()
                .collect(),
            edges: map.iter().flat_map(|(a, neighbors)| neighbors.iter().map(move |(b, len)| ((*a, *b), *len))).collect(),
            neighbors: map,
            solutions: vec![],
            minlen: usize::max_value(),
            reachable_cache: HashMap::new(),
            recursion_cache: HashMap::new(),
        }
    }

    fn pathlen(&self, candidate: &[Tile]) -> usize {
        (1..candidate.len())
            .map(|i| {
                self.dijkstra_search(candidate[i], candidate[i - 1], &candidate[..=i])
                    .unwrap()
            })
            .sum()
    }

    fn recurse(&mut self, visited: &mut Vec<Tile>, remaining: Vec<Tile>) -> usize {
        if remaining.is_empty() {
            return 0
        }

        let state = (*visited.last().unwrap(), remaining);
        if let Some(n) = self.recursion_cache.get(&state) {
            return *n
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
        println!("{} {:?} {:?}", best, visited, state.1);
        self.recursion_cache.insert(state, best);
        best
    }

    fn backtrack(&mut self, candidate: Candidate) {
        //println!("{:?}", candidate);
        if self.reject(&candidate) {
            return;
        }
        if self.accept(&candidate) {
            let len = candidate.0;
            if len < self.minlen {
                println!("solution: {} {:?}", len, candidate);
                self.minlen = len;
            }
            self.solutions.push(candidate.clone())
        }
        for s in self.extensions(candidate) {
            self.backtrack(s)
        }
    }

    fn reject(&mut self, candidate: &Candidate) -> bool {
        let n = candidate.1.len();
        if n < 2 {
            return false;
        }

        if candidate.0 > self.minlen {
            return true
        }

        let last_tile = *candidate.1.last().unwrap();
        /*if candidate[..n - 1].contains(&last_tile) {
            return true;
        }*/

        !self.reachable(last_tile, candidate.1[n - 2], &candidate.1[..n - 1])
    }

    fn accept(&self, candidate: &Candidate) -> bool {
        candidate.1.len() == self.all_keys.len()
    }

    fn extensions(&self, candidate: Candidate) -> impl Iterator<Item = Candidate>  {
        let last = *candidate.1.last().unwrap();
        let nexts: Vec<_> = self.all_keys
            .iter()
            .copied()
            .filter(|tile| !candidate.1.contains(tile))
            .filter_map(|tile| {
                self.dijkstra_search(tile, last, &candidate.1).map(|dist| (tile, dist))
            }).collect();

        nexts.into_iter().map(move |(tile, dist)| {
            let mut next = candidate.clone();
            next.0 += dist;
            next.1.push(tile);
            next
        })
    }

    fn reachable(&mut self, tile: Tile, from: Tile, candidate: &[Tile]) -> bool {
        let mut keyring: Vec<_> = candidate.to_vec();
        keyring.sort();
        let state = (tile, from, keyring);
        if let Some(r) = self.reachable_cache.get(&state) {
            return *r
        }

        //println!("{:?} -?> {:?}", from, tile);
        let mut visited = HashSet::new();
        let r = self.depth_first_search(tile, from, &mut visited, candidate);
        //self.breath_first_search(tile, from, candidate).is_some()

        self.reachable_cache.insert(state, r);
        r
    }

    fn depth_first_search(
        &self,
        tile: Tile,
        from: Tile,
        visited: &mut HashSet<Tile>,
        candidate: &[Tile],
    ) -> bool {
        if from == tile {
            return true;
        }

        visited.insert(from);
        if !self.passable(from, candidate) {
            return false;
        }
        for next in self.neighbors(from) {
            if visited.contains(&next) {
                continue;
            }
            if self.depth_first_search(tile, next, visited, candidate) {
                return true;
            }
        }
        false
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

    fn neighbors<'a>(&'a self, tile: Tile) -> impl Iterator<Item = Tile> + 'a {
        self.neighbors[&tile].iter().map(|(t, _)| *t)
    }
}

fn build_mapgraph(pos: Pos, map: &HashMap<Pos, Tile>) -> HashMap<Tile, Vec<(Tile, usize)>> {
    let mut nodes = HashSet::new();
    let mut edges = HashMap::new();

    nodes.insert(pos);

    let mut queue = vec![pos];
    while let Some(pos) = queue.pop() {
        println!("{:?}", pos);
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

        let edges = build_mapgraph(start_pos, &map);

        let mut part1 = Part1::new(edges);
        /*part1.backtrack((0, vec![Tile::Start]));
        for s in &part1.solutions {
            //println!("{} {:?}", part1.pathlen(s), s);
        }*/
        let mut keys: Vec<_> = part1.all_keys.iter().copied().filter(|&k| k != Tile::Start).collect();
        keys.sort();
        part1.recurse(&mut vec![Tile::Start], keys);
    }
}
