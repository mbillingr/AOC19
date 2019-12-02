pub struct Computer {
    pub sr: Vec<i64>,
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

    pub fn classify_step(&mut self, classification: &mut Vec<CellUse>) -> Option<bool> {
        match self.peek().0 {
            Op::Halt => classification[self.pc].set_op(),
            Op::Add(a, b, c) | Op::Mul(a, b, c) => {
                classification[self.pc].set_op();
                classification[self.pc + 1].set_param();
                classification[self.pc + 2].set_param();
                classification[self.pc + 3].set_param();
                classification[a].set_read();
                classification[b].set_read();
                classification[c].set_write();
            }
        }
        self.step()
    }

    fn fetch(&mut self) -> Op {
        let (op, delta) = self.peek();
        self.pc += delta;
        op
    }

    pub fn peek(&self) -> (Op, usize) {
        let i = self.pc;
        let o = self.sr[i];
        let a = self.sr.get(i + 1).map(|&x| x as usize).unwrap_or(99999);
        let b = self.sr.get(i + 2).map(|&x| x as usize).unwrap_or(99999);
        let c = self.sr.get(i + 3).map(|&x| x as usize).unwrap_or(99999);
        match o {
            1 => (Op::Add(a, b, c), 4),
            2 => (Op::Mul(a, b, c), 4),
            99 => (Op::Halt, 1),
            _ => panic!("Unknown opcode: {}", o),
        }
    }
}

pub enum Op {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CellUse {
    op: bool,
    param: bool,
    write: bool,
    read: bool,
}

impl Default for CellUse {
    fn default() -> Self {
        CellUse {
            op: false,
            param: false,
            write: false,
            read: false,
        }
    }
}

impl CellUse {
    pub fn set_op(&mut self) {
        self.op = true;
    }
    pub fn set_param(&mut self) {
        self.param = true;
    }
    pub fn set_write(&mut self) {
        self.write = true;
    }
    pub fn set_read(&mut self) {
        self.read = true;
    }
}

impl std::fmt::Display for CellUse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", if self.op { "X" } else { "-" })?;
        write!(f, "{}", if self.param { "P" } else { "-" })?;
        write!(f, "{}", if self.read { "R" } else { "-" })?;
        write!(f, "{}", if self.write { "W" } else { "-" })
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
