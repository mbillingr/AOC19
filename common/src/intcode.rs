pub type Computer = IoComputer<NoStream, NoStream>;

pub struct IoComputer<I: Input, O: Output> {
    pub sr: Vec<i64>,
    pc: usize,
    pub input: I,
    pub output: O,
}

pub trait Input {
    fn init() -> Self;
    fn read(&mut self) -> i64;
}

pub trait Output {
    fn init() -> Self;
    fn write(&mut self, val: i64);
}

pub struct NoStream;

impl Input for NoStream {
    fn init() -> Self {
        NoStream
    }
    fn read(&mut self) -> i64 {
        panic!("read from non-input")
    }
}

impl Output for NoStream {
    fn init() -> NoStream {
        NoStream
    }
    fn write(&mut self, _: i64) {
        panic!("write to non-output")
    }
}

impl<T: Iterator<Item = i64>> Input for T {
    fn init() -> T {
        unimplemented!()
    }
    fn read(&mut self) -> i64 {
        self.next().expect("Input underflow")
    }
}

impl Output for Vec<i64> {
    fn init() -> Self {
        vec![]
    }
    fn write(&mut self, val: i64) {
        self.push(val)
    }
}

impl<I: Input, O: Output> IoComputer<I, O> {
    pub fn new(input: &[i64]) -> Self {
        IoComputer {
            sr: input.to_vec(),
            pc: 0,
            input: Input::init(),
            output: Output::init(),
        }
    }

    pub fn with_io(program: &[i64], input: I, output: O) -> Self {
        IoComputer {
            sr: program.to_vec(),
            pc: 0,
            input,
            output,
        }
    }

    pub fn step(&mut self) -> Option<bool> {
        match self.fetch()? {
            Op::Halt => return Some(false),
            Op::Add(a, b, c) => self.set(c, self.get(a)? + self.get(b)?)?,
            Op::Mul(a, b, c) => self.set(c, self.get(a)? * self.get(b)?)?,
            Op::Inp(a) => {
                let x = self.input.read();
                self.set(a, x)?;
            }
            Op::Out(a) => self.output.write(self.get(a)?),
            Op::Jit(a, b) => {
                if self.get(a)? != 0 {
                    self.pc = self.get(b)? as usize;
                }
            }
            Op::Jif(a, b) => {
                if self.get(a)? == 0 {
                    self.pc = self.get(b)? as usize;
                }
            }
            Op::Equ(a, b, c) => self.set(c, if self.get(a)? == self.get(b)? { 1 } else { 0 })?,
            Op::Ltn(a, b, c) => self.set(c, if self.get(a)? < self.get(b)? { 1 } else { 0 })?,
        }
        Some(true)
    }

    pub fn classify_step(&mut self, classification: &mut Vec<CellUse>) -> Option<bool> {
        match self.peek()?.0 {
            Op::Halt => classification[self.pc].set_op(),
            Op::Add(a, b, c) | Op::Mul(a, b, c) | Op::Ltn(a, b, c) | Op::Equ(a, b, c) => {
                classification[self.pc].set_op();
                self.classify_operand('R', self.pc + 1, a, classification);
                self.classify_operand('R', self.pc + 2, b, classification);
                self.classify_operand('W', self.pc + 3, c, classification);
            }
            Op::Jif(a, b) | Op::Jit(a, b) => {
                classification[self.pc].set_op();
                self.classify_operand('R', self.pc + 1, a, classification);
                self.classify_operand('R', self.pc + 2, b, classification);
            }
            Op::Inp(a) => {
                classification[self.pc].set_op();
                self.classify_operand('W', self.pc + 1, a, classification);
            }
            Op::Out(a) => {
                classification[self.pc].set_op();
                self.classify_operand('R', self.pc + 1, a, classification);
            }
        }
        self.step()
    }

    fn classify_operand(
        &mut self,
        mode: char,
        idx: usize,
        o: Operand,
        classification: &mut Vec<CellUse>,
    ) {
        match o {
            Operand::Imm(_) => {
                classification[idx].set_immediate();
            }
            Operand::Pos(p) => {
                classification[idx].set_param();
                match mode {
                    'R' => classification[p].set_read(),
                    'W' => classification[p].set_write(),
                    _ => panic!("invalid mode"),
                }
            }
        }
    }

    fn fetch(&mut self) -> Option<Op> {
        let (op, delta) = self.peek()?;
        self.pc += delta;
        Some(op)
    }

    pub fn peek(&self) -> Option<(Op, usize)> {
        let i = self.pc;
        let o = self.sr[i] % 100;
        let fa = (self.sr[i] / 100) % 10;
        let fb = (self.sr[i] / 1000) % 10;
        let fc = (self.sr[i] / 10000) % 10;
        let a = self.sr.get(i + 1).copied().unwrap_or(999999);
        let b = self.sr.get(i + 2).copied().unwrap_or(999999);
        let c = self.sr.get(i + 3).copied().unwrap_or(999999);
        let a = Operand::new(fa, a);
        let b = Operand::new(fb, b);
        let c = Operand::new(fc, c);
        Some(match o {
            1 => (Op::Add(a, b, c), 4),
            2 => (Op::Mul(a, b, c), 4),
            3 => (Op::Inp(a), 2),
            4 => (Op::Out(a), 2),
            5 => (Op::Jit(a, b), 3),
            6 => (Op::Jif(a, b), 3),
            7 => (Op::Ltn(a, b, c), 4),
            8 => (Op::Equ(a, b, c), 4),
            99 => (Op::Halt, 1),
            _ => panic!("Unknown opcode: {}", o),
        })
    }

    fn get(&self, o: Operand) -> Option<i64> {
        match o {
            Operand::Imm(i) => Some(i),
            Operand::Pos(p) => self.sr.get(p).copied(),
        }
    }

    fn set(&mut self, o: Operand, val: i64) -> Option<()> {
        match o {
            Operand::Imm(i) => None,
            Operand::Pos(p) => self.sr.get_mut(p).map(|cell| *cell = val),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Add(Operand, Operand, Operand),
    Mul(Operand, Operand, Operand),
    Inp(Operand),
    Out(Operand),
    Jit(Operand, Operand),
    Jif(Operand, Operand),
    Ltn(Operand, Operand, Operand),
    Equ(Operand, Operand, Operand),
    Halt,
}

#[derive(Debug)]
pub enum Operand {
    Pos(usize),
    Imm(i64),
}

impl Operand {
    pub fn new(flag: i64, x: i64) -> Self {
        match flag {
            0 => Operand::Pos(x as usize),
            1 => Operand::Imm(x),
            _ => panic!("Invalid flag"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CellUse {
    op: bool,
    param: bool,
    write: bool,
    read: bool,
    immediate: bool,
}

impl Default for CellUse {
    fn default() -> Self {
        CellUse {
            op: false,
            param: false,
            write: false,
            read: false,
            immediate: false,
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
    pub fn set_immediate(&mut self) {
        self.immediate = true;
    }
}

impl std::fmt::Display for CellUse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", if self.op { "X" } else { "-" })?;
        write!(
            f,
            "{}",
            match (self.param, self.immediate) {
                (true, true) => panic!("invalid flags"),
                (true, false) => "P",
                (false, true) => "I",
                (false, false) => "-",
            }
        )?;
        write!(
            f,
            "{}",
            if !self.param && self.immediate {
                "I"
            } else {
                "-"
            }
        )?;
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

    #[test]
    fn example5_1_1() {
        let mut c = Computer::new(&[1002, 4, 3, 4, 33]);
        while c.step().unwrap() {}
        assert_eq!(c.sr, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn example5_1_2() {
        let mut c = Computer::new(&[102, 3, 4, 4, 33]);
        while c.step().unwrap() {}
        assert_eq!(c.sr, vec![102, 3, 4, 4, 99]);
    }

    #[test]
    fn example5_input() {
        let mut c: IoComputer<_, _> =
            IoComputer::with_io(&[3, 3, 99, 0], std::iter::once(42), NoStream);
        while c.step().unwrap() {}
        assert_eq!(c.sr, vec![3, 3, 99, 42]);
    }

    #[test]
    fn example5_output() {
        let mut c: IoComputer<_, _> = IoComputer::with_io(&[4, 3, 99, 42], NoStream, vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.sr, vec![4, 3, 99, 42]);
        assert_eq!(c.output, vec![42]);
    }

    #[test]
    fn example5_2_1() {
        let prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut c = IoComputer::with_io(&prog, std::iter::once(7), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![0]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(8), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![1]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(9), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example5_2_2() {
        let prog = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut c = IoComputer::with_io(&prog, std::iter::once(7), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![1]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(8), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![0]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(9), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example5_2_3() {
        let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut c = IoComputer::with_io(&prog, std::iter::once(7), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![0]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(8), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![1]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(9), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example5_2_4() {
        let prog = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut c = IoComputer::with_io(&prog, std::iter::once(7), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![1]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(8), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![0]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(9), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example5_2_5() {
        let prog = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut c = IoComputer::with_io(&prog, std::iter::once(0), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![0]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(1), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![1]);
    }

    #[test]
    fn example5_2_6() {
        let prog = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut c = IoComputer::with_io(&prog, std::iter::once(0), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![0]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(1), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![1]);
    }

    #[test]
    fn example5_2_7() {
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut c = IoComputer::with_io(&prog, std::iter::once(7), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![999]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(8), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![1000]);

        let mut c = IoComputer::with_io(&prog, std::iter::once(9), vec![]);
        while c.step().unwrap() {}
        assert_eq!(c.output, vec![1001]);
    }
}
