use std::collections::HashMap;

fn main() {
    /*let mut seq = get_input(INPUT);
    for _ in 0..100 {
        seq = iterate(seq);
    }
    println!("Part 1: {}", seq[..8].iter().map(|x| x.to_string()).collect::<String>());*/


    /*let mut seq = get_input(INPUT);
    let n = seq.len();
    let mut seq = seq.into_iter().cycle().take(n * 10000).collect();
    for i in 0..100 {
        seq = iterate(seq);
        println!("{}", i);
    }
    println!("{}", seq[..8].iter().map(|x| x.to_string()).collect::<String>());
    let offset: usize = seq[..7].iter().map(|x| x.to_string()).collect::<String>().parse().unwrap();
    println!("Part 2: {}", seq[offset..offset+8].iter().map(|x| x.to_string()).collect::<String>());*/

    let mut seq = get_input("123");
    let n = seq.len();
    let mut seq: Vec<_> = seq.into_iter().cycle().take(n * 10).collect();
    seq = iterate(seq);
    println!("{:?}", seq);
}

fn iterate2(sequence: Vec<i32>) -> Vec<i32> {
    let n = sequence.len();
    (1..=n)
        .map(|i| pattern(i).zip(&sequence).map(|(p, x)| (p * *x)).sum::<i32>().abs() % 10)
        .collect()
}

fn get_input(input: &str) -> Vec<i32> {
    input.bytes().map(|i| (i - b'0') as i32).collect()
}

fn iterate(sequence: Vec<i32>) -> Vec<i32> {
    let n = sequence.len();
    (1..=n)
        .map(|i| pattern(i).zip(&sequence).map(|(p, x)| (p * *x)).sum::<i32>().abs() % 10)
        .collect()
}

fn pattern(n: usize) -> impl Iterator<Item = i32> {
    let a = std::iter::repeat(0).take(n);
    let b = std::iter::repeat(1).take(n);
    let c = std::iter::repeat(0).take(n);
    let d = std::iter::repeat(-1).take(n);
    a.chain(b).chain(c).chain(d).cycle().skip(1)
}

const  INPUT: &str = "59717513948900379305109702352254961099291386881456676203556183151524797037683068791860532352118123252250974130706958763348105389034831381607519427872819735052750376719383812473081415096360867340158428371353702640632449827967163188043812193288449328058464005995046093112575926165337330100634707115160053682715014464686531460025493602539343245166620098362467196933484413717749680188294435582266877493265037758875197256932099061961217414581388227153472347319505899534413848174322474743198535953826086266146686256066319093589456135923631361106367290236939056758783671975582829257390514211329195992209734175732361974503874578275698611819911236908050184158";
