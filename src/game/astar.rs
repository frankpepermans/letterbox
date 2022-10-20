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
    ) -> Option<Vec<Coordinates>> {
        let mut open = BinaryHeap::from([PathNode::initial(start, goal, heuristic)]);
        let mut closed = HashMap::new();
        let mut lookup = HashMap::from([(start, 0)]);
        let targets = Vec::from_iter(self.entanglements.iter().map(|e| vec![e.0, e.1]).flatten());

        while let Some(current) = open.pop() {
            if current.index == goal {
                return Some(to_path(current, closed));
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
                            // detect closest entanglement index which is not yet visited, or goal
                            let target = targets
                                .iter()
                                .filter(|t| !lookup.contains_key(*t))
                                .min_by(|a, b| heuristic(&index, *a).cmp(&heuristic(&index, *b)));

                            let h = heuristic(&index, target.unwrap_or(&goal));
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
