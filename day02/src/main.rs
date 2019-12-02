use common::intcode::Computer;

fn main() {
    extra();
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

fn extra() {
    let mut c = Computer::new(&INPUT);
    c.sr[1] = 67;
    c.sr[2] = 18;
    let mut cls = vec![Default::default(); INPUT.len()];
    while c.classify_step(&mut cls).unwrap() {}
    for (i, ((inp, c), mem)) in INPUT.iter().zip(cls).zip(&c.sr).enumerate() {
        println!("{:4} {} {:4} -> {}", i, c, inp, mem);
    }
}

const INPUT: [i64; 129] = [
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 6, 19, 23, 2, 23, 6, 27, 1, 5,
    27, 31, 1, 10, 31, 35, 2, 6, 35, 39, 1, 39, 13, 43, 1, 43, 9, 47, 2, 47, 10, 51, 1, 5, 51, 55,
    1, 55, 10, 59, 2, 59, 6, 63, 2, 6, 63, 67, 1, 5, 67, 71, 2, 9, 71, 75, 1, 75, 6, 79, 1, 6, 79,
    83, 2, 83, 9, 87, 2, 87, 13, 91, 1, 10, 91, 95, 1, 95, 13, 99, 2, 13, 99, 103, 1, 103, 10, 107,
    2, 107, 10, 111, 1, 111, 9, 115, 1, 115, 2, 119, 1, 9, 119, 0, 99, 2, 0, 14, 0,
];
