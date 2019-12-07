use common::intcode::IoComputer;
use permute::permute;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (max_sig, _) = find_maximum(&INPUT);
    println!("Part 1: {}", max_sig);

    let (max_sig2, _) = find_maximum2(&INPUT);
    println!("Part 2: {}", max_sig2);
}

fn find_maximum(program: &[i64]) -> (i64, Vec<i64>) {
    permute(vec![0, 1, 2, 3, 4])
        .into_iter()
        .map(|seq| (amplifier_chain(&seq, &program), seq))
        .max()
        .unwrap()
}

fn find_maximum2(program: &[i64]) -> (i64, Vec<i64>) {
    permute(vec![9, 8, 7, 6, 5])
        .into_iter()
        .map(|seq| (run_loop(&seq, &program), seq))
        .inspect(|x| println!("{:?}", x))
        .max()
        .unwrap()
}

fn amplifier_chain(phases: &[i64], program: &[i64]) -> i64 {
    let mut signal = 0;
    for &p in phases {
        signal = amplifier(signal, p, program);
    }
    signal
}

fn amplifier(sig_in: i64, phase: i64, program: &[i64]) -> i64 {
    let mut c = IoComputer::with_io(&program, vec![phase, sig_in].into_iter(), vec![]);
    while c.step().unwrap() {}
    c.output.pop().unwrap()
}

fn run_loop(phases: &[i64], prog: &[i64]) -> i64 {
    let (s1, r1) = mpsc::sync_channel(0);
    let (s2, r2) = mpsc::sync_channel(0);
    let (s3, r3) = mpsc::sync_channel(0);
    let (s4, r4) = mpsc::sync_channel(0);
    let (s5, r5) = mpsc::sync_channel(0);

    let (s0, r0) = mpsc::sync_channel(9999999);

    let mut c1 = IoComputer::with_io(&prog, r1.into_iter(), s2.clone());
    let mut c2 = IoComputer::with_io(&prog, r2.into_iter(), s3.clone());
    let mut c3 = IoComputer::with_io(&prog, r3.into_iter(), s4.clone());
    let mut c4 = IoComputer::with_io(&prog, r4.into_iter(), s5.clone());
    let mut c5 = IoComputer::with_io(&prog, r5.into_iter(), (s1.clone(), s0));

    let t1 = thread::spawn(move || {
        while c1.step().unwrap() {}
        // need to read one more input when thread 1 is done, because otherwise thread 5 fails to write the final output
        c1.input.next().unwrap();
    });
    let t2 = thread::spawn(move || while c2.step().unwrap() {});
    let t3 = thread::spawn(move || while c3.step().unwrap() {});
    let t4 = thread::spawn(move || while c4.step().unwrap() {});
    let t5 = thread::spawn(move || while c5.step().unwrap() {});

    s1.send(phases[0]).unwrap();
    s2.send(phases[1]).unwrap();
    s3.send(phases[2]).unwrap();
    s4.send(phases[3]).unwrap();
    s5.send(phases[4]).unwrap();

    s1.send(0).unwrap();

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
    t5.join().unwrap();

    r0.into_iter().last().unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example7_1() {
        let prog = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let out = find_maximum(&prog);
        assert_eq!(out, (43210, vec![4, 3, 2, 1, 0]));
    }

    #[test]
    fn example7_2() {
        let prog = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let out = find_maximum(&prog);
        assert_eq!(out, (54321, vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn example7_3() {
        let prog = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let out = find_maximum(&prog);
        assert_eq!(out, (65210, vec![1, 0, 4, 3, 2]));
    }

    #[test]
    fn example7_4() {
        let prog = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(run_loop(&vec![9, 8, 7, 6, 5], &prog), 139629729);
    }

    #[test]
    fn example7_5() {
        let prog = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(run_loop(&vec![9, 7, 8, 5, 6], &prog), 18216);
    }
}
