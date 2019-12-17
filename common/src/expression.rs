use super::intcode2::Computable;
use std::collections::HashMap;
use std::ops;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Expression {
    Invalid,
    Symbol(&'static str),
    Const(i64),
    Add(Vec<Expression>),
    Mul(Vec<Expression>),
}

impl Expression {
    pub fn eval(&self, symbols: &HashMap<&str, i64>) -> i64 {
        match self {
            Expression::Invalid => panic!("Attempt to evaluate invalid expression"),
            Expression::Symbol(s) => *symbols
                .get(s)
                .expect(&format!("Unbound symbol {} in {:?}", s, symbols)),
            Expression::Const(i) => *i,
            Expression::Add(terms) => terms.iter().map(|x| x.eval(symbols)).sum(),
            Expression::Mul(factors) => factors.iter().map(|x| x.eval(symbols)).product(),
        }
    }
}

impl Computable for Expression {
    fn invalid() -> Self {
        Expression::Invalid
    }

    fn as_i64(&self) -> i64 {
        self.eval(&HashMap::new())
    }
}

impl From<i64> for Expression {
    fn from(i: i64) -> Self {
        Expression::Const(i)
    }
}

impl ops::Add for Expression {
    type Output = Expression;
    fn add(self, rhs: Self) -> Self {
        use Expression::*;
        match (self, rhs) {
            (Invalid, _) => Invalid,
            (_, Invalid) => Invalid,
            (Const(0), x) | (x, Const(0)) => x,
            (Const(a), Const(b)) => Const(a + b),
            (Add(mut a), Add(b)) => {
                a.extend(b);
                Add(a)
            }
            (Add(mut a), b) | (b, Add(mut a)) => {
                a.push(b);
                Add(a)
            }
            (a, b) => Add(vec![a, b]),
        }
    }
}

impl ops::Mul for Expression {
    type Output = Expression;
    fn mul(self, rhs: Self) -> Self {
        use Expression::*;
        match (self, rhs) {
            (Invalid, _) => Invalid,
            (_, Invalid) => Invalid,
            (Const(0), _) | (_, Const(0)) => Const(0),
            (Const(1), x) | (x, Const(1)) => x,
            (Const(a), Const(b)) => Const(a * b),
            (Mul(mut a), Mul(b)) => {
                a.extend(b);
                Mul(a)
            }
            (Mul(mut a), b) | (b, Mul(mut a)) => {
                a.push(b);
                Mul(a)
            }
            (a, b) => Mul(vec![a, b]),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Invalid => write!(f, "__invalid__"),
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::Const(i) => write!(f, "{}", i),
            Expression::Add(terms) => write!(
                f,
                "{}",
                terms
                    .iter()
                    .map(Self::to_string)
                    .collect::<Vec<_>>()
                    .join(" + ")
            ),
            Expression::Mul(terms) => write!(
                f,
                "{}",
                terms
                    .iter()
                    .map(|x| match x {
                        Expression::Add(_) => format!("({})", x),
                        _ => x.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" * ")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intcode2::ComputerImpl;

    #[test]
    fn expression_1() {
        let mut c = ComputerImpl::<Expression, ()>::new(&[4, 3, 99, 42]);
        let output = c.map(std::iter::empty()).unwrap();
        assert_eq!(output, vec![Expression::Const(42)])
    }

    #[test]
    fn expression_2() {
        let mut c = ComputerImpl::<Expression, ()>::new(&[3, 5, 4, 5, 99, 42]);
        let output = c.map(std::iter::once(Expression::Symbol("x"))).unwrap();
        assert_eq!(output, vec![Expression::Symbol("x")])
    }

    #[test]
    fn expression_3() {
        let mut c = ComputerImpl::<Expression, ()>::new(&[3, 9, 1, 9, 9, 9, 4, 9, 99, 42]);
        let output = c.map(std::iter::once(Expression::Symbol("x"))).unwrap();
        assert_eq!(
            output,
            vec![Expression::Add(vec![
                Expression::Symbol("x"),
                Expression::Symbol("x")
            ])]
        );
    }
}
