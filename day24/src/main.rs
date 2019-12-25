use std::collections::HashMap;

fn main() {
    let field = &mut Field::new(INPUT);
    let back = &mut Field::empty();

    let mut seen = HashMap::new();
    let mut step = 0;
    while !seen.contains_key(field) {
        seen.insert(field.clone(), step);
        simstep(field, back);
        step += 1;
        std::mem::swap(field, back);
    }
    println!("Part 1: {}", field.biodiversity());

    let field = &mut Field2::new(INPUT);
    let back = &mut Field2::empty();

    for _ in 0..200 {
        simstep2(field, back);
        std::mem::swap(field, back);
    }

    println!("Part 2: {}", field.count_bugs());
}

fn simstep2(field: &Field2, back: &mut Field2) {
    for k in 0..N as isize {
        for i in 0..5 {
            for j in 0..5 {
                if i == 2 && j == 2 {
                    continue;
                }

                let n_neighbors = field.count_negighbors(i, j, k);

                match (field.get(i, j, k).unwrap(), n_neighbors) {
                    ('#', 1) => back.set(i, j, k, '#'),
                    ('#', _) => back.set(i, j, k, '.'),
                    ('.', 1) | ('.', 2) => {
                        if k == 0 || k == N as isize - 1 {
                            panic!("Field too shallow")
                        }
                        back.set(i, j, k, '#');
                    }
                    ('.', _) => back.set(i, j, k, '.'),
                    _ => panic!("invalid configuration"),
                }
            }
        }
    }
}

struct Field2 {
    data: Vec<[char; 25]>,
}

const N: usize = 405;

impl Field2 {
    fn empty() -> Self {
        Field2 {
            data: vec![['.'; 25]; N],
        }
    }

    fn new(input: &str) -> Self {
        let mut data = vec![['.'; 25]; N];
        for (i, ch) in input
            .chars()
            .filter(|&ch| ch == '.' || ch == '#')
            .enumerate()
        {
            data[N / 2][i] = ch;
        }
        Field2 { data }
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

    fn count_bugs(&self) -> usize {
        self.data
            .iter()
            .map(|f| f.iter().filter(|ch| **ch == '#').count())
            .sum()
    }

    fn count_negighbors(&self, i: isize, j: isize, k: isize) -> usize {
        Self::neighbors(i, j, k)
            .filter_map(|(y, x, z)| self.get(y, x, z))
            .filter(|&ch| ch == '#')
            .count()
    }

    fn neighbors(i: isize, j: isize, k: isize) -> impl Iterator<Item = (isize, isize, isize)> {
        let up_down = match i {
            0 => std::iter::once(Some((1, 2, -1)))
                .chain(std::iter::once(Some((i + 1, j, 0))))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None)),
            1 if j == 2 => std::iter::once(Some((i - 1, j, 0)))
                .chain(std::iter::once(Some((0, 0, 1))))
                .chain(std::iter::once(Some((0, 1, 1))))
                .chain(std::iter::once(Some((0, 2, 1))))
                .chain(std::iter::once(Some((0, 3, 1))))
                .chain(std::iter::once(Some((0, 4, 1)))),
            2 if j == 2 => panic!("invalid field i={}, j=2", i),
            3 if j == 2 => std::iter::once(Some((i + 1, j, 0)))
                .chain(std::iter::once(Some((4, 0, 1))))
                .chain(std::iter::once(Some((4, 1, 1))))
                .chain(std::iter::once(Some((4, 2, 1))))
                .chain(std::iter::once(Some((4, 3, 1))))
                .chain(std::iter::once(Some((4, 4, 1)))),
            1 | 2 | 3 => std::iter::once(Some((i - 1, j, 0)))
                .chain(std::iter::once(Some((i + 1, j, 0))))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None)),
            4 => std::iter::once(Some((i - 1, j, 0)))
                .chain(std::iter::once(Some((3, 2, -1))))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None)),
            _ => panic!("invalid field i={}", i),
        };

        let left_right = match j {
            0 => std::iter::once(Some((2, 1, -1)))
                .chain(std::iter::once(Some((i, j + 1, 0))))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None)),
            1 if i == 2 => std::iter::once(Some((i, j - 1, 0)))
                .chain(std::iter::once(Some((0, 0, 1))))
                .chain(std::iter::once(Some((1, 0, 1))))
                .chain(std::iter::once(Some((2, 0, 1))))
                .chain(std::iter::once(Some((3, 0, 1))))
                .chain(std::iter::once(Some((4, 0, 1)))),
            2 if i == 2 => panic!("invalid field i=2, j=2"),
            3 if i == 2 => std::iter::once(Some((i, j + 1, 0)))
                .chain(std::iter::once(Some((0, 4, 1))))
                .chain(std::iter::once(Some((1, 4, 1))))
                .chain(std::iter::once(Some((2, 4, 1))))
                .chain(std::iter::once(Some((3, 4, 1))))
                .chain(std::iter::once(Some((4, 4, 1)))),
            1 | 2 | 3 => std::iter::once(Some((i, j - 1, 0)))
                .chain(std::iter::once(Some((i, j + 1, 0))))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None)),
            4 => std::iter::once(Some((i, j - 1, 0)))
                .chain(std::iter::once(Some((2, 3, -1))))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None))
                .chain(std::iter::once(None)),
            _ => panic!("invalid field i={}", i),
        };

        up_down
            .chain(left_right)
            .filter_map(|x| x)
            .map(move |(a, b, c)| (a, b, c + k))
            .filter(|&(_, _, z)| z >= 0 && z < N as isize)
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
                _ => panic!("invalid configuration"),
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
        Field { data: ['.'; 25] }
    }

    fn new(input: &str) -> Self {
        let mut data = ['.'; 25];
        for (i, ch) in input
            .chars()
            .filter(|&ch| ch == '.' || ch == '#')
            .enumerate()
        {
            data[i] = ch;
        }
        Field { data }
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
        (0..25)
            .map(|i| 2usize.pow(i))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example24_1() {
        let input = "....#
#..#.
#..##
..#..
#....";

        let field = &mut Field2::new(input);
        let back = &mut Field2::empty();

        for _ in 0..10 {
            simstep2(field, back);
            std::mem::swap(field, back);
        }

        let k = N / 2 - 1;
        for line in field.data[k].chunks(5) {
            for ch in line {
                print!("{}", ch);
            }
            println!()
        }
    }

    #[test]
    fn example24_2() {
        let input = ".....
..#..
.....
.....
.....";

        let field = &mut Field2::new(input);
        let back = &mut Field2::empty();

        println!("{:?}", Field2::neighbors(2, 3, 10).collect::<Vec<_>>());

        println!("{}", field.count_negighbors(0, 2, N as isize / 2));
        println!("{}", field.count_negighbors(0, 2, N as isize / 2 - 1));
        println!("{}", field.count_negighbors(0, 2, N as isize / 2 + 1));

        simstep2(field, back);
        std::mem::swap(field, back);

        for dk in &[-1, 0, 1] {
            let k = N as isize / 2 + dk;
            for line in field.data[k as usize].chunks(5) {
                for ch in line {
                    print!("{}", ch);
                }
                println!()
            }
            println!()
        }
    }
}
