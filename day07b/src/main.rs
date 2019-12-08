use common::expression::Expression;
use common::intcode2::Computer;
use permute::permute;
use std::collections::HashMap;

fn main() {
    use Expression::*;
    let mut amp_expressions = vec![];
    for phase in 0..=9 {
        let mut c = Computer::new(&INPUT);
        let (output, _) = c
            .map(std::iter::once(Const(phase)).chain(std::iter::repeat(Symbol("x"))))
            .unwrap();
        amp_expressions.push(output);
    }

    println!("{}", find_maximum(&amp_expressions));
    println!("{}", find_maximum2(&amp_expressions));
}

fn find_maximum(amp_exprs: &[Vec<Expression>]) -> i64 {
    permute(vec![0, 1, 2, 3, 4])
        .into_iter()
        .map(|seq| {
            let mut syms = HashMap::new();
            syms.insert("x", 0);
            for s in seq {
                let y = amp_exprs[s][0].eval(&syms);
                syms.insert("x", y);
            }
            syms["x"]
        })
        .max()
        .unwrap()
}

fn find_maximum2(amp_exprs: &[Vec<Expression>]) -> i64 {
    permute(vec![9, 8, 7, 6, 5])
        .into_iter()
        .map(|seq| run_loop2(&seq, amp_exprs))
        .max()
        .unwrap()
}

fn run_loop2(phases: &[usize], amp_exprs: &[Vec<Expression>]) -> i64 {
    let mut syms = HashMap::new();
    syms.insert("x", 0);
    for i in 0..amp_exprs[5].len() {
        for &p in phases {
            let y = amp_exprs[p][i].eval(&syms);
            syms.insert("x", y);
        }
    }
    syms["x"]
}

const INPUT: [i64; 499] = [
    3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 34, 47, 72, 81, 94, 175, 256, 337, 418, 99999, 3, 9,
    102, 3, 9, 9, 1001, 9, 3, 9, 4, 9, 99, 3, 9, 101, 4, 9, 9, 1002, 9, 5, 9, 4, 9, 99, 3, 9, 1001,
    9, 5, 9, 1002, 9, 5, 9, 1001, 9, 2, 9, 1002, 9, 5, 9, 101, 5, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9,
    9, 4, 9, 99, 3, 9, 1001, 9, 4, 9, 102, 4, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101,
    2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9,
    3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9,
    9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3,
    9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9,
    4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
    1002, 9, 2, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9,
    9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
    102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
    9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
    102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4,
    9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9,
    102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9,
    4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
    101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99,
];
