use std::cell::RefCell;
use std::collections::HashMap;
use std::ops;

const MEMORY_SIZE: usize = 65535;

pub type Computer = ComputerImpl<i64, ()>;

pub trait Computable:
    Clone + From<i64> + ops::Add<Output = Self> + ops::Mul<Output = Self> + PartialEq + PartialOrd
{
    fn invalid() -> Self;
    fn as_i64(&self) -> i64;
}

impl Computable for i64 {
    fn invalid() -> Self {
        999999999
    }

    fn as_i64(&self) -> i64 {
        *self
    }
}

pub trait Hooks: Default {
    fn mem_fetch(&mut self, addr: usize);
    fn mem_read(&mut self, addr: usize);
    fn mem_write(&mut self, addr: usize);
}

impl Hooks for () {
    fn mem_fetch(&mut self, _addr: usize) {}
    fn mem_read(&mut self, _addr: usize) {}
    fn mem_write(&mut self, _addr: usize) {}
}

pub enum MemType {
    Op,
    Read,
    Write,
}

impl Hooks for HashMap<usize, MemType> {
    fn mem_fetch(&mut self, addr: usize) {
        match self.get(&addr) {
            None => {
                self.insert(addr, MemType::Op);
            }
            Some(MemType::Write) => println!("WARNING: executing mutated operation"),
            _ => {}
        }
    }

    fn mem_read(&mut self, addr: usize) {
        match self.get(&addr) {
            None => {
                self.insert(addr, MemType::Read);
            }
            _ => {}
        }
    }

    fn mem_write(&mut self, addr: usize) {
        self.insert(addr, MemType::Write);
    }
}

#[derive(Clone)]
pub struct ComputerImpl<T: Computable, H: Hooks = ()> {
    pub sr: Vec<T>,
    pub pc: usize,
    pub rel_base: isize,
    pub hooks: RefCell<H>,
}

impl<T: Computable, H: Hooks> ComputerImpl<T, H> {
    pub fn new(program: &[i64]) -> Self {
        let mut sr: Vec<_> = program.iter().cloned().map(T::from).collect();
        sr.resize(MEMORY_SIZE, 0.into());
        ComputerImpl {
            sr,
            pc: 0,
            rel_base: 0,
            hooks: RefCell::new(H::default()),
        }
    }

    pub fn map(
        &mut self,
        mut input: impl Iterator<Item = T>,
    ) -> Option<(Vec<T>, impl Iterator<Item = T>)> {
        let mut output = vec![];
        let mut next_input = None;
        loop {
            match self.run(next_input.take())? {
                WhatsUp::Halt => break,
                WhatsUp::NeedInput => next_input = Some(input.next().expect("out of input values")),
                WhatsUp::Output(x) => output.push(x),
            }
        }
        Some((output, input))
    }

    pub fn run(&mut self, mut input: Option<T>) -> Option<WhatsUp<T>> {
        loop {
            let pc = self.pc;
            match self.fetch()? {
                Op::Halt => return Some(WhatsUp::Halt),
                Op::Add(a, b, c) => self.set(c, self.get(a)? + self.get(b)?)?,
                Op::Mul(a, b, c) => self.set(c, self.get(a)? * self.get(b)?)?,
                Op::Inp(a) => match input.take() {
                    Some(x) => self.set(a, x)?,
                    None => {
                        self.pc = pc;
                        return Some(WhatsUp::NeedInput);
                    }
                },
                Op::Out(a) => return Some(WhatsUp::Output(self.get(a)?)),
                Op::Jit(a, b) => {
                    if self.get(a)?.as_i64() != 0 {
                        self.pc = self.get(b)?.as_i64() as usize;
                    }
                }
                Op::Jif(a, b) => {
                    if self.get(a)?.as_i64() == 0 {
                        self.pc = self.get(b)?.as_i64() as usize;
                    }
                }
                Op::Equ(a, b, c) => self.set(
                    c,
                    if self.get(a)? == self.get(b)? {
                        1.into()
                    } else {
                        0.into()
                    },
                )?,
                Op::Ltn(a, b, c) => self.set(
                    c,
                    if self.get(a)? < self.get(b)? {
                        1.into()
                    } else {
                        0.into()
                    },
                )?,
                Op::Crb(a) => {
                    self.rel_base += self.get(a)?.as_i64() as isize;
                }
            }
        }
    }

    pub fn fetch(&mut self) -> Option<Op<T>> {
        self.hooks.borrow_mut().mem_fetch(self.pc);
        let (op, delta) = self.peek()?;
        self.pc += delta;
        Some(op)
    }

    fn mem_read(&self, index: usize) -> Option<T> {
        self.hooks.borrow_mut().mem_read(index);
        self.sr.get(index).cloned()
    }

    fn mem_write(&mut self, index: usize, value: T) -> Option<()> {
        self.hooks.borrow_mut().mem_write(index);
        *self.sr.get_mut(index)? = value;
        Some(())
    }

    pub fn peek(&self) -> Option<(Op<T>, usize)> {
        self.peek_at(self.pc)
    }

    pub fn peek_at(&self, i: usize) -> Option<(Op<T>, usize)> {
        let op = self.sr[i].as_i64();
        let o = op % 100;
        let fa = (op / 100) % 10;
        let fb = (op / 1000) % 10;
        let fc = (op / 10000) % 10;
        let a = self.sr.get(i + 1).cloned().unwrap_or(T::invalid());
        let b = self.sr.get(i + 2).cloned().unwrap_or(T::invalid());
        let c = self.sr.get(i + 3).cloned().unwrap_or(T::invalid());
        let a = || Operand::new(fa, a);
        let b = || Operand::new(fb, b);
        let c = || Operand::new(fc, c);
        Some(match o {
            1 => (Op::Add(a()?, b()?, c()?), 4),
            2 => (Op::Mul(a()?, b()?, c()?), 4),
            3 => (Op::Inp(a()?), 2),
            4 => (Op::Out(a()?), 2),
            5 => (Op::Jit(a()?, b()?), 3),
            6 => (Op::Jif(a()?, b()?), 3),
            7 => (Op::Ltn(a()?, b()?, c()?), 4),
            8 => (Op::Equ(a()?, b()?, c()?), 4),
            9 => (Op::Crb(a()?), 2),
            99 => (Op::Halt, 1),
            //_ => panic!("Unknown opcode: {}", o),
            _ => return None,
        })
    }

    fn get(&self, o: Operand<T>) -> Option<T> {
        match o {
            Operand::Imm(i) => Some(i),
            Operand::Pos(p) => self.mem_read(p),
            Operand::Rel(o) => self.mem_read((self.rel_base as isize + o) as usize),
        }
    }

    fn set(&mut self, o: Operand<T>, val: T) -> Option<()> {
        match o {
            Operand::Imm(_) => None,
            Operand::Pos(p) => self.mem_write(p, val),
            Operand::Rel(o) => self.mem_write((self.rel_base as isize + o) as usize, val),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum WhatsUp<T> {
    Halt,
    NeedInput,
    Output(T),
}

#[derive(Debug, Clone)]
pub enum Op<T: Computable> {
    Add(Operand<T>, Operand<T>, Operand<T>),
    Mul(Operand<T>, Operand<T>, Operand<T>),
    Inp(Operand<T>),
    Out(Operand<T>),
    Jit(Operand<T>, Operand<T>),
    Jif(Operand<T>, Operand<T>),
    Ltn(Operand<T>, Operand<T>, Operand<T>),
    Equ(Operand<T>, Operand<T>, Operand<T>),
    Crb(Operand<T>),
    Halt,
}

#[derive(Debug, Copy, Clone)]
pub enum Operand<T: Computable> {
    Pos(usize),
    Imm(T),
    Rel(isize),
}

impl<T: Computable> Operand<T> {
    pub fn new(flag: i64, x: T) -> Option<Self> {
        match flag {
            0 => Some(Operand::Pos(x.as_i64() as usize)),
            1 => Some(Operand::Imm(x)),
            2 => Some(Operand::Rel(x.as_i64() as isize)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looping1() {
        let prog = &[
            00101, 1, 14, 14, //  0 : cnt = cnt + 1
            00004, 14, //  4 : OUT cnt
            00108, 10, 14, 15, //  6 : cond = cnt == 10
            01006, 15, 0,     // 10 : IF !cond JMP 0
            00099, // 13 : HALT
            0,     // 14 : cnt
            0,     // 15 : cond
        ];
        let mut c = Computer::new(prog);
        /*println!("{:?}", c.peek_at(0));
        println!("{:?}", c.peek_at(4));
        println!("{:?}", c.peek_at(6));
        println!("{:?}", c.peek_at(10));
        println!("{:?}", c.peek_at(13));*/
        let (output, _) = c.map(std::iter::empty()).unwrap();
        assert_eq!(output, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn example5_output() {
        run_program(&[4, 3, 99, 42], &[], &[42])
    }

    #[test]
    fn example5_2_1() {
        let prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        run_program(&prog, &[7], &[0]);
        run_program(&prog, &[8], &[1]);
        run_program(&prog, &[9], &[0]);
    }

    #[test]
    fn example5_2_2() {
        let prog = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        run_program(&prog, &[7], &[1]);
        run_program(&prog, &[8], &[0]);
        run_program(&prog, &[9], &[0]);
    }

    #[test]
    fn example5_2_3() {
        let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        run_program(&prog, &[7], &[0]);
        run_program(&prog, &[8], &[1]);
        run_program(&prog, &[9], &[0]);
    }

    #[test]
    fn example5_2_4() {
        let prog = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        run_program(&prog, &[7], &[1]);
        run_program(&prog, &[8], &[0]);
        run_program(&prog, &[9], &[0]);
    }

    #[test]
    fn example5_2_5() {
        let prog = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        run_program(&prog, &[-1], &[1]);
        run_program(&prog, &[0], &[0]);
        run_program(&prog, &[1], &[1]);
        run_program(&prog, &[2], &[1]);
    }

    #[test]
    fn example5_2_6() {
        let prog = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        run_program(&prog, &[-1], &[1]);
        run_program(&prog, &[0], &[0]);
        run_program(&prog, &[1], &[1]);
        run_program(&prog, &[2], &[1]);
    }

    #[test]
    fn example5_2_7() {
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        run_program(&prog, &[7], &[999]);
        run_program(&prog, &[8], &[1000]);
        run_program(&prog, &[9], &[1001]);
    }

    #[test]
    fn example9_1() {
        let prog = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        run_program(&prog, &[], &prog);
    }

    #[test]
    fn example9_2() {
        let prog = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut c = Computer::new(&prog);
        let (output, _) = c.map(std::iter::empty()).unwrap();
        assert_eq!(output[0].to_string().len(), 16)
    }

    #[test]
    fn example9_3() {
        let prog = vec![104, 1125899906842624, 99];
        run_program(&prog, &[], &[1125899906842624]);
    }

    fn run_program(prog: &[i64], input: &[i64], expected_output: &[i64]) {
        let mut c = Computer::new(prog);
        let (output, _) = c.map(input.iter().cloned()).unwrap();
        assert_eq!(output, expected_output);
    }
}
