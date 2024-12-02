use crate::parsing::{ReadExt, WhitespaceSeparatedList};
use crate::solver::Solver;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<i32>>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .flat_map(|l| WhitespaceSeparatedList::from_str(&l))
            .map(|l| l.into())
            .collect())
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().filter(|v| is_safe(v)).count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|v| generate_combinations(v))
            .filter(|v| v.iter().any(|d| is_safe(d)))
            .count()
    }
}

fn generate_combinations(input: &[i32]) -> Vec<Vec<i32>> {
    let mut v = (0..input.len())
        .map(|n| {
            input
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != n)
                .map(|(_, n)| *n)
                .collect()
        })
        .collect::<Vec<_>>();
    v.push(input.to_vec());
    v
}

fn is_safe(v: &[i32]) -> bool {
    let diffs = v
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|v| v.0 - v.1)
        .collect_vec();
    let first_sign = diffs.first().copied().unwrap_or_default().signum();
    diffs
        .iter()
        .all(|&n| n.signum() == first_sign && n.abs() >= 1 && n.abs() <= 3)
}
