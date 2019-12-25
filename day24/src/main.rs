use std::collections::HashMap;

fn main() {
    let mut field = &mut Field::new(INPUT);
    let mut back = &mut Field::empty();

    let mut seen = HashMap::new();
    let mut step = 0;
    while !seen.contains_key(field) {
        seen.insert(field.clone(), step);
        simstep(field, back);
        step += 1;
        std::mem::swap(field, back);
    }
    println!("Part 1: {}", field.biodiversity());
}

struct Field2 {
    data: Vec<[char; 25]>,
}

const N: usize = 401;

impl Field2 {
    fn empty() -> Self {
        Field2 {
            data: vec![[' '; 25]; N],
        }
    }

    fn new(input: &str) -> Self {
        let mut data = vec![['.'; 25]; N];
        for (i, ch) in input.chars().filter(|&ch| ch == '.' || ch == '#').enumerate() {
            data[N/2][i] = ch;
        }
        Field2 {
            data,
        }
    }

    fn get(&self, i: isize, j: isize, k: isize) -> Option<char> {
        if i < 0 || j < 0 || i > 4 || j > 4 {
            None
        } else {
            Some(self.data[k as usize][(j + i * 5) as usize])
        }
    }

    fn set(&mut self, i: isize, j: isize, k: isize, ch: char) {
        self.data[k as usize][(j + i * 5) as usize] = ch;
    }

    fn neighbors(&self, i: isize, j: isize, k: isize) -> impl Iterator<Item=(isize, isize, isize)> {
        match (i, j) {
            (1, 1) => vec![(i-1, j, 0), (i+1, j, 0), (i, j-1, 0), (i, j+1, 0)].into_iter(),
            _ => unimplemented!()
        }.map(move |(a, b, c)| (a, b, c + k))
    }
}

fn simstep(field: &Field, back: &mut Field) {
    for i in 0..5 {
        for j in 0..5 {
            let n_neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|&(di, dj)| (i + di, j + dj))
                .filter_map(|(y, x)| field.get(y, x))
                .filter(|&ch| ch == '#')
                .count();
            match (field.get(i, j).unwrap(), n_neighbors) {
                ('#', 1) => back.set(i, j, '#'),
                ('#', _) => back.set(i, j, '.'),
                ('.', 1) | ('.', 2) => back.set(i, j, '#'),
                ('.', _) => back.set(i, j, '.'),
                _ => panic!("invalid configuration")
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Field {
    data: [char; 25],
}

impl Field {
    fn empty() -> Self {
        Field {
            data: [' '; 25],
        }
    }

    fn new(input: &str) -> Self {
        let mut data = [' '; 25];
        for (i, ch) in input.chars().filter(|&ch| ch == '.' || ch == '#').enumerate() {
            data[i] = ch;
        }
        Field {
            data,
        }
    }

    fn get(&self, i: isize, j: isize) -> Option<char> {
        if i < 0 || j < 0 || i > 4 || j > 4 {
            None
        } else {
            Some(self.data[(j + i * 5) as usize])
        }
    }

    fn set(&mut self, i: isize, j: isize, ch: char) {
        self.data[(j + i * 5) as usize] = ch;
    }

    fn biodiversity(&self) -> usize {
        (0..25).map(|i| 2usize.pow(i))
            .zip(self.data.iter())
            .filter(|(_, ch)| **ch == '#')
            .map(|(p, _)| p)
            .sum()
    }
}

const INPUT: &str = "###..
.##..
#....
##..#
.###.";