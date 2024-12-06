use crate::grid::{Coord, Grid};
use crate::solver::Solver;
use std::io::BufRead;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(Grid::from_reader(r)?)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter_with_coords()
            .filter(|(_, &v)| v == b'X')
            .map(|(coords, _)| {
                input
                    .neighbours_coords8(coords)
                    .iter()
                    .map(|c| c.diff(&coords))
                    .filter(|&dir| find_sequence_from_point(input, coords, dir))
                    .count()
            })
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter_with_coords()
            .filter(|(c, &v)| v == b'A' && is_xmas_center(input, c))
            .count()
    }
}

fn find_sequence_from_point(grid: &Grid<u8>, coords: (usize, usize), dir: (isize, isize)) -> bool {
    let m_pos = coords.add_offset(dir);
    let a_pos = m_pos.add_offset(dir);
    let s_pos = a_pos.add_offset(dir);
    grid.get(m_pos) == Some(&b'M')
        && grid.get(a_pos) == Some(&b'A')
        && grid.get(s_pos) == Some(&b'S')
}

fn is_xmas_center(grid: &Grid<u8>, coords: &(usize, usize)) -> bool {
    [(-1, -1), (1, -1)]
        .iter()
        .map(|&(x, y)| {
            let p1 = grid
                .get(coords.add_offset((x, y)))
                .copied()
                .unwrap_or_default();
            let p2 = grid
                .get(coords.add_offset((-x, -y)))
                .copied()
                .unwrap_or_default();
            (p1, p2)
        })
        .all(|(p1, p2)| p1 == b'M' && p2 == b'S' || p1 == b'S' && p2 == b'M')
}
