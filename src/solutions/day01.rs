use crate::parsing::BufReadExt;
use crate::solver::Solver;
use itertools::Itertools;
use sscanf::sscanf;
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

pub struct Problem;

pub struct Numbers {
    left: u32,
    right: u32,
}

impl FromStr for Numbers {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = sscanf!(s, "{}   {}", u32, u32)?;
        Ok(Numbers { left: l, right: r })
    }
}

impl Solver for Problem {
    type Input = Vec<Numbers>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(r.split_lines())
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let left = input.iter().map(|n| n.left).sorted();
        let right = input.iter().map(|n| n.right).sorted();

        left.zip(right).map(|(l, r)| l.abs_diff(r)).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let occurrences = input
            .iter()
            .map(|n| n.right)
            .sorted()
            .dedup_with_count()
            .map(|(count, n)| (n, count))
            .collect::<HashMap<_, _>>();

        input
            .iter()
            .map(|n| n.left)
            .map(|n| occurrences.get(&n).copied().unwrap_or_default() as u32 * n)
            .sum()
    }
}
