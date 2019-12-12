pub mod backtracking;
pub mod expression;
pub mod intcode;
pub mod intcode2;

pub fn gcd(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}
