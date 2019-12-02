fn main() {
    part1();
    part2();
}

fn part1() {
    let mut c = Computer::new(&INPUT);
    c.sr[1] = 12;
    c.sr[2] = 2;
    while c.step().unwrap() {}
    println!("Part 1: {}", c.sr[0]);
}

fn part2() {
    let target = 19690720;

    let (noun, verb) = (0..100)
        .flat_map(|noun| (0..100).map(move |verb| (noun, verb)))
        .filter(|&(noun, verb)| part2_compute(noun, verb).unwrap_or(-1) == target)
        .next()
        .expect("No solution found");

    println!(
        "Part 2: noun={}, verb={}, result={}",
        noun,
        verb,
        100 * noun + verb
    );
}

fn part2_compute(noun: i64, verb: i64) -> Option<i64> {
    let mut c = Computer::new(&INPUT);
    c.sr[1] = noun;
    c.sr[2] = verb;
    while c.step()? {}
    Some(c.sr[0])
}

enum Op {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}

struct Computer {
    sr: Vec<i64>,
    pc: usize,
}

impl Computer {
    pub fn new(input: &[i64]) -> Self {
        Computer {
            sr: input.to_vec(),
            pc: 0,
        }
    }

    pub fn step(&mut self) -> Option<bool> {
        match self.fetch() {
            Op::Halt => return Some(false),
            Op::Add(a, b, c) => *self.sr.get_mut(c)? = self.sr.get(a)? + self.sr.get(b)?,
            Op::Mul(a, b, c) => *self.sr.get_mut(c)? = self.sr.get(a)? * self.sr.get(b)?,
        }
        return Some(true);
    }

    fn fetch(&mut self) -> Op {
        let i = self.pc;
        let o = self.sr[i];
        let a = self.sr.get(i + 1).map(|&x| x as usize).unwrap_or(99999);
        let b = self.sr.get(i + 2).map(|&x| x as usize).unwrap_or(99999);
        let c = self.sr.get(i + 3).map(|&x| x as usize).unwrap_or(99999);
        self.pc += 4;
        match o {
            1 => Op::Add(a, b, c),
            2 => Op::Mul(a, b, c),
            99 => {
                self.pc = i + 1;
                Op::Halt
            }
            _ => panic!("Unknown opcode: {}", o),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let mut c = Computer::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert!(c.step().unwrap());
        assert_eq!(c.sr, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert!(c.step().unwrap());
        assert_eq!(c.sr, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert!(!c.step().unwrap());
    }

    #[test]
    fn example1() {
        let mut c = Computer::new(&[1, 0, 0, 0, 99]);
        while c.step().unwrap() {}
        assert_eq!(c.sr, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn example2() {
        let mut c = Computer::new(&[2, 3, 0, 3, 99]);
        while c.step().unwrap() {}
        assert_eq!(c.sr, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn example3() {
        let mut c = Computer::new(&[2, 4, 4, 5, 99, 0]);
        while c.step().unwrap() {}
        assert_eq!(c.sr, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn example4() {
        let mut c = Computer::new(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        while c.step().unwrap() {}
        assert_eq!(c.sr, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}

const INPUT: [i64; 129] = [
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 6, 19, 23, 2, 23, 6, 27, 1, 5,
    27, 31, 1, 10, 31, 35, 2, 6, 35, 39, 1, 39, 13, 43, 1, 43, 9, 47, 2, 47, 10, 51, 1, 5, 51, 55,
    1, 55, 10, 59, 2, 59, 6, 63, 2, 6, 63, 67, 1, 5, 67, 71, 2, 9, 71, 75, 1, 75, 6, 79, 1, 6, 79,
    83, 2, 83, 9, 87, 2, 87, 13, 91, 1, 10, 91, 95, 1, 95, 13, 99, 2, 13, 99, 103, 1, 103, 10, 107,
    2, 107, 10, 111, 1, 111, 9, 115, 1, 115, 2, 119, 1, 9, 119, 0, 99, 2, 0, 14, 0,
];
