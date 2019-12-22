use common::ModularValue;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

type Deck = VecDeque<I>;

const DECK_SIZE: I = 119315717514047;
const REPETITIONS: I = 101741582076661;

fn main() {
    let mut input = String::new();
    File::open("data/day22-input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let deck = explicit_shuffle(&input, 10007);
    println!(
        "Part 1: {:?}",
        deck.iter().position(|&x| x == 2019).unwrap()
    );

    let (a, b) = build_procedure(&input, DECK_SIZE);
    println!(
        "Part 2: {}",
        apply_procedure_alot(a, b, 2020, DECK_SIZE, REPETITIONS)
    );
}

type I = i128;

fn apply_procedure_alot(
    a: ModularValue<I>,
    b: ModularValue<I>,
    i: I,
    deck_size: I,
    n_reps: I,
) -> ModularValue<I> {
    let i = ModularValue::new(i, deck_size);
    let series = ((b.pow(n_reps) - 1) / (b - 1)).unwrap();

    let a = a * series;
    let b = b.pow(n_reps);

    i * b + a
}

fn build_procedure(input: &str, deck_size: I) -> (ModularValue<I>, ModularValue<I>) {
    let mut a = ModularValue::new(0, deck_size);
    let mut b = ModularValue::new(1, deck_size);
    for line in input.lines().rev() {
        match () {
            _ if line.starts_with("deal with") => {
                let n: I = line.split_whitespace().last().unwrap().parse().unwrap();
                let k = ModularValue::new(n, deck_size).inv().unwrap();
                b = b * k;
                a = a * k;
            }
            _ if line.starts_with("deal into") => {
                a = ModularValue::new(deck_size - 1, deck_size) - a;
                b = ModularValue::new(0, deck_size) - b;
            }
            _ if line.starts_with("cut") => {
                let n: I = line.split_whitespace().last().unwrap().parse().unwrap();
                a = a + deck_size + n;
            }
            _ => panic!("{}", line),
        };
    }
    (a, b)
}

fn cut(mut deck: Deck, n: isize) -> Deck {
    if n >= 0 {
        for _ in 0..n {
            let x = deck.pop_front().unwrap();
            deck.push_back(x);
        }
    } else {
        for _ in 0..-n {
            let x = deck.pop_back().unwrap();
            deck.push_front(x);
        }
    }
    deck
}

fn deal_into_new_stack(deck: Deck) -> Deck {
    deck.into_iter().rev().collect()
}

fn deal_with_increment(mut deck: Deck, n: usize) -> Deck {
    let mut output = VecDeque::from(vec![-1; deck.len()]);
    let mut i = 0;
    while let Some(x) = deck.pop_front() {
        output[i] = x;
        i = (i + n) % output.len();
    }
    output
}

fn explicit_shuffle(input: &str, deck_size: I) -> Deck {
    let mut deck: Deck = (0..deck_size).collect();

    for line in input.lines() {
        match () {
            _ if line.starts_with("deal with increment") => {
                let n = line.split_whitespace().last().unwrap().parse().unwrap();
                deck = deal_with_increment(deck, n);
            }
            _ if line.starts_with("deal into new stack") => {
                deck = deal_into_new_stack(deck);
            }
            _ if line.starts_with("cut") => {
                let n = line.split_whitespace().last().unwrap().parse().unwrap();
                deck = cut(deck, n);
            }
            _ => panic!("{}", line),
        }
    }
    deck
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_into() {
        let deck_size: I = 7;
        let input = "deal into new stack";

        let deck: Vec<_> = explicit_shuffle(input, deck_size).into();

        let (a, b) = build_procedure(input, deck_size);

        let mut idx = vec![];
        for i in 0..deck_size {
            idx.push(apply_procedure_alot(a, b, i, deck_size, 1));
        }

        assert_eq!(idx, deck);
    }

    #[test]
    fn test_deal_with() {
        let deck_size: I = 11;
        let input = "deal with increment 5";

        let deck: Vec<_> = explicit_shuffle(input, deck_size).into();

        let (a, b) = build_procedure(input, deck_size);

        let mut idx = vec![];
        for i in 0..deck_size {
            idx.push(apply_procedure_alot(a, b, i, deck_size, 1));
        }

        assert_eq!(idx, deck);
    }

    //#[test]
    fn test_cut() {
        let deck_size: I = 11;
        let input = "cut -3";

        let deck: Vec<_> = explicit_shuffle(input, deck_size).into();

        let (a, b) = build_procedure(input, deck_size);

        let mut idx = vec![];
        for i in 0..deck_size {
            idx.push(apply_procedure_alot(a, b, i, deck_size, 1));
        }

        assert_eq!(idx, deck);
    }
}
