use crate::grid::Grid;
use crate::solver::Solver;
use anyhow::anyhow;
use fnv::FnvHashSet;
use std::collections::VecDeque;
use std::io::BufRead;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Grid::from_reader_callback(r, |e| e.checked_sub(b'0').ok_or(anyhow!("Invalid value")))
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter_with_coords()
            .filter(|(_, &v)| v == 0)
            .map(|(coord, _)| trailhead_score(input, coord))
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter_with_coords()
            .filter(|(_, &v)| v == 0)
            .map(|(coord, _)| trailhead_rating(input, coord))
            .sum()
    }
}

fn trailhead_score(g: &Grid<u8>, coord: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    let mut nines = FnvHashSet::default();

    // we assume the first element is always 0
    queue.push_back(coord);

    while let Some(coord) = queue.pop_front() {
        let value = g.get(coord).unwrap();
        let neighbours = g.neighbours_coords4(coord);

        for n_coord in neighbours {
            if let Some(&v) = g.get(n_coord) {
                if *value == 8 && v == 9 {
                    nines.insert(n_coord);
                } else if v == value + 1 {
                    queue.push_back(n_coord);
                }
            }
        }
    }

    nines.len()
}

fn trailhead_rating(g: &Grid<u8>, coord: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    let mut nines = 0;

    // we assume the first element is always 0
    queue.push_back(coord);

    while let Some(coord) = queue.pop_front() {
        let value = g.get(coord).unwrap();
        let neighbours = g.neighbours_coords4(coord);

        for n_coord in neighbours {
            if let Some(&v) = g.get(n_coord) {
                if *value == 8 && v == 9 {
                    nines += 1;
                } else if v == value + 1 {
                    queue.push_back(n_coord);
                }
            }
        }
    }

    nines
}
