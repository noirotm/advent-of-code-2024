// DO NOT EDIT THIS FILE
use crate::solver::Solver;

mod day01;
mod day02;
mod day03;


pub fn exec_day(day: u32) {
    match day {
        1 => day01::Problem {}.solve(day),
        2 => day02::Problem {}.solve(day),
        3 => day03::Problem {}.solve(day),

        d => println!("Day {d} hasn't been solved yet :("),
    }
}

pub fn exec_all_days() {
    println!("Day 1:");
    day01::Problem {}.solve(1);
    println!("Day 2:");
    day02::Problem {}.solve(2);
    println!("Day 3:");
    day03::Problem {}.solve(3);
}