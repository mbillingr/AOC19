use common::lcm;
use std::collections::HashSet;

fn main() {
    let mut moons: Vec<_> = INPUT.iter().map(|&pos| Moon::new(pos)).collect();

    for _ in 0..1000 {
        update_moons(&mut moons);
    }

    println!("Part 1: {}", compute_energy(&moons));

    let mut moons: Vec<_> = INPUT.iter().map(|&pos| Moon::new(pos)).collect();

    // I'm not sure why this works without resetting `moons` in-between...
    // Probably because has to do with the fact that we are looking for the common multiple anyway?
    let x_cycle = find_independent_cycle(0, &mut moons) as i64;
    let y_cycle = find_independent_cycle(1, &mut moons) as i64;
    let z_cycle = find_independent_cycle(2, &mut moons) as i64;

    println!("Part 2: {}", lcm(x_cycle, lcm(y_cycle, z_cycle)));
}

fn find_independent_cycle(i: usize, moons: &mut [Moon]) -> usize {
    let state0 = (
        moons[0].pos.get(i),
        moons[0].vel.get(i),
        moons[1].pos.get(i),
        moons[1].vel.get(i),
        moons[2].pos.get(i),
        moons[2].vel.get(i),
        moons[3].pos.get(i),
        moons[3].vel.get(i),
    );
    let mut states = HashSet::new();
    states.insert(state0);
    let mut steps: usize = 0;
    loop {
        steps += 1;
        update_moons(moons);
        let state = (
            moons[0].pos.get(i),
            moons[0].vel.get(i),
            moons[1].pos.get(i),
            moons[1].vel.get(i),
            moons[2].pos.get(i),
            moons[2].vel.get(i),
            moons[3].pos.get(i),
            moons[3].vel.get(i),
        );
        if !states.insert(state) {
            if state == state0 {
                break;
            } else {
                panic!("cycle does not include initial position")
            }
        }
    }
    steps
}

fn update_moons(moons: &mut [Moon]) {
    let n = moons.len();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                let pj = moons[j].pos;
                moons[i].update_velocity(pj);
            }
        }
    }

    for m in moons {
        m.update_position();
    }
}

fn compute_energy(moons: &[Moon]) -> i32 {
    let mut e = 0;
    for m in moons {
        let epot = m.pos.x.abs() + m.pos.y.abs() + m.pos.z.abs();
        let ekin = m.vel.x.abs() + m.vel.y.abs() + m.vel.z.abs();
        e += epot * ekin;
    }
    e
}

fn delta_velocity(p1: Vector, p2: Vector) -> Vector {
    Vector {
        x: delta_velocity_scalar(p1.x, p2.x),
        y: delta_velocity_scalar(p1.y, p2.y),
        z: delta_velocity_scalar(p1.z, p2.z),
    }
}

fn delta_velocity_scalar(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Moon {
    pos: Vector,
    vel: Vector,
}

impl Moon {
    fn new(pos: Vector) -> Self {
        Moon {
            pos,
            vel: Vector::new(0, 0, 0),
        }
    }

    fn update_velocity(&mut self, other_pos: Vector) {
        self.vel = self.vel.add(delta_velocity(self.pos, other_pos))
    }

    fn update_position(&mut self) {
        self.pos = self.pos.add(self.vel);
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vector { x, y, z }
    }

    pub fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn get(&self, idx: usize) -> i32 {
        match idx {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!(),
        }
    }
}

const INPUT: [Vector; 4] = [
    Vector { x: 17, y: -9, z: 4 },
    Vector { x: 2, y: 2, z: -13 },
    Vector { x: -1, y: 5, z: -1 },
    Vector { x: 4, y: 7, z: -7 },
];

/*const INPUT: [Vector; 4] = [
    Vector {x:-1, y:0, z:2},
    Vector {x:2, y:-10, z:-7},
    Vector {x:4, y:-8, z:8},
    Vector {x:3, y:5, z:-1}
];*/
