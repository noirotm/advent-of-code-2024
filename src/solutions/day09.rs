use crate::parsing::ReadAll;
use crate::solver::Solver;
use std::cmp::PartialEq;
use std::io::BufRead;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    FreeSpace,
    File(usize),
}

#[derive(Clone, Debug)]
pub struct Entry {
    size: u8,
    node: Node,
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Entry>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(parse_input(&r.read_all()))
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let input_clone = input.clone();
        compute_checksum(Arc::new(input_clone))
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        0
    }
}

fn parse_input(s: &str) -> Vec<Entry> {
    s.char_indices()
        .map(|(i, c)| {
            let size = c.to_digit(10).unwrap_or_default() as u8;
            let node = if i % 2 == 0 {
                Node::File(i / 2)
            } else {
                Node::FreeSpace
            };

            Entry { size, node }
        })
        .collect()
}

fn produce_nodes_from_end(
    entries: Arc<Vec<Entry>>,
    queue: crossbeam_channel::Sender<usize>,
    n: usize,
) {
    // reverse iteration: we push every non-empty entry N times
    let mut sent = 0;
    for entry in entries.iter().rev() {
        for _ in 0..entry.size {
            if let Node::File(node_id) = entry.node {
                if queue.send(node_id).is_err() {
                    return;
                }

                sent += 1;
                if sent == n {
                    return;
                }
            }
        }
    }
}

fn compute_checksum(entries: Arc<Vec<Entry>>) -> u64 {
    let n_filled = entries
        .iter()
        .filter(|e| e.node != Node::FreeSpace)
        .map(|e| e.size as usize)
        .sum::<usize>();
    let n_free = entries
        .iter()
        .filter(|e| e.node == Node::FreeSpace)
        .map(|e| e.size as usize)
        .sum();

    let (tx, rx) = crossbeam_channel::bounded(5);
    let entries_clone = Arc::clone(&entries);
    let handle = std::thread::spawn(move || produce_nodes_from_end(entries_clone, tx, n_free));

    let mut checksum = 0;
    let mut pos = 0;
    'entries_loop: for entry in entries.iter() {
        for _ in 0..entry.size {
            match entry.node {
                Node::FreeSpace => {
                    if let Ok(node_id) = rx.recv().inspect_err(|e| println!("{:?}", e)) {
                        checksum += (pos * node_id) as u64;
                    }
                }
                Node::File(node_id) => {
                    checksum += (pos * node_id) as u64;
                }
            }
            pos += 1;
            if pos == n_filled {
                drop(rx);
                break 'entries_loop;
            }
        }
    }

    handle.join().unwrap();

    checksum
}
