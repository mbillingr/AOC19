use crate::intcode2::{Computer, Op, Operand, MEMORY_SIZE};
use std::collections::{HashMap, HashSet};

fn analyze(intcode: &[i64]) {
    let mut alz = Analyzer {
        mem: vec![CellType::Unknown; MEMORY_SIZE],
        compiled: HashMap::new(),
        vm: Computer::new(intcode),
    };

    alz.walk();
    let mut keys: Vec<_> = alz.compiled.keys().collect();
    keys.sort();

    let mut labels = vec![];
    let mut ops = vec![];
    for k in keys {
        labels.push(*k);
        ops.push(alz.compiled[k].clone());
    }

    let ops = transform(ops);

    for (k, op) in labels.iter().zip(&ops) {
        println!("{:5}  {:?}", k, op);
    }

    let labels = find_used_labels(labels, &ops);
    let blocks = cut_blocks(labels, &ops);

    println!("{:?}", blocks);

    //build_block(&ops);
}

fn build_block(ops: &[FixOp]) -> Vec<String> {
    use FixOp::*;
    use Operand::*;
    let mut code = vec![];
    for op in ops {
        let c = match op {
            Inp(c) => format!("{} = inp();", build_operand(c)),
            Equ(a, b, c) => format!(
                "{} = {} == {};",
                build_operand(c),
                build_operand(a),
                build_operand(b)
            ),
            _ => unimplemented!("{:?}", op),
        };
        println!("{}", c);
        code.push(c);
    }
    code
}

fn build_operand(p: &Operand<i64>) -> String {
    match p {
        Operand::Imm(i) => i.to_string(),
        Operand::Pos(p) => format!("pos_{}", p),
        _ => unimplemented!(),
    }
}

fn find_used_labels(labels: Vec<usize>, ops: &[FixOp]) -> Vec<Option<usize>> {
    use Operand::*;
    let mut used = HashSet::new();
    used.insert(0);
    for op in ops {
        match op {
            FixOp::Jmp(label) => {
                used.insert(*label);
            }
            FixOp::Jit(_, Imm(label))
            | FixOp::Jif(_, Imm(label))
            | FixOp::Set(Imm(label), Rel(0)) => {
                used.insert(*label as usize);
            }
            _ => {}
        }
    }

    labels.into_iter().map(|l| used.get(&l).cloned()).collect()
}

fn cut_blocks(labels: Vec<Option<usize>>, ops: &[FixOp]) -> HashMap<usize, &[FixOp]> {
    let mut label = 0;
    let mut start = 0;

    let mut blocks = HashMap::new();

    for (i, l) in labels.iter().enumerate().skip(1) {
        if let Some(l) = l {
            blocks.insert(label, &ops[start..i]);
            start = i;
            label = *l;
        }
    }
    blocks.insert(start, &ops[start..]);

    blocks
}

fn transform(ops: Vec<FixOp>) -> Vec<FixOp> {
    use FixOp::*;
    use Operand::*;
    ops.into_iter()
        .map(|op| match op {
            Add(Imm(a), Imm(b), c) => Set(Imm(a + b), c),
            Mul(Imm(a), Imm(b), c) => Set(Imm(a * b), c),
            Add(a, Imm(0), c) | Add(Imm(0), a, c) => Set(a, c),
            Mul(a, Imm(1), c) | Mul(Imm(1), a, c) => Set(a, c),
            Jit(Imm(1), Imm(p)) => FixOp::Jmp(p as usize),
            Jif(Imm(0), Imm(p)) => FixOp::Jmp(p as usize),
            _ => op,
        })
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CellType {
    Unknown,
    Constant,
    Mutable,
}

struct Analyzer {
    mem: Vec<CellType>,
    compiled: HashMap<usize, FixOp>,
    vm: Computer,
}

impl Analyzer {
    fn walk(&mut self) {
        loop {
            let pc = self.vm.pc;
            let (op, delta) = self.vm.peek().expect("invalid op");
            self.vm.pc += delta;

            if self.compiled.contains_key(&pc) {
                //ops.push(FixOp::Jmp(pc));
                println!("falling into compiled code...");
                return;
            }

            let mut dynamic = false;

            for i in 0..delta {
                if let Some(_) = self.mark_constant(Operand::Pos(pc + i)) {
                    dynamic = true;
                }
            }

            let fop = match op {
                _ if dynamic => match op {
                    Op::Jit(_, _) | Op::Jif(_, _) => panic!("please, no dynamic jumps!"),
                    _ => FixOp::Dynamic(op),
                },
                Op::Jit(Operand::Imm(1), Operand::Rel(0)) => FixOp::Jr0,
                Op::Jif(Operand::Imm(0), Operand::Rel(0)) => FixOp::Jr0,
                //Op::Jit(Operand::Imm(1), Operand::Imm(p)) => FixOp::Jmp(p as usize),
                //Op::Jif(Operand::Imm(0), Operand::Imm(p)) => FixOp::Jmp(p as usize),
                Op::Halt => FixOp::Halt,
                Op::Invalid => FixOp::Invalid,
                Op::Add(a, b, c) => FixOp::Add(a, b, c),
                Op::Mul(a, b, c) => FixOp::Mul(a, b, c),
                Op::Inp(c) => FixOp::Inp(c),
                Op::Out(a) => FixOp::Out(a),
                Op::Equ(a, b, c) => FixOp::Equ(a, b, c),
                Op::Ltn(a, b, c) => FixOp::Ltn(a, b, c),
                Op::Jit(a, b) => FixOp::Jit(a, b),
                Op::Jif(a, b) => FixOp::Jif(a, b),
                Op::Crb(a) => FixOp::Crb(a),
            };

            println!("{:4}  {:?}", pc, fop);

            //ops.push(fop.clone());
            self.compiled.insert(pc, fop.clone());

            match fop {
                FixOp::Halt | FixOp::Jr0 | FixOp::Invalid => return,
                FixOp::Out(_) | FixOp::Dynamic(_) => {}
                FixOp::Set(_, c)
                | FixOp::Add(_, _, c)
                | FixOp::Mul(_, _, c)
                | FixOp::Inp(c)
                | FixOp::Equ(_, _, c)
                | FixOp::Ltn(_, _, c) => {
                    if let Some(p) = self.mark_mutable(c) {
                        self.force_mut(p)
                    }
                }
                FixOp::Jmp(p) => {
                    self.vm.pc = p;
                }
                FixOp::Jit(_, b) | FixOp::Jif(_, b) => {
                    assert!(self.mark_constant(b).is_none());
                    let vm = self.vm.clone();
                    self.vm.pc = self.get(b) as usize;
                    let branch = self.walk();
                    self.vm = vm;
                }
                FixOp::Crb(a) => {
                    self.vm.rel_base += self.get(a) as isize;
                }

                FixOp::Unknown => println!("ignoring {:?}", fop),
                _ => unimplemented!("{:?}", fop),
            }
        }
    }

    fn force_mut(&mut self, pos: usize) {
        unimplemented!()
    }

    fn get(&self, o: Operand<i64>) -> i64 {
        match o {
            Operand::Imm(i) => i,
            Operand::Pos(p) => unimplemented!(),
            Operand::Rel(o) => unimplemented!(),
            Operand::Push | Operand::Pop => unimplemented!(),
        }
    }

    fn mark_constant(&mut self, x: Operand<i64>) -> Option<usize> {
        match x {
            Operand::Imm(_) => {}
            Operand::Pos(p) => {
                if self.mem[p] == CellType::Mutable {
                    eprintln!("Memory location {} cannot be both, mutable and constant", p);
                    return Some(p);
                } else {
                    self.mem[p] = CellType::Constant;
                }
            }
            Operand::Rel(r) => {
                self.mem[(self.vm.rel_base as isize + r) as usize] = CellType::Constant
            }
            Operand::Push | Operand::Pop => unimplemented!(),
        }
        None
    }

    fn mark_mutable(&mut self, x: Operand<i64>) -> Option<usize> {
        match x {
            Operand::Imm(_) => {}
            Operand::Pos(p) => {
                if self.mem[p] == CellType::Constant {
                    self.mem[p] = CellType::Mutable;
                    eprintln!("Memory location {} cannot be both, mutable and constant", p);
                    return Some(p);
                } else {
                    self.mem[p] = CellType::Mutable;
                }
            }
            Operand::Rel(r) => {
                self.mem[(self.vm.rel_base as isize + r) as usize] = CellType::Mutable
            }
            Operand::Push | Operand::Pop => unimplemented!(),
        }
        None
    }
}

#[derive(Debug, Clone)]
enum FixOp {
    Invalid,
    Halt,
    Add(Operand<i64>, Operand<i64>, Operand<i64>),
    Mul(Operand<i64>, Operand<i64>, Operand<i64>),
    Inp(Operand<i64>),
    Out(Operand<i64>),
    Jit(Operand<i64>, Operand<i64>),
    Jif(Operand<i64>, Operand<i64>),
    Ltn(Operand<i64>, Operand<i64>, Operand<i64>),
    Equ(Operand<i64>, Operand<i64>, Operand<i64>),
    Crb(Operand<i64>),

    Set(Operand<i64>, Operand<i64>),
    Jmp(usize),
    Jr0,
    Dynamic(Op<i64>),
    Unknown,
}

#[cfg(test)]
mod tests {
    use crate::intcode_decompile::analyze;

    #[test]
    fn analysis1() {
        analyze(&INPUT15);
    }

    const INPUT15: [i64; 1045] = [
        3, 1033, 1008, 1033, 1, 1032, 1005, 1032, 31, 1008, 1033, 2, 1032, 1005, 1032, 58, 1008,
        1033, 3, 1032, 1005, 1032, 81, 1008, 1033, 4, 1032, 1005, 1032, 104, 99, 102, 1, 1034,
        1039, 1001, 1036, 0, 1041, 1001, 1035, -1, 1040, 1008, 1038, 0, 1043, 102, -1, 1043, 1032,
        1, 1037, 1032, 1042, 1106, 0, 124, 1001, 1034, 0, 1039, 102, 1, 1036, 1041, 1001, 1035, 1,
        1040, 1008, 1038, 0, 1043, 1, 1037, 1038, 1042, 1106, 0, 124, 1001, 1034, -1, 1039, 1008,
        1036, 0, 1041, 1002, 1035, 1, 1040, 102, 1, 1038, 1043, 1002, 1037, 1, 1042, 1106, 0, 124,
        1001, 1034, 1, 1039, 1008, 1036, 0, 1041, 1001, 1035, 0, 1040, 1001, 1038, 0, 1043, 1002,
        1037, 1, 1042, 1006, 1039, 217, 1006, 1040, 217, 1008, 1039, 40, 1032, 1005, 1032, 217,
        1008, 1040, 40, 1032, 1005, 1032, 217, 1008, 1039, 7, 1032, 1006, 1032, 165, 1008, 1040,
        33, 1032, 1006, 1032, 165, 1101, 2, 0, 1044, 1105, 1, 224, 2, 1041, 1043, 1032, 1006, 1032,
        179, 1102, 1, 1, 1044, 1105, 1, 224, 1, 1041, 1043, 1032, 1006, 1032, 217, 1, 1042, 1043,
        1032, 1001, 1032, -1, 1032, 1002, 1032, 39, 1032, 1, 1032, 1039, 1032, 101, -1, 1032, 1032,
        101, 252, 1032, 211, 1007, 0, 60, 1044, 1105, 1, 224, 1101, 0, 0, 1044, 1106, 0, 224, 1006,
        1044, 247, 101, 0, 1039, 1034, 101, 0, 1040, 1035, 1002, 1041, 1, 1036, 1002, 1043, 1,
        1038, 101, 0, 1042, 1037, 4, 1044, 1105, 1, 0, 92, 17, 17, 33, 88, 37, 85, 63, 23, 14, 79,
        46, 37, 69, 8, 6, 63, 55, 61, 21, 86, 19, 37, 78, 49, 15, 54, 28, 54, 94, 91, 14, 11, 40,
        56, 96, 20, 20, 82, 28, 12, 91, 68, 43, 18, 63, 16, 82, 71, 8, 83, 88, 25, 79, 67, 26, 55,
        33, 51, 74, 68, 59, 64, 58, 78, 30, 65, 64, 9, 48, 87, 26, 85, 32, 82, 92, 21, 34, 99, 1,
        20, 66, 34, 85, 65, 58, 87, 12, 21, 13, 51, 90, 54, 19, 12, 85, 3, 88, 47, 31, 93, 95, 49,
        70, 95, 55, 7, 67, 2, 92, 42, 80, 88, 42, 24, 91, 2, 59, 41, 41, 70, 89, 42, 83, 43, 92,
        44, 93, 62, 26, 63, 99, 81, 35, 98, 70, 71, 79, 8, 90, 26, 66, 94, 22, 47, 55, 90, 93, 6,
        87, 92, 88, 40, 73, 40, 97, 14, 73, 90, 31, 92, 16, 35, 93, 36, 27, 69, 57, 97, 80, 34, 58,
        42, 95, 34, 9, 93, 22, 94, 45, 79, 32, 33, 90, 72, 77, 58, 29, 63, 56, 95, 37, 61, 58, 51,
        57, 8, 25, 86, 75, 25, 63, 64, 93, 57, 7, 79, 85, 57, 53, 97, 16, 63, 40, 71, 52, 23, 33,
        75, 13, 56, 65, 90, 26, 12, 66, 93, 26, 36, 64, 30, 10, 75, 18, 77, 76, 86, 33, 98, 4, 23,
        52, 64, 66, 82, 38, 90, 17, 63, 94, 24, 97, 20, 92, 70, 63, 80, 19, 73, 8, 74, 93, 16, 98,
        77, 52, 38, 90, 46, 49, 76, 84, 53, 50, 22, 93, 19, 16, 61, 47, 54, 67, 56, 78, 21, 77, 52,
        88, 4, 64, 91, 90, 10, 97, 10, 51, 89, 15, 57, 97, 22, 79, 59, 92, 17, 84, 71, 30, 96, 58,
        82, 52, 93, 48, 20, 62, 4, 89, 64, 53, 85, 37, 92, 52, 89, 43, 80, 86, 2, 41, 81, 53, 53,
        82, 77, 31, 66, 92, 31, 44, 81, 14, 49, 96, 66, 42, 91, 2, 61, 82, 36, 32, 90, 8, 61, 32,
        67, 52, 25, 81, 15, 63, 27, 59, 61, 1, 15, 88, 87, 62, 10, 85, 47, 75, 24, 46, 63, 24, 77,
        34, 73, 34, 45, 71, 10, 96, 46, 43, 75, 31, 23, 72, 37, 87, 57, 88, 63, 30, 6, 86, 91, 16,
        53, 16, 89, 81, 11, 32, 75, 22, 82, 69, 50, 88, 53, 67, 50, 65, 67, 26, 81, 83, 20, 14, 23,
        89, 98, 57, 64, 3, 79, 7, 69, 89, 57, 1, 61, 65, 14, 52, 76, 66, 83, 3, 57, 90, 82, 53, 13,
        72, 94, 37, 26, 97, 77, 32, 53, 43, 78, 22, 36, 65, 83, 98, 55, 82, 58, 48, 24, 68, 92, 18,
        22, 90, 65, 28, 81, 33, 63, 79, 3, 31, 65, 92, 53, 46, 74, 7, 80, 37, 79, 79, 83, 42, 82,
        84, 33, 21, 79, 79, 21, 81, 55, 4, 95, 10, 53, 84, 14, 25, 86, 65, 24, 74, 53, 26, 61, 47,
        19, 66, 86, 58, 99, 37, 83, 35, 46, 3, 11, 89, 27, 66, 53, 33, 67, 8, 95, 44, 45, 70, 71,
        65, 59, 49, 77, 25, 3, 56, 83, 39, 91, 3, 52, 86, 67, 57, 99, 86, 40, 39, 3, 99, 25, 69,
        94, 93, 62, 36, 37, 91, 17, 26, 80, 98, 77, 15, 5, 90, 25, 40, 69, 11, 85, 66, 56, 40, 83,
        61, 10, 85, 33, 28, 86, 26, 41, 61, 4, 86, 78, 20, 71, 78, 47, 94, 39, 92, 26, 61, 91, 52,
        69, 20, 47, 45, 99, 38, 96, 39, 98, 76, 58, 28, 94, 27, 47, 97, 2, 45, 54, 64, 94, 98, 27,
        69, 54, 23, 72, 89, 96, 22, 58, 21, 16, 79, 28, 45, 55, 78, 75, 15, 92, 67, 10, 81, 80, 64,
        61, 13, 30, 98, 65, 57, 35, 4, 22, 96, 72, 92, 47, 51, 87, 33, 78, 26, 83, 20, 5, 93, 22,
        73, 83, 68, 24, 17, 61, 69, 39, 62, 53, 20, 95, 84, 53, 83, 36, 48, 99, 33, 13, 42, 90, 97,
        87, 9, 55, 64, 34, 94, 7, 78, 62, 42, 43, 83, 54, 82, 57, 24, 36, 98, 95, 54, 63, 75, 52,
        15, 40, 92, 87, 77, 5, 13, 93, 48, 82, 71, 65, 97, 96, 1, 3, 68, 49, 97, 9, 77, 88, 99, 25,
        78, 4, 84, 97, 77, 4, 92, 91, 76, 53, 71, 58, 64, 55, 68, 97, 96, 48, 99, 2, 86, 51, 69,
        15, 72, 42, 72, 44, 86, 55, 73, 0, 0, 21, 21, 1, 10, 1, 0, 0, 0, 0, 0, 0,
    ];

    const INPUT13: [i64; 2720] = [
        1, 380, 379, 385, 1008, 2719, 612378, 381, 1005, 381, 12, 99, 109, 2720, 1102, 1, 0, 383,
        1102, 0, 1, 382, 21002, 382, 1, 1, 21001, 383, 0, 2, 21101, 37, 0, 0, 1106, 0, 578, 4, 382,
        4, 383, 204, 1, 1001, 382, 1, 382, 1007, 382, 40, 381, 1005, 381, 22, 1001, 383, 1, 383,
        1007, 383, 26, 381, 1005, 381, 18, 1006, 385, 69, 99, 104, -1, 104, 0, 4, 386, 3, 384,
        1007, 384, 0, 381, 1005, 381, 94, 107, 0, 384, 381, 1005, 381, 108, 1105, 1, 161, 107, 1,
        392, 381, 1006, 381, 161, 1102, 1, -1, 384, 1105, 1, 119, 1007, 392, 38, 381, 1006, 381,
        161, 1101, 0, 1, 384, 20102, 1, 392, 1, 21102, 24, 1, 2, 21102, 0, 1, 3, 21102, 138, 1, 0,
        1105, 1, 549, 1, 392, 384, 392, 20102, 1, 392, 1, 21101, 0, 24, 2, 21101, 0, 3, 3, 21101,
        0, 161, 0, 1105, 1, 549, 1102, 0, 1, 384, 20001, 388, 390, 1, 21001, 389, 0, 2, 21102, 1,
        180, 0, 1106, 0, 578, 1206, 1, 213, 1208, 1, 2, 381, 1006, 381, 205, 20001, 388, 390, 1,
        21001, 389, 0, 2, 21101, 0, 205, 0, 1105, 1, 393, 1002, 390, -1, 390, 1102, 1, 1, 384,
        20102, 1, 388, 1, 20001, 389, 391, 2, 21102, 1, 228, 0, 1106, 0, 578, 1206, 1, 261, 1208,
        1, 2, 381, 1006, 381, 253, 20102, 1, 388, 1, 20001, 389, 391, 2, 21101, 0, 253, 0, 1105, 1,
        393, 1002, 391, -1, 391, 1101, 0, 1, 384, 1005, 384, 161, 20001, 388, 390, 1, 20001, 389,
        391, 2, 21102, 279, 1, 0, 1105, 1, 578, 1206, 1, 316, 1208, 1, 2, 381, 1006, 381, 304,
        20001, 388, 390, 1, 20001, 389, 391, 2, 21102, 1, 304, 0, 1105, 1, 393, 1002, 390, -1, 390,
        1002, 391, -1, 391, 1102, 1, 1, 384, 1005, 384, 161, 21002, 388, 1, 1, 21002, 389, 1, 2,
        21102, 0, 1, 3, 21101, 338, 0, 0, 1105, 1, 549, 1, 388, 390, 388, 1, 389, 391, 389, 21002,
        388, 1, 1, 20101, 0, 389, 2, 21102, 1, 4, 3, 21101, 0, 365, 0, 1105, 1, 549, 1007, 389, 25,
        381, 1005, 381, 75, 104, -1, 104, 0, 104, 0, 99, 0, 1, 0, 0, 0, 0, 0, 0, 432, 18, 21, 1, 1,
        20, 109, 3, 22101, 0, -2, 1, 21202, -1, 1, 2, 21102, 1, 0, 3, 21101, 414, 0, 0, 1105, 1,
        549, 21201, -2, 0, 1, 22101, 0, -1, 2, 21102, 1, 429, 0, 1106, 0, 601, 1201, 1, 0, 435, 1,
        386, 0, 386, 104, -1, 104, 0, 4, 386, 1001, 387, -1, 387, 1005, 387, 451, 99, 109, -3,
        2105, 1, 0, 109, 8, 22202, -7, -6, -3, 22201, -3, -5, -3, 21202, -4, 64, -2, 2207, -3, -2,
        381, 1005, 381, 492, 21202, -2, -1, -1, 22201, -3, -1, -3, 2207, -3, -2, 381, 1006, 381,
        481, 21202, -4, 8, -2, 2207, -3, -2, 381, 1005, 381, 518, 21202, -2, -1, -1, 22201, -3, -1,
        -3, 2207, -3, -2, 381, 1006, 381, 507, 2207, -3, -4, 381, 1005, 381, 540, 21202, -4, -1,
        -1, 22201, -3, -1, -3, 2207, -3, -4, 381, 1006, 381, 529, 22101, 0, -3, -7, 109, -8, 2106,
        0, 0, 109, 4, 1202, -2, 40, 566, 201, -3, 566, 566, 101, 639, 566, 566, 2101, 0, -1, 0,
        204, -3, 204, -2, 204, -1, 109, -4, 2105, 1, 0, 109, 3, 1202, -1, 40, 593, 201, -2, 593,
        593, 101, 639, 593, 593, 21002, 0, 1, -2, 109, -3, 2105, 1, 0, 109, 3, 22102, 26, -2, 1,
        22201, 1, -1, 1, 21102, 523, 1, 2, 21102, 588, 1, 3, 21101, 1040, 0, 4, 21102, 630, 1, 0,
        1106, 0, 456, 21201, 1, 1679, -2, 109, -3, 2105, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 2, 2, 2, 2, 2, 0, 0, 2, 0, 0, 2, 2, 2, 0, 0, 2, 2, 2, 0, 2,
        2, 0, 2, 2, 2, 2, 0, 2, 2, 0, 2, 2, 0, 0, 2, 2, 0, 1, 1, 0, 2, 2, 2, 2, 0, 2, 0, 2, 0, 0,
        2, 0, 0, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 0, 0, 2, 0, 2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 0,
        0, 0, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 0, 2, 0, 2, 2, 2, 2, 2, 0,
        0, 2, 2, 0, 2, 2, 0, 1, 1, 0, 0, 2, 2, 2, 0, 0, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2,
        0, 2, 2, 0, 2, 2, 2, 0, 2, 0, 2, 2, 0, 0, 2, 2, 0, 1, 1, 0, 2, 2, 0, 2, 0, 2, 2, 2, 0, 2,
        2, 0, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 0, 2, 0, 2, 2, 2, 2, 2, 2, 0, 2, 0, 2, 2, 0, 1, 1, 0,
        2, 2, 2, 2, 0, 0, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        0, 2, 0, 0, 2, 0, 0, 1, 1, 0, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2, 0, 0, 2, 2, 2,
        0, 0, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 0, 2, 0, 2, 0, 2, 2, 0, 0, 2, 2,
        0, 2, 2, 2, 0, 0, 2, 2, 0, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 0, 2, 0, 2, 0, 2, 2, 0, 1, 1, 0,
        2, 0, 2, 0, 2, 2, 2, 0, 2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 2, 0, 0,
        2, 2, 2, 0, 0, 2, 0, 1, 1, 0, 0, 2, 2, 0, 2, 0, 0, 2, 0, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 0, 2, 2, 0, 0, 2, 2, 0, 2, 0, 0, 0, 2, 2, 0, 0, 1, 1, 0, 2, 2, 0, 2, 2, 0, 2, 2, 2, 2,
        2, 2, 0, 2, 0, 0, 2, 0, 0, 2, 2, 0, 0, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 0, 1, 1, 0,
        2, 2, 0, 2, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2, 2, 0, 0, 2, 2, 0, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2,
        2, 2, 2, 2, 0, 2, 0, 1, 1, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 0,
        2, 2, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 0, 2, 0, 1, 1, 0, 0, 2, 0, 2, 0, 2, 2, 2, 0, 2,
        2, 2, 2, 2, 0, 2, 0, 2, 2, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 2, 0, 2, 0, 2, 0, 0, 0, 1, 1, 0,
        2, 2, 0, 2, 0, 2, 2, 2, 2, 0, 2, 0, 2, 2, 0, 0, 2, 2, 0, 0, 2, 2, 2, 0, 2, 0, 2, 2, 0, 2,
        2, 2, 2, 0, 2, 0, 0, 1, 1, 0, 0, 2, 2, 0, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 2,
        2, 2, 0, 2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 1, 1, 0, 2, 2, 0, 0, 2, 0, 2, 2, 2, 0,
        2, 2, 0, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 1, 1, 0,
        2, 2, 2, 2, 0, 0, 0, 2, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 2, 0, 2, 2, 2, 2, 0, 2, 2, 0, 0, 2,
        0, 2, 0, 2, 2, 2, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 53, 34, 9, 31, 78, 23, 10, 70, 2, 23, 4, 91, 45, 37, 6, 65, 96, 79,
        60, 70, 83, 95, 31, 20, 21, 44, 67, 15, 76, 63, 62, 36, 5, 68, 83, 43, 7, 33, 22, 51, 49,
        6, 11, 43, 95, 97, 89, 55, 82, 54, 15, 32, 83, 44, 57, 69, 21, 59, 81, 26, 79, 92, 43, 53,
        34, 31, 10, 78, 8, 64, 16, 62, 44, 26, 81, 75, 4, 62, 77, 15, 38, 57, 3, 52, 75, 79, 66,
        74, 54, 33, 77, 96, 91, 74, 14, 87, 61, 62, 47, 5, 14, 36, 13, 9, 62, 95, 97, 27, 98, 82,
        55, 56, 38, 95, 73, 13, 25, 12, 67, 62, 89, 73, 22, 96, 70, 92, 46, 33, 60, 35, 84, 16, 84,
        7, 86, 93, 89, 91, 59, 18, 71, 26, 84, 75, 91, 71, 59, 62, 20, 89, 77, 13, 58, 39, 71, 49,
        35, 24, 70, 78, 74, 72, 24, 73, 90, 35, 55, 71, 4, 78, 81, 44, 16, 76, 84, 26, 94, 69, 63,
        15, 45, 66, 81, 58, 4, 16, 5, 54, 67, 17, 65, 13, 81, 32, 75, 34, 20, 29, 43, 13, 49, 91,
        67, 25, 44, 45, 69, 89, 9, 91, 61, 71, 57, 77, 4, 67, 80, 85, 95, 65, 95, 93, 32, 71, 1,
        52, 9, 52, 58, 72, 73, 94, 36, 13, 60, 73, 70, 87, 27, 6, 18, 40, 81, 93, 14, 85, 85, 76,
        91, 83, 22, 88, 24, 93, 93, 5, 97, 87, 25, 70, 97, 89, 82, 89, 8, 5, 3, 42, 16, 70, 82, 30,
        82, 49, 69, 4, 42, 92, 72, 21, 58, 12, 83, 42, 9, 19, 33, 75, 12, 88, 64, 79, 37, 75, 33,
        33, 56, 71, 6, 5, 78, 9, 2, 15, 80, 28, 80, 4, 60, 1, 80, 91, 77, 57, 47, 9, 19, 39, 93,
        65, 69, 11, 61, 57, 45, 49, 94, 34, 28, 77, 77, 70, 54, 7, 13, 57, 68, 95, 64, 85, 61, 12,
        50, 75, 76, 33, 8, 14, 71, 72, 61, 47, 21, 12, 83, 33, 71, 97, 27, 3, 5, 96, 52, 88, 12,
        33, 62, 85, 58, 37, 18, 4, 57, 51, 79, 89, 77, 81, 33, 85, 51, 8, 57, 95, 44, 57, 10, 11,
        33, 75, 65, 31, 35, 45, 19, 90, 79, 30, 84, 54, 15, 30, 43, 55, 64, 56, 18, 76, 41, 73, 69,
        25, 81, 7, 68, 66, 86, 46, 56, 84, 7, 58, 77, 73, 18, 12, 53, 82, 86, 53, 45, 31, 77, 16,
        38, 24, 98, 43, 38, 24, 78, 11, 32, 42, 70, 42, 35, 87, 77, 13, 35, 87, 18, 38, 65, 46, 85,
        28, 2, 66, 21, 95, 34, 31, 75, 68, 46, 90, 83, 63, 88, 34, 5, 51, 87, 59, 70, 18, 93, 73,
        24, 45, 31, 72, 71, 84, 22, 82, 4, 90, 97, 17, 51, 95, 68, 4, 32, 70, 63, 86, 10, 65, 60,
        50, 27, 53, 61, 57, 56, 52, 31, 5, 71, 93, 70, 36, 70, 15, 8, 27, 8, 65, 3, 27, 72, 16, 71,
        7, 26, 91, 16, 32, 33, 1, 90, 56, 59, 48, 2, 24, 58, 16, 95, 75, 92, 18, 33, 69, 21, 56,
        22, 52, 54, 48, 9, 53, 71, 17, 57, 81, 61, 37, 14, 61, 41, 43, 74, 84, 78, 63, 51, 79, 40,
        54, 26, 81, 93, 18, 6, 71, 68, 57, 36, 37, 62, 6, 44, 68, 73, 17, 66, 49, 24, 27, 9, 55,
        66, 46, 76, 55, 98, 47, 75, 32, 51, 21, 90, 59, 44, 81, 22, 67, 10, 57, 46, 35, 97, 36, 69,
        38, 5, 63, 22, 80, 91, 30, 88, 18, 91, 32, 63, 26, 1, 80, 57, 45, 60, 18, 7, 54, 86, 45,
        31, 43, 17, 48, 8, 64, 45, 10, 71, 94, 85, 32, 90, 17, 97, 41, 24, 40, 1, 15, 54, 91, 66,
        76, 7, 97, 30, 83, 82, 64, 23, 12, 87, 92, 98, 86, 18, 61, 86, 53, 77, 59, 81, 98, 78, 33,
        31, 94, 23, 88, 39, 33, 23, 86, 76, 91, 32, 70, 32, 69, 30, 64, 52, 32, 1, 37, 82, 82, 79,
        28, 57, 49, 23, 78, 78, 80, 84, 36, 54, 78, 40, 91, 51, 25, 70, 18, 8, 61, 44, 69, 12, 68,
        44, 84, 85, 11, 21, 51, 91, 15, 77, 18, 78, 53, 52, 62, 92, 65, 49, 86, 66, 53, 36, 58, 11,
        63, 98, 85, 47, 47, 71, 22, 91, 18, 40, 82, 2, 16, 74, 24, 98, 98, 89, 32, 23, 53, 19, 53,
        74, 65, 22, 26, 51, 5, 77, 19, 22, 84, 38, 11, 96, 45, 21, 9, 94, 52, 3, 45, 79, 19, 12,
        12, 30, 24, 50, 90, 92, 60, 64, 96, 8, 8, 79, 83, 21, 80, 7, 10, 72, 86, 37, 28, 68, 31,
        39, 63, 90, 36, 1, 92, 96, 62, 87, 38, 62, 33, 40, 93, 92, 9, 29, 42, 34, 97, 58, 14, 75,
        75, 1, 25, 10, 61, 43, 73, 23, 58, 34, 25, 69, 23, 22, 78, 51, 84, 38, 35, 13, 34, 5, 24,
        49, 56, 43, 7, 82, 44, 38, 66, 28, 92, 66, 8, 46, 35, 30, 86, 71, 64, 54, 74, 57, 12, 76,
        79, 75, 24, 83, 11, 74, 21, 11, 9, 57, 25, 93, 98, 94, 39, 67, 54, 68, 67, 63, 89, 18, 46,
        83, 69, 94, 16, 23, 66, 40, 92, 55, 89, 68, 4, 48, 96, 53, 8, 60, 38, 96, 67, 11, 27, 87,
        95, 66, 16, 57, 13, 1, 42, 89, 3, 55, 38, 84, 39, 28, 97, 2, 25, 83, 88, 93, 39, 13, 48,
        30, 76, 43, 36, 64, 64, 11, 70, 76, 3, 13, 90, 63, 73, 6, 27, 76, 52, 76, 75, 65, 79, 26,
        94, 94, 31, 52, 10, 64, 55, 88, 19, 92, 51, 69, 25, 44, 71, 75, 90, 21, 35, 54, 53, 28, 61,
        68, 60, 82, 31, 3, 43, 93, 85, 4, 43, 13, 31, 7, 44, 16, 31, 25, 93, 70, 42, 36, 58, 90,
        63, 94, 30, 91, 2, 17, 16, 612378,
    ];
}
