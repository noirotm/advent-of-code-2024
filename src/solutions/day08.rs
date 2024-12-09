use crate::grid::{Coord, Grid};
use crate::solver::Solver;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use std::fmt::Display;
use std::io::BufRead;

pub struct Problem;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Entry {
    Empty,
    Antenna(u8),
}

impl Default for Entry {
    fn default() -> Self {
        Self::Empty
    }
}

impl TryFrom<u8> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Entry::Empty),
            b => Ok(Entry::Antenna(b)),
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Entry::Empty => '.',
                Entry::Antenna(b) => *b as char,
            }
        )
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Vector {
    pos: (usize, usize),
    dir: (isize, isize),
}

impl Vector {
    pub fn next(&self) -> (usize, usize) {
        self.pos.add_offset(self.dir)
    }

    pub fn move_forward(&mut self) {
        self.pos = self.next();
    }
}

impl Solver for Problem {
    type Input = Grid<Entry>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(Grid::from_reader(r)?)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        // make a multi-map of all coords of various points
        let antennas = create_antennas(input);

        // foreach set of antennas, make pairs, and measure distance, then project offset in the
        // opposite direction.
        // Ignore coordinates outside the grid.
        // make a set of all unique points
        let mut antinodes = FnvHashSet::default();
        for (_, v) in antennas.iter() {
            antinodes.extend(
                v.iter()
                    .cartesian_product(v.iter())
                    .filter(|(a, b)| a != b)
                    .map(|(a, b)| a.add_offset(a.diff(b)))
                    .filter(|c| input.contains_coord(c)),
            );
        }

        antinodes.len()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let antennas = create_antennas(input);

        let mut antinodes = FnvHashSet::default();
        for (_, v) in antennas.iter() {
            antinodes.extend(
                v.iter()
                    .cartesian_product(v.iter())
                    .filter(|(a, b)| a != b)
                    .flat_map(|(a, b)| create_all_antinodes(input, a, a.diff(b))),
            );
        }

        antinodes.len()
    }
}

fn create_antennas(input: &Grid<Entry>) -> FnvHashMap<Entry, FnvHashSet<(usize, usize)>> {
    let mut antennas = FnvHashMap::default();
    for (c, v) in input
        .iter_with_coords()
        .filter(|&(_, v)| v.ne(&Entry::Empty))
    {
        antennas
            .entry(v.clone())
            .and_modify(|e: &mut FnvHashSet<(usize, usize)>| _ = e.insert(c))
            .or_insert(FnvHashSet::from_iter([c]));
    }

    antennas
}

fn create_all_antinodes(
    g: &Grid<Entry>,
    c: &(usize, usize),
    d: (isize, isize),
) -> Vec<(usize, usize)> {
    let mut antinodes = vec![];
    let mut vec = Vector { pos: *c, dir: d };

    // antenna itself
    antinodes.push(vec.pos);

    while g.contains_coord(&vec.next()) {
        vec.move_forward();
        antinodes.push(vec.pos);
    }

    antinodes
}
