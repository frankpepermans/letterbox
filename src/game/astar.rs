use std::collections::{BinaryHeap, HashMap};

use super::coordinates::Coordinates;
use super::matrix::Matrix;
use super::movement::Movement;
use super::{node::Node, path_node::PathNode};

const WEIGHT: i32 = 1;

pub trait AStar {
    fn astar(
        &self,
        start: Coordinates,
        goal: Coordinates,
        heuristic: &dyn Fn(&Coordinates, &Coordinates) -> i32,
        partial_path: Option<Vec<(usize, usize)>>,
    ) -> Option<Vec<Coordinates>>;
}

pub fn manhattan_heuristic(a: &Coordinates, b: &Coordinates) -> i32 {
    let dx = (b.0 as i32 - a.0 as i32).abs();
    let dy = (b.1 as i32 - a.1 as i32).abs();

    dx + dy
}

impl AStar for Matrix<Node> {
    fn astar(
        &self,
        start: Coordinates,
        goal: Coordinates,
        heuristic: &dyn Fn(&Coordinates, &Coordinates) -> i32,
        partial_path: Option<Vec<(usize, usize)>>,
    ) -> Option<Vec<Coordinates>> {
        let mut open = BinaryHeap::from([PathNode::initial(start, goal, heuristic)]);
        let mut closed = HashMap::new();
        let mut lookup = HashMap::from([(start, 0)]);

        while let Some(current) = open.pop() {
            if current.index == goal {
                return Some(to_path(current, closed));
            } else if let Some(path) = &partial_path {
                if let Some(pos) = path.iter().position(|it| it == &current.index) {
                    return Some([to_path(current, closed), path[pos + 1..].to_vec()].concat());
                }
            }

            closed.insert(current.index, current);

            let g_score_self = *lookup.get(&current.index).unwrap_or(&WEIGHT);

            for n in self.nearest_neighbours(current.index) {
                if let Some(index) = n {
                    if !closed.contains_key(&index) {
                        let visited = lookup.get(&index);
                        let g_score_n = *visited.unwrap_or(&WEIGHT);
                        let g = g_score_self + g_score_n;

                        if visited.is_none() || g < g_score_n {
                            let h = heuristic(&index, &goal);
                            let path_node = PathNode {
                                index,
                                parent: Some(current.index),
                                f: g + h,
                                h,
                                g,
                            };

                            if visited.is_some() {
                                open.retain(|node| node.index != index);
                            }

                            open.push(path_node);
                            lookup.insert(index, g);
                        }
                    }
                }
            }
        }

        None
    }
}

#[inline(always)]
fn to_path(node: PathNode, closed: HashMap<(usize, usize), PathNode>) -> Vec<Coordinates> {
    let mut path = vec![node.index];
    let mut current = node;

    while let Some(parent) = current.parent {
        current = closed[&parent];

        path.insert(0, current.index);
    }

    path
}
