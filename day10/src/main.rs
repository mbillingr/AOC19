use std::collections::{HashMap, HashSet};

fn main() {
    let map = get_map(&INPUT);

    let (pos, n) = find_maxpos(&map);

    println!("Part 1: {}", n);

    let order = laserify(pos, &map);

    let n200 = &order[199].pos();
    println!("Part 2: {}", n200.0 * 100 + n200.1)
}

fn get_map(input: &str) -> HashSet<(i64, i64)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.bytes().enumerate().filter_map(move |(i, b)| match b {
                b'.' => None,
                b'#' => Some((i as i64, j as i64)),
                _ => panic!("unexpected character: {}", b),
            })
        })
        .collect()
}

fn find_maxpos(map: &HashSet<(i64, i64)>) -> ((i64, i64), i64) {
    let (mut xmax, mut ymax) = (0, 0);
    for pos in map {
        xmax = xmax.max(pos.0);
        ymax = ymax.max(pos.1);
    }

    let mut maxvis = 0;
    let mut maxpos = (9999, 9999);

    for &pos in map {
        let n = count_visible(pos, &map);
        if n > maxvis {
            maxvis = n;
            maxpos = pos;
        }
    }

    (maxpos, maxvis)
}

fn count_visible(pos: (i64, i64), map: &HashSet<(i64, i64)>) -> i64 {
    let mut vis = HashMap::new();

    let mut count = 0;

    for p in map {
        if *p == pos {
            continue;
        }

        if !vis.contains_key(p) {
            trace_line(pos, *p, &mut vis, map);
        }

        if let Visibility::Visible = vis[p] {
            count += 1;
        }
    }

    count
}

fn trace_line(
    from: (i64, i64),
    to: (i64, i64),
    vis: &mut HashMap<(i64, i64), Visibility>,
    map: &HashSet<(i64, i64)>,
) {
    use Visibility::*;
    let mut visible = Visible;
    vis.insert(from, visible);
    for (x, y) in integer_positions(from, to) {
        vis.insert((x, y), visible);
        if map.contains(&(x, y)) {
            visible = Blocked;
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Visibility {
    Visible,
    Blocked,
}

fn integer_positions(from: (i64, i64), to: (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    let d = (to.0 - from.0, to.1 - from.1);
    let g = gcd(d.0, d.1);
    let e = if g == 0 { (0, 0) } else { (d.0 / g, d.1 / g) };
    (1..=g).map(move |i| (from.0 + e.0 * i, from.1 + e.1 * i))
}

fn gcd(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

#[derive(Debug)]
struct LaserTarget {
    x: i64,
    y: i64,
    phi: Angle,
    d2: i64,
}

impl LaserTarget {
    fn new(x: i64, y: i64, offset: (i64, i64)) -> Self {
        let (x0, y0) = (x, y);
        let (x, y) = (x - offset.0, y - offset.1);
        LaserTarget {
            x: x0,
            y: y0,
            phi: Angle::from_f64(f64::atan2(x as f64, -y as f64) * 180.0 / std::f64::consts::PI),
            d2: (x * x + y * y),
        }
    }

    fn pos(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

fn laserify(laser_pos: (i64, i64), map: &HashSet<(i64, i64)>) -> Vec<LaserTarget> {
    let mut targets: Vec<LaserTarget> = map
        .iter()
        .map(|&(x, y)| LaserTarget::new(x, y, laser_pos))
        .collect();

    let mut destroyed = vec![];

    let mut laser_phi = Angle(0.0);
    while !targets.is_empty() {
        targets.sort_by_key(|t| t.d2);
        targets.sort_by_key(|t| t.phi - laser_phi);

        laser_phi = targets
            .iter()
            .find(|t| t.phi > laser_phi + TOL)
            .map(|t| t.phi)
            .unwrap_or(laser_phi + TOL);

        let target = targets.swap_remove(0);
        destroyed.push(target);
    }

    destroyed
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Angle(f64);

impl Angle {
    fn from_f64(f: f64) -> Self {
        Angle((f + 360.0) % 360.0)
    }
}

impl std::ops::Sub for Angle {
    type Output = Angle;
    fn sub(self, rhs: Self) -> Self {
        Angle((self.0 - rhs.0 + 360.0) % 360.0)
    }
}

impl std::ops::Add for Angle {
    type Output = Angle;
    fn add(self, rhs: Self) -> Self {
        Angle((self.0 + rhs.0 + 360.0) % 360.0)
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl Ord for Angle {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

impl Eq for Angle {}

const TOL: Angle = Angle(1e-9);

const INPUT: &str = "..#..###....#####....###........#
.##.##...#.#.......#......##....#
#..#..##.#..###...##....#......##
..####...#..##...####.#.......#.#
...#.#.....##...#.####.#.###.#..#
#..#..##.#.#.####.#.###.#.##.....
#.##...##.....##.#......#.....##.
.#..##.##.#..#....#...#...#...##.
.#..#.....###.#..##.###.##.......
.##...#..#####.#.#......####.....
..##.#.#.#.###..#...#.#..##.#....
.....#....#....##.####....#......
.#..##.#.........#..#......###..#
#.##....#.#..#.#....#.###...#....
.##...##..#.#.#...###..#.#.#..###
.#..##..##...##...#.#.#...#..#.#.
.#..#..##.##...###.##.#......#...
...#.....###.....#....#..#....#..
.#...###..#......#.##.#...#.####.
....#.##...##.#...#........#.#...
..#.##....#..#.......##.##.....#.
.#.#....###.#.#.#.#.#............
#....####.##....#..###.##.#.#..#.
......##....#.#.#...#...#..#.....
...#.#..####.##.#.........###..##
.......#....#.##.......#.#.###...
...#..#.#.........#...###......#.
.#.##.#.#.#.#........#.#.##..#...
.......#.##.#...........#..#.#...
.####....##..#..##.#.##.##..##...
.#.#..###.#..#...#....#.###.#..#.
............#...#...#.......#.#..
.........###.#.....#..##..#.##...";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example10_1() {
        let data = ".#..#
.....
#####
....#
...##";
        let map = get_map(&data);
        assert_eq!(find_maxpos(&map), ((3, 4), 8))
    }

    #[test]
    fn example10_2() {
        let data = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let map = get_map(&data);
        assert_eq!(find_maxpos(&map), ((5, 8), 33))
    }

    #[test]
    fn example10_large_2() {
        let data = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let mut map = get_map(&data);
        let pos = (11, 13);
        map.remove(&pos);

        let order = laserify(pos, &map);

        assert_eq!(order[1 - 1].pos(), (11, 12));
        assert_eq!(order[2 - 1].pos(), (12, 1));
        assert_eq!(order[3 - 1].pos(), (12, 2));
        assert_eq!(order[10 - 1].pos(), (12, 8));
        assert_eq!(order[20 - 1].pos(), (16, 0));
        assert_eq!(order[50 - 1].pos(), (16, 9));
        assert_eq!(order[100 - 1].pos(), (10, 16));
        assert_eq!(order[199 - 1].pos(), (9, 6));
        assert_eq!(order[200 - 1].pos(), (8, 2));
        assert_eq!(order[201 - 1].pos(), (10, 9));
        //assert_eq!(order[299-1].pos(), (11, 1));
    }

    #[test]
    fn polar_conversion() {
        assert_eq!(LaserTarget::new(0, -1, (0, 0)).phi, Angle(0.0));
        assert_eq!(LaserTarget::new(1, 0, (0, 0)).phi, Angle(90.0));
        assert_eq!(LaserTarget::new(0, 1, (0, 0)).phi, Angle(180.0));
        assert_eq!(LaserTarget::new(-1, 0, (0, 0)).phi, Angle(270.0));
    }
}
