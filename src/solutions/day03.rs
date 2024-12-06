use crate::parsing::ReadAll;
use crate::solver::Solver;
use regex::RegexBuilder;
use std::io::BufRead;

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(r.read_all())
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        mul_total(input)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        mul_total(&filter_string(input))
    }
}

fn mul_total(s: &str) -> u64 {
    let re = RegexBuilder::new("mul\\((\\d{1,3}),(\\d{1,3})\\)")
        .multi_line(true)
        .build()
        .unwrap();

    re.captures_iter(s)
        .map(|c| (c[1].parse::<u64>().unwrap(), c[2].parse::<u64>().unwrap()))
        .map(|(x, y)| x * y)
        .sum()
}

fn filter_string(s: &str) -> String {
    let mut enabled = true;
    let mut cursor = s;
    let mut result = String::new();

    while !cursor.is_empty() {
        if enabled {
            if cursor.get(0..7).unwrap_or_default() == "don't()" {
                enabled = false;
            } else {
                result.push_str(&cursor[0..1]);
            }
        } else if cursor.get(0..4).unwrap_or_default() == "do()" {
            enabled = true;
        }

        cursor = &cursor[1..];
    }

    result
}
