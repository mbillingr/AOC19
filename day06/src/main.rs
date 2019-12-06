use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("data/day06-input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let bodies = load(&input);

    println!("Part 1: {}", count_orbits("COM", &bodies));
    println!("Part 2: {}", find_orbital_distance("YOU", "SAN", &bodies));
}

fn load(input: &str) -> HashMap<&str, SpaceObject> {
    let mut bodies: HashMap<&str, SpaceObject> = HashMap::new();

    for (a, b) in input.lines().map(|line| {
        let mut l = line.split(')');
        let center = l.next().unwrap();
        (center, l.next().unwrap())
    }) {
        if !bodies.contains_key(a) {
            bodies.insert(a, SpaceObject::new(""));
        }

        if !bodies.contains_key(b) {
            bodies.insert(b, SpaceObject::new(""));
        }

        bodies.get_mut(a).unwrap().orbited_by.insert(b);
        bodies.get_mut(b).unwrap().orbits = a;
    }

    bodies
}

fn count_orbits(root: &str, bodies: &HashMap<&str, SpaceObject>) -> usize {
    let obj = &bodies[root];
    obj.orbited_by
        .iter()
        .map(|&o| count_orbits(o, bodies))
        .sum::<usize>()
        + count_children(root, bodies)
}

fn count_children(root: &str, bodies: &HashMap<&str, SpaceObject>) -> usize {
    bodies[root]
        .orbited_by
        .iter()
        .map(|&o| 1 + count_children(o, bodies))
        .sum::<usize>()
}

fn find_orbital_distance(a: &str, b: &str, bodies: &HashMap<&str, SpaceObject>) -> usize {
    let mut path_to_a = find_path(a, bodies);
    let mut path_to_b = find_path(b, bodies);

    path_to_a.pop();
    path_to_b.pop();

    for i in 0..path_to_a.len() {
        if path_to_a[i] != path_to_b[i] {
            return path_to_a.len() - i + path_to_b.len() - i;
        }
    }
    panic!()
}

fn find_path<'a>(target: &'a str, bodies: &HashMap<&'a str, SpaceObject<'a>>) -> Vec<&'a str> {
    if target == "" {
        return vec![];
    }

    let mut v = find_path(bodies[target].orbits, bodies);
    v.push(target);
    v
}

struct SpaceObject<'a> {
    orbits: &'a str,
    orbited_by: HashSet<&'a str>,
}

impl<'a> SpaceObject<'a> {
    pub fn new(orbits: &'a str) -> Self {
        SpaceObject {
            orbits,
            orbited_by: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let bodies = load(input);
        assert_eq!(count_orbits("K", &bodies), 1);
        assert_eq!(count_orbits("J", &bodies), 3);
        assert_eq!(count_orbits("E", &bodies), 7);
        assert_eq!(count_orbits("COM", &bodies), 42);
    }
    #[test]

    fn example2() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let bodies = load(input);
        assert_eq!(find_orbital_distance("YOU", "SAN", &bodies), 4);
    }
}
