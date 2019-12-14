use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("data/day14-input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let reactions: HashMap<_, _> = input
        .lines()
        .map(Reaction::new)
        .map(Reaction::as_entry)
        .collect();

    println!(
        "Part 1: {}",
        compute_requirements(1, "FUEL", &reactions, &mut HashMap::new())
    );

    println!(
        "Part 2: {}",
        bisect_fuel_from_ore(1000000000000, &reactions)
    );
}

fn bisect_fuel_from_ore(target_ore: usize, reactions: &HashMap<&str, Reaction>) -> usize {
    let mut lower = 0;
    let mut upper = 9999999999999;

    let mut last_attempt = 0;

    loop {
        let attempt = (upper + lower) / 2;
        if attempt == last_attempt {
            return lower;
        }
        last_attempt = attempt;

        let n_ore = compute_requirements(attempt, "FUEL", &reactions, &mut HashMap::new());

        if n_ore <= target_ore {
            lower = attempt;
        }

        if n_ore > target_ore {
            upper = attempt;
        }
    }
}

fn compute_requirements<'a>(
    n: usize,
    name: &'a str,
    reactions: &'a HashMap<&str, Reaction>,
    storage: &mut HashMap<&'a str, usize>,
) -> usize {
    let r = &reactions[name];

    let mut n_avail = *storage.entry(name).or_insert(0);

    if n <= n_avail {
        storage.insert(name, n_avail - n);
        return 0;
    }

    let n_react = (n - n_avail + r.nout - 1) / r.nout;

    n_avail += n_react * r.nout;
    n_avail -= n;

    storage.insert(name, n_avail);

    let mut total_ore = 0;
    for (&iname, &amount) in &r.inputs {
        if iname == "ORE" {
            total_ore += amount * n_react
        } else {
            total_ore += compute_requirements(amount * n_react, iname, reactions, storage);
        }
    }
    total_ore
}

#[derive(Debug)]
struct Reaction<'a> {
    name: &'a str,
    nout: usize,
    inputs: HashMap<&'a str, usize>,
}

impl<'a> Reaction<'a> {
    fn new(line: &'a str) -> Self {
        let mut parts = line.split(" => ");
        let inputs = parts.next().unwrap().split(", ");
        let mut output = parts.next().unwrap().split_whitespace();

        let nout = output.next().unwrap().parse().unwrap();
        let name = output.next().unwrap();

        let inputs = inputs
            .map(|item| {
                let mut item = item.split_whitespace();
                let n = item.next().unwrap().parse().unwrap();
                let name = item.next().unwrap();
                (name, n)
            })
            .collect();

        Reaction { name, nout, inputs }
    }

    fn as_entry(self) -> (&'a str, Self) {
        (self.name, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example14_1() {
        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

        let reactions: HashMap<_, _> = input
            .lines()
            .map(Reaction::new)
            .map(Reaction::as_entry)
            .collect();
        assert_eq!(
            31,
            compute_requirements(1, "FUEL", &reactions, &mut HashMap::new())
        )
    }
}
