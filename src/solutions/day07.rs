use crate::parsing::BufReadExt;
use crate::solver::Solver;
use sscanf::sscanf;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::iter::zip;
use std::str::FromStr;

pub struct Problem;

pub struct Equation {
    total: u64,
    terms: Vec<u64>,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (total, b) = sscanf!(s, "{}: {}", u64, String).map_err(|s| anyhow::anyhow!("{}", s))?;
        let terms = b
            .split_whitespace()
            .map(u64::from_str)
            .collect::<Result<Vec<u64>, _>>()?;

        Ok(Self { total, terms })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Add => '+',
                Op::Mul => '*',
                Op::Concat => '|',
            }
        )
    }
}

impl Op {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Concat => u64::from_str(&format!("{a}{b}")).unwrap_or_default(),
        }
    }
}

impl Solver for Problem {
    type Input = Vec<Equation>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(r.split_lines())
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        // find the largest input
        let max_ops = input
            .iter()
            .map(|equation| equation.terms.len())
            .max()
            .unwrap();

        // generate all configs
        let op_cmbs = generate_op_combinations(max_ops);

        // test all equations
        input
            .iter()
            .filter(|eq| is_equation_possible(eq, &op_cmbs))
            .map(|eq| eq.total)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        // find the largest input
        let max_ops = input
            .iter()
            .map(|equation| equation.terms.len())
            .max()
            .unwrap();

        println!("1");

        // generate all configs
        let op_cmbs = generate_op_combinations_with_concat(max_ops);

        println!("2");

        // test all equations
        input
            .iter()
            .filter(|eq| is_equation_possible(eq, &op_cmbs))
            .map(|eq| eq.total)
            .sum()
    }
}

fn generate_op_combinations(n: usize) -> Vec<Vec<Op>> {
    // number of combinations
    let n_combs = 2u32.pow(n as u32);
    let mut output = Vec::with_capacity(n_combs as usize);

    // convert bit representation
    for i in 0..n_combs {
        let mut list = Vec::with_capacity(n);
        for j in 0..n {
            let bit = (i >> j) & 1;
            list.push(if bit == 0 { Op::Add } else { Op::Mul });
        }
        output.push(list);
    }

    output
}

fn generate_op_combinations_with_concat(n: usize) -> Vec<Vec<Op>> {
    if n == 0 {
        return vec![vec![]];
    }

    let mut result = Vec::new();
    for mut list in generate_op_combinations_with_concat(n - 1) {
        let mut c = list.clone();
        c.push(Op::Add);
        result.push(c);

        let mut c = list.clone();
        c.push(Op::Mul);
        result.push(c);

        list.push(Op::Concat);
        result.push(list);
    }

    result
}

fn is_equation_possible(eq: &Equation, operations: &[Vec<Op>]) -> bool {
    operations
        .iter()
        .any(|op| compute_equation(&eq.terms, op) == eq.total)
}

fn compute_equation(terms: &[u64], operations: &[Op]) -> u64 {
    zip(terms.iter().skip(1), operations.iter())
        //.inspect(|n| _ = dbg!(n))
        .fold(terms[0], |acc, (&n, op)| op.apply(acc, n))
}
