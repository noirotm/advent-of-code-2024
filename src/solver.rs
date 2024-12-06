use humantime::format_duration;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::{Duration, Instant};

fn input_file(day: u32) -> String {
    format!("input/{:02}.txt", day)
}

pub struct Timings {
    pub parse_duration: Duration,
    pub part1_duration: Duration,
    pub part2_duration: Duration,
}

pub struct ProblemOutput {
    pub part1: String,
    pub part2: String,
    pub timings: Timings,
}

impl Display for ProblemOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Parsing: ({})",
            format_duration(self.timings.parse_duration)
        )?;
        writeln!(
            f,
            "Part 1: {:<20} ({})",
            self.part1,
            format_duration(self.timings.part1_duration)
        )?;
        write!(
            f,
            "Part 2: {:<20} ({})",
            self.part2,
            format_duration(self.timings.part2_duration)
        )
    }
}

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input>;
    fn solve_first(&self, input: &Self::Input) -> Self::Output1;
    fn solve_second(&self, input: &Self::Input) -> Self::Output2;

    fn load_input<P: AsRef<Path>>(&self, p: P) -> anyhow::Result<Self::Input> {
        let f = File::open(p)?;
        self.parse_input(BufReader::new(f))
    }

    fn solve(&self, day: u32) -> ProblemOutput {
        let input_file = input_file(day);

        let start = Instant::now();
        let input = self
            .load_input(input_file)
            .expect("unable to open input file");
        let parse_duration = start.elapsed();

        let start = Instant::now();
        let s1 = self.solve_first(&input);
        let part1_duration = start.elapsed();

        let start = Instant::now();
        let s2 = self.solve_second(&input);
        let part2_duration = start.elapsed();

        ProblemOutput {
            part1: s1.to_string(),
            part2: s2.to_string(),
            timings: Timings {
                parse_duration,
                part1_duration,
                part2_duration,
            },
        }
    }
}
