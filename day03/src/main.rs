use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("data/day03-input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut wires = input
        .split('\n')
        .map(|w| w.split(',').map(|dn| (&dn[0..1], dn[1..].parse().unwrap())));

    let mut w1_map = HashMap::new();

    let mut w1_pos = Cursor::new();
    let mut steps: i64 = 0;
    for (d, n) in wires.next().unwrap() {
        for _ in 0..n {
            w1_pos = w1_pos.step(d);
            steps += 1;
            w1_map.insert(w1_pos, steps);
        }
    }

    let mut crossings_manhdist = BinaryHeap::new();
    let mut crossings_stepdist = BinaryHeap::new();

    let mut w2_pos = Cursor::new();
    let mut steps: i64 = 0;
    for (d, n) in wires.next().unwrap() {
        for _ in 0..n {
            w2_pos = w2_pos.step(d);
            steps += 1;
            if let Some(s) = w1_map.get(&w2_pos) {
                let total_steps = steps + *s;
                crossings_stepdist.push(-total_steps);
                crossings_manhdist.push(-w2_pos.manhattan());
            }
        }
    }

    println!("Part 1 Distance: {:?}", -crossings_manhdist.pop().unwrap());
    println!("Part 2 Steps: {:?}", -crossings_stepdist.pop().unwrap());
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Cursor(i64, i64);

impl Cursor {
    pub fn new() -> Self {
        Cursor(0, 0)
    }

    pub fn step(self, dir: &str) -> Self {
        match dir {
            "R" => Cursor(self.0 + 1, self.1),
            "L" => Cursor(self.0 - 1, self.1),
            "U" => Cursor(self.0, self.1 + 1),
            "D" => Cursor(self.0, self.1 - 1),
            _ => panic!("Invalid direction: {}", dir),
        }
    }

    pub fn manhattan(&self) -> i64 {
        self.0.abs() + self.1.abs()
    }
}
