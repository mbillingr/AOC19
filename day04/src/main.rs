use common::backtracking::BackTracking;
use std::marker::PhantomData;

fn main() {
    let mut search = NumberSearch::<Part1>::new();
    search.backtrack();
    println!("Number of solutions (Part 1): {}", search.solutions.len());

    let mut search = NumberSearch::<Part2>::new();
    search.backtrack();
    println!("Number of solutions (Part 2): {}", search.solutions.len());
}

trait Part {
    fn accept_solution(c: &[u8], min: u64, max: u64) -> bool;
}

struct Part1;

impl Part for Part1 {
    fn accept_solution(c: &[u8], min: u64, max: u64) -> bool {
        if c.len() != 6 {
            return false;
        }

        if !c.windows(2).any(|win| win[1] == win[0]) {
            return false;
        }

        let nr = NumberSearch::<Self>::candidate_to_number(c);
        nr >= min && nr <= max
    }
}

struct Part2;

impl Part for Part2 {
    fn accept_solution(c: &[u8], min: u64, max: u64) -> bool {
        if !Part1::accept_solution(c, min, max) {
            return false;
        }

        let mut digit_counts = [0; 10];

        for d in c {
            digit_counts[*d as usize] += 1;
        }

        digit_counts.contains(&2)
    }
}

struct NumberSearch<T: Part> {
    min: u64,
    max: u64,

    solutions: Vec<Vec<u8>>,
    _p: PhantomData<T>,
}

impl<T: Part> NumberSearch<T> {
    fn new() -> Self {
        NumberSearch {
            min: 284639,
            max: 748759,
            solutions: vec![],
            _p: PhantomData,
        }
    }

    fn candidate_to_number(digits: &[u8]) -> u64 {
        digits
            .iter()
            .rev()
            .scan(1u64, |ord, &d| {
                let o = *ord;
                *ord *= 10;
                Some(d as u64 * o)
            })
            .sum()
    }
}

impl<T: Part> BackTracking for NumberSearch<T> {
    type PartialCandidate = Vec<u8>;

    fn output(&mut self, c: &Self::PartialCandidate) {
        self.solutions.push(c.clone())
    }

    fn root(&self) -> Self::PartialCandidate {
        vec![]
    }

    fn reject(&self, c: &Self::PartialCandidate) -> bool {
        let n = c.len();

        if n > 6 {
            return true;
        }

        if n >= 2 {
            if c[n - 1] < c[n - 2] {
                return true;
            }
        }

        false
    }

    fn accept(&self, c: &Self::PartialCandidate) -> bool {
        T::accept_solution(c, self.min, self.max)
    }

    fn extend(
        &self,
        c: &Self::PartialCandidate,
    ) -> Box<dyn Iterator<Item = Self::PartialCandidate>> {
        let c = c.clone();
        Box::new((0..=9).map(move |i| {
            let mut c_new = c.clone();
            c_new.push(i);
            c_new
        }))
    }
}
