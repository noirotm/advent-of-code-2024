use crate::solutions::{exec_all_days, exec_day};
use humantime::format_duration;
use std::env;
use std::time::Duration;

mod grid;
mod parsing;
mod solutions;
mod solver;

fn main() {
    if let Some(day) = env::args().nth(1) {
        if let Some(o) = exec_day(day.parse().unwrap_or(1)) {
            println!("{o}");
        } else {
            eprintln!("Day {day} not found");
        }
    } else {
        let outputs = exec_all_days();
        for (i, o) in outputs.iter().enumerate() {
            println!("=== Day {} ===\n{o}", i + 1);
        }

        println!("=== Global stats ===");
        let total_parsed = outputs
            .iter()
            .map(|o| o.timings.parse_duration)
            .sum::<Duration>();
        let total_solved = outputs
            .iter()
            .map(|o| o.timings.part1_duration + o.timings.part2_duration)
            .sum::<Duration>();

        println!("Parsing: {}", format_duration(total_parsed));
        println!("Solving: {}", format_duration(total_solved));
        println!("Total:   {}", format_duration(total_parsed + total_solved));
    }
}
