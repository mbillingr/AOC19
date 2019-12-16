fn main() {
    let mut seq = get_input(INPUT);
    for _ in 0..100 {
        seq = iterate(seq);
    }
    println!("Part 1: {}", seq[..8].iter().map(|x| x.to_string()).collect::<String>());


    let input = INPUT;

    let seq = get_input(input);
    let offset: usize = input[..7].parse().unwrap();

    let n = seq.len();
    // the optimizations below and in iterate2 work only if we are not interested in the first half
    // of the result.
    assert!(offset > n * 10000 / 2);
    let mut seq: Vec<_> = seq.into_iter().cycle().take(n * 10000).skip(offset).collect();

    for _ in 0..100 {
        seq = iterate2(seq);
    }
    println!("Part 2: {}", seq[..8].iter().map(|x| x.to_string()).collect::<String>());
}

fn iterate2(mut sequence: Vec<i64>) -> Vec<i64> {
    // assume we are only looking the second part of the signal, where the pattern is
    // always 0s followed by 1s...
    let n = sequence.len();
    let mut sum = 0;
    for i in (0..n).rev() {
        sum += sequence[i];
        sequence[i] = sum.abs() % 10;
    }
    sequence
}

fn get_input(input: &str) -> Vec<i64> {
    input.bytes().map(|i| (i - b'0') as i64).collect()
}

fn iterate(sequence: Vec<i64>) -> Vec<i64> {
    let n = sequence.len();
    (1..=n)
        .map(|i| pattern(i).zip(&sequence).map(|(p, x)| (p * *x)).sum::<i64>().abs() % 10)
        .collect()
}

fn pattern(n: usize) -> impl Iterator<Item = i64> {
    let a = std::iter::repeat(0).take(n);
    let b = std::iter::repeat(1).take(n);
    let c = std::iter::repeat(0).take(n);
    let d = std::iter::repeat(-1).take(n);
    a.chain(b).chain(c).chain(d).cycle().skip(1)
}

const  INPUT: &str = "59717513948900379305109702352254961099291386881456676203556183151524797037683068791860532352118123252250974130706958763348105389034831381607519427872819735052750376719383812473081415096360867340158428371353702640632449827967163188043812193288449328058464005995046093112575926165337330100634707115160053682715014464686531460025493602539343245166620098362467196933484413717749680188294435582266877493265037758875197256932099061961217414581388227153472347319505899534413848174322474743198535953826086266146686256066319093589456135923631361106367290236939056758783671975582829257390514211329195992209734175732361974503874578275698611819911236908050184158";
