use crate::parsing::{BufReadExt, ReadAll};
use crate::solver::Solver;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use sscanf::sscanf;
use std::cmp::Ordering;
use std::convert::Infallible;
use std::io::BufRead;
use std::str::FromStr;

pub struct Protocol {
    rules: FnvHashMap<u8, FnvHashSet<u8>>,
    updates: Vec<Vec<u8>>,
}

impl FromStr for Protocol {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.as_bytes().split_groups::<Vec<String>>();

        let mut rules = FnvHashMap::default();
        for (a, b) in v[0].lines().flat_map(|s| sscanf!(s, "{}|{}", u8, u8)) {
            rules
                .entry(a)
                .and_modify(|v: &mut FnvHashSet<u8>| _ = v.insert(b))
                .or_insert(FnvHashSet::from_iter([b]));
        }

        let updates = v[1].lines().map(|s| s.as_bytes().split_commas()).collect();

        Ok(Self { rules, updates })
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Protocol;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(r.read_all().parse::<Protocol>()?)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .updates
            .iter()
            .filter(|v| {
                v.iter()
                    .tuple_windows()
                    .all(|(a, b)| input.rules.get(a).map_or(false, |v| v.contains(b)))
            })
            .flat_map(|v| find_middle(v))
            .map(u64::from)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .updates
            .iter()
            .filter(|v| {
                v.iter()
                    .tuple_windows()
                    .any(|(a, b)| !input.rules.get(a).map_or(false, |v| v.contains(b)))
            })
            .map(|v| to_correct(v, &input.rules))
            .flat_map(|v| find_middle(&v))
            .map(u64::from)
            .sum()
    }
}

fn find_middle(v: &[u8]) -> Option<u8> {
    v.get(v.len() / 2).copied()
}

fn to_correct(v: &[u8], rules: &FnvHashMap<u8, FnvHashSet<u8>>) -> Vec<u8> {
    // here we really want to sort the input array
    // so we implement a sorter that gives us the natural order
    // if the tuple is present in the rules
    // and reverses the order otherwise
    // this may not be stable, but it's good enough for this problem
    let mut output = v.to_vec();

    output.sort_by(|a, b| {
        rules.get(&a).map_or(Ordering::Greater, |v| {
            if v.contains(&b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    });

    output
}
