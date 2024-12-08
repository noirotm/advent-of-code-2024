use crate::grid::{Coord, Grid};
use crate::solver::Solver;
use anyhow::anyhow;
use fnv::FnvHashSet;
use rayon::prelude::*;
use std::io::BufRead;

pub struct Problem;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Vector {
    pos: (usize, usize),
    dir: (isize, isize),
}

impl Vector {
    pub fn new(pos: (usize, usize)) -> Self {
        Self { pos, dir: (0, -1) }
    }

    pub fn next(&self) -> (usize, usize) {
        self.pos.add_offset(self.dir)
    }

    pub fn move_forward(&mut self) {
        self.pos = self.next();
    }

    pub fn turn_right(&mut self) {
        self.dir = match self.dir {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => unreachable!(),
        };
    }
}

pub struct Lab {
    grid: Grid<Entry>,
    start: (usize, usize),
}

#[derive(Clone, PartialEq)]
pub enum Entry {
    Empty,
    Wall,
    Guard,
}

impl TryFrom<u8> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Entry::Empty),
            b'#' => Ok(Entry::Wall),
            b'^' => Ok(Entry::Guard),
            _ => Err(anyhow!("Invalid entry")),
        }
    }
}

impl Solver for Problem {
    type Input = Lab;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        let grid = Grid::from_reader(r)?;
        let start = grid
            .iter_with_coords()
            .find_map(|(c, v)| (*v == Entry::Guard).then_some(c))
            .unwrap();

        Ok(Lab { grid, start })
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        find_all_visited(input).len()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut initial_visited = find_all_visited(input);

        // ignore the starting position
        initial_visited.remove(&input.start);

        // for each point initially visited, we try inserting a wall and try to detect a loop
        // aka whether we find a vector that we already visited
        initial_visited
            .par_iter()
            .filter(|p| {
                let mut lab = Lab {
                    grid: input.grid.clone(),
                    start: input.start,
                };
                if let Some(e) = lab.grid.get_mut(*p) {
                    *e = Entry::Wall
                }

                are_we_looping(&lab)
            })
            .count()
    }
}

fn find_all_visited(input: &Lab) -> FnvHashSet<(usize, usize)> {
    let mut visited = FnvHashSet::default();
    let mut vec = Vector::new(input.start);

    while input.grid.contains_coord(&vec.pos) {
        visited.insert(vec.pos);

        while input
            .grid
            .get(vec.next())
            .is_some_and(|e| e == &Entry::Wall)
        {
            vec.turn_right();
        }

        vec.move_forward();
    }

    visited
}

fn are_we_looping(input: &Lab) -> bool {
    let mut visited = FnvHashSet::default();
    let mut vec = Vector::new(input.start);

    while input.grid.contains_coord(&vec.pos) {
        if visited.contains(&vec) {
            return true;
        }

        visited.insert(vec.clone());

        while input
            .grid
            .get(vec.next())
            .is_some_and(|e| e == &Entry::Wall)
        {
            vec.turn_right();
            visited.insert(vec.clone());
        }

        vec.move_forward();
    }

    false
}
