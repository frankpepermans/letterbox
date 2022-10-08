use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{Read, Write};

use crate::Matrix;

use super::coordinates::Coordinates;
use super::movement::Movement;
use super::{node::Node, path_node::PathNode};

const WEIGHT: i32 = 1;

pub trait AStar {
    fn astar(
        &self,
        start: Coordinates,
        goal: Coordinates,
        heuristic: &dyn Fn(Coordinates, Coordinates) -> i32,
    ) -> Option<Vec<Coordinates>>;
}

pub fn manhattan_heuristic(a: Coordinates, b: Coordinates) -> i32 {
    let dx = (b.0 as i32 - a.0 as i32).abs();
    let dy = (b.1 as i32 - a.1 as i32).abs();

    dx + dy
}

impl AStar for Matrix<Node> {
    fn astar(
        &self,
        start: Coordinates,
        goal: Coordinates,
        heuristic: &dyn Fn(Coordinates, Coordinates) -> i32,
    ) -> Option<Vec<Coordinates>> {
        let mut open = BinaryHeap::from([PathNode::initial(start, goal, heuristic)]);
        let mut closed = HashMap::new();
        let mut lookup = HashMap::from([(start, 0)]);
        let mut targets =
            Vec::from_iter(self.entanglements.iter().map(|e| vec![e.0, e.1]).flatten());

        targets.push(goal);

        while let Some(current) = open.pop() {
            let mut current = current;

            if current.index == goal {
                let mut path = vec![current];

                while let Some(parent) = current.parent {
                    current = closed[&parent];

                    path.insert(0, current);
                }

                return Some(path.iter().map(|it| it.index).collect());
            }

            closed.insert(current.index, current);

            let g_score_self = lookup.entry(current.index).or_insert(WEIGHT).clone();

            for n in self.nearest_neighbours(current.index) {
                if let Some(index) = n {
                    if !closed.contains_key(&index) {
                        let visited = lookup.contains_key(&index);
                        let g_score_n = if visited { lookup[&index] } else { WEIGHT };
                        let g_score = g_score_self + g_score_n;

                        if !visited || g_score < g_score_n {
                            // detect closest entanglement index which is not yet visited, or goal
                            // targets
                            let target = targets
                                .clone()
                                .into_iter()
                                .filter(|t| !lookup.contains_key(t))
                                .min_by(|a, b| {
                                    let d_a = heuristic(index, *a);
                                    let d_b = heuristic(index, *b);

                                    if d_a < d_b {
                                        Ordering::Less
                                    } else if d_a > d_b {
                                        Ordering::Greater
                                    } else {
                                        Ordering::Equal
                                    }
                                });
                            let target = match target {
                                Some(value) => value,
                                _ => goal,
                            };
                            let h = heuristic(index, target);
                            let path_node = PathNode {
                                index: index,
                                parent: Some(current.index),
                                f: g_score + h,
                                h: h,
                                g: g_score,
                            };

                            if visited {
                                open.retain(|node| node.index != index);
                            }

                            open.push(path_node);
                            lookup.insert(index, g_score);
                        }
                    }
                }
            }
        }

        None
    }
}

impl From<EncodedMatrix> for Matrix<Node> {
    fn from(encoded: EncodedMatrix) -> Self {
        Matrix {
            vec: encoded
                .cells
                .iter()
                .map(|it| it.to_owned().into())
                .collect(),
            rows: encoded.rows,
            cols: encoded.cols,
            entanglements: Vec::new(),
        }
    }
}

impl Into<EncodedMatrix> for Matrix<Node> {
    fn into(self) -> EncodedMatrix {
        EncodedMatrix {
            cells: self.vec.iter().map(|it| it.to_owned().into()).collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

#[derive(Debug)]
pub struct EncodedMatrix {
    pub cells: Vec<u8>,
    pub rows: usize,
    pub cols: usize,
}

impl EncodedMatrix {
    pub fn to_file(&self, file_name: &str) -> std::io::Result<()> {
        let mut data = vec![self.rows as u8, self.cols as u8];

        data.append(&mut self.cells.clone());

        let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());

        e.write_all(&data).expect("could not write to file");

        let data = e.finish().expect("could not zip bytes");

        let mut pos = 0;
        let mut buffer = File::create(file_name)?;

        while pos < data.len() {
            let bytes_written = buffer.write(&data[pos..])?;
            pos += bytes_written;
        }

        Ok(())
    }

    pub fn from_file(file_name: &str) -> Self {
        let raw = std::fs::read(file_name).expect("could not read from file");

        let mut z = flate2::read::ZlibDecoder::new(&raw[..]);
        let mut v: Vec<u8> = Vec::new();

        z.read_to_end(&mut v).expect("could not read to vector");

        let rows = v.remove(0);
        let cols = v.remove(0);

        Self {
            rows: rows as usize,
            cols: cols as usize,
            cells: v,
        }
    }
}
